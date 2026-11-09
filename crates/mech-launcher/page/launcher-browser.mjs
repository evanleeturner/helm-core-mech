#!/usr/bin/env node
// SPDX-License-Identifier: MIT
/**
 * The launcher page's headless browser run. It builds the served folder
 * from the general page kit and this page, serves it through the player's
 * own server, and runs each check it holds, the answers in one JSON
 * file out.
 *
 * Call: `node launcher-browser.mjs <output.json> [check ...]`, every check
 * when none is named, with SERVE (the player's server binary), KIT
 * (oxide95's `web-shell/page` folder), PLAYWRIGHT (Playwright's
 * `index.mjs`), DISC_WASM (the built `page_disc.wasm`) and SCRATCH (a folder for the run's own files) in the
 * environment, and AXE (axe-core's `axe.min.js`) for the `keys` check, which
 * fails without it. Playwright and axe are never dependencies: each is read
 * from the path given, as oxide95's browser runs do. Exits 0 when every check ran, each
 * passing or not; 2 on a wrong call; nonzero when the run itself broke.
 */
import { spawn } from "node:child_process";
import {
  closeSync,
  copyFileSync,
  ftruncateSync,
  mkdirSync,
  mkdtempSync,
  openSync,
  readdirSync,
  readFileSync,
  renameSync,
  rmSync,
  writeFileSync,
  writeSync,
} from "node:fs";
import { createServer } from "node:http";
import { dirname, extname, join } from "node:path";
import { createInterface } from "node:readline";
import { fileURLToPath } from "node:url";

import { DISC_FILES, EMPTY_AT_SOURCE } from "./launcher-disc.mjs";
import {
  HELD,
  HELD_ELSEWHERE,
  IMAGE_NOT_CHECKED,
  KEPT,
  NOT_ISOLATED,
  STARTED,
  heldWrong,
  wrongFiles,
} from "./launcher-words.mjs";

const PAGE_DIR = dirname(fileURLToPath(import.meta.url));
const SERVE_WAIT_MS = 10000;
const TYPES = {
  css: "text/css",
  html: "text/html",
  js: "text/javascript",
  json: "application/json",
  mjs: "text/javascript",
  wasm: "application/wasm",
};

/** True for a file the served folder takes: a page file, no driver, no test. */
function isPageFile(name) {
  return (
    /\.(html|mjs|css|wasm)$/.test(name) &&
    !name.endsWith("-browser.mjs") &&
    !name.endsWith(".test.mjs")
  );
}

/** Builds the served folder: the kit's page files, then this page's over them, and the disc's wasm. */
function assemble(kit, wasm, scratch) {
  const root = mkdtempSync(join(scratch, "launcher-"));
  for (const folder of [kit, PAGE_DIR])
    for (const name of readdirSync(folder).filter(isPageFile))
      copyFileSync(join(folder, name), join(root, name));
  copyFileSync(wasm, join(root, "page_disc.wasm"));
  return root;
}

/** Starts the player's server on the folder; resolves to its address, its one stdout line. */
async function startServe(binary, root) {
  const child = spawn(binary, [root, "0"], {
    stdio: ["ignore", "pipe", "pipe"],
  });
  const log = [];
  child.stderr.on("data", (chunk) => log.push(chunk.toString()));
  const line = await Promise.race([
    new Promise((resolve) =>
      createInterface({ input: child.stdout }).once("line", resolve),
    ),
    new Promise((resolve, reject) =>
      child.once("exit", (code) =>
        reject(new Error(`the server exited ${code}`)),
      ),
    ),
    new Promise((resolve, reject) =>
      setTimeout(
        () => reject(new Error("the server printed no address")),
        SERVE_WAIT_MS,
      ),
    ),
  ]);
  return { child, address: line.trim().replace(/\/$/, ""), log };
}

/** A server with neither header, the `isolation` check's other half; the run's own, never shipped. */
async function startBare(root) {
  const server = createServer((request, response) => {
    const name = new URL(request.url, "http://bare").pathname.slice(1);
    let body = null;
    if (!name.includes("..")) {
      try {
        body = readFileSync(join(root, name));
      } catch {
        body = null;
      }
    }
    if (body === null) {
      response.writeHead(404);
      response.end();
      return;
    }
    const type = TYPES[extname(name).slice(1)] ?? "application/octet-stream";
    response.writeHead(200, { "Content-Type": type });
    response.end(body);
  });
  await new Promise((listening) => server.listen(0, "127.0.0.1", listening));
  return { server, address: `http://127.0.0.1:${server.address().port}` };
}

/** The status line's states that wait on the disc's reader. */
const BUSY = ["loading", "checking", "keeping"];

/** Waits for the status line to leave every busy state. */
async function settle(page) {
  await page.waitForFunction(
    (busy) => !busy.includes(document.getElementById("status").dataset.state),
    BUSY,
  );
}

/**
 * Opens the page, in a browser context of its own and so a page storage of
 * its own, and waits for it to settle past its load's check of a kept copy.
 */
async function open(browser, url) {
  const page = await browser.newPage();
  const errors = [];
  page.on("pageerror", (error) => errors.push(error.message));
  const response = await page.goto(url);
  await settle(page);
  return { page, response, errors };
}

/** The page's isolation, its status line and the headers it was sent. */
async function readIsolation(opened) {
  const headers = opened.response.headers();
  const page = await opened.page.evaluate(() => ({
    isolated: globalThis.crossOriginIsolated,
    state: document.getElementById("status").dataset.state,
    sentence: document.getElementById("status").textContent,
  }));
  await opened.page.close();
  return {
    opener: headers["cross-origin-opener-policy"] ?? null,
    embedder: headers["cross-origin-embedder-policy"] ?? null,
    ...page,
    errors: opened.errors,
  };
}

const BLOCK = 2048;
const IMAGE_SECTOR = 19;
const IMAGE_FILE_BYTES = 300 << 20;
const MARKER_BYTES = [0xa5, 0x5a, 0x3c, 0xc3, 0x7e];
const IMAGE_PLACES = [
  0,
  1 << 20,
  100 << 20,
  200 << 20,
  IMAGE_FILE_BYTES - (1 << 20),
];
/** The measured disc's files at depths 0 to 4, 1,066 in all; the names are made here. */
const FOLDER_DEPTHS = [44, 807, 53, 2, 160];
const FOLDER_BYTES = 225 << 20;

/** A directory record, as `logical_disc` reads one, for the run's own image. */
function record(name, extent, size, folder) {
  const length = 33 + name.length + ((name.length + 1) % 2);
  const bytes = Buffer.alloc(length);
  bytes[0] = length;
  bytes.writeUInt32LE(extent, 2);
  bytes.writeUInt32BE(extent, 6);
  bytes.writeUInt32LE(size, 10);
  bytes.writeUInt32BE(size, 14);
  bytes[25] = folder ? 2 : 0;
  bytes[32] = name.length;
  name.copy(bytes, 33);
  return bytes;
}

/** A sparse iso holding one 300 MiB file, its 5 markers planted: a made disc, no game's bytes. */
function writeImage(path) {
  const file = openSync(path, "w");
  try {
    ftruncateSync(file, IMAGE_SECTOR * BLOCK + IMAGE_FILE_BYTES);
    const put = (sector, at, bytes) =>
      writeSync(file, bytes, 0, bytes.length, sector * BLOCK + at);
    const root = record(Buffer.from([0]), 18, BLOCK, true);
    const sizeLow = Buffer.alloc(2);
    sizeLow.writeUInt16LE(BLOCK);
    const sizeHigh = Buffer.alloc(2);
    sizeHigh.writeUInt16BE(BLOCK);
    put(16, 0, Buffer.from([1]));
    put(16, 1, Buffer.from("CD001"));
    put(16, 6, Buffer.from([1]));
    put(16, 40, Buffer.alloc(32, 0x20));
    put(16, 128, sizeLow);
    put(16, 130, sizeHigh);
    put(16, 156, root);
    put(17, 0, Buffer.from([255]));
    put(17, 1, Buffer.from("CD001"));
    put(17, 6, Buffer.from([1]));
    put(
      18,
      0,
      Buffer.concat([
        root,
        record(Buffer.from([1]), 18, BLOCK, true),
        record(
          Buffer.from("MARKERS.BIN;1"),
          IMAGE_SECTOR,
          IMAGE_FILE_BYTES,
          false,
        ),
      ]),
    );
    IMAGE_PLACES.forEach((place, index) => {
      put(IMAGE_SECTOR, place, Buffer.from([MARKER_BYTES[index]]));
    });
  } finally {
    closeSync(file);
  }
}

/** A sparse file of `size` bytes, with one byte planted when `plant` names one. */
function writeSparse(path, size, plant) {
  const file = openSync(path, "w");
  try {
    ftruncateSync(file, size);
    if (plant) writeSync(file, Buffer.from([plant.byte]), 0, 1, plant.at);
  } finally {
    closeSync(file);
  }
}

/**
 * A synthetic folder of 1,066 nested sparse files, 225 MiB in all, 5 markers
 * planted across its depths. Answers its count and the places to read, each
 * path from the disc's top in the kernel's spelling.
 */
function writeFolder(root) {
  const paths = [];
  FOLDER_DEPTHS.forEach((count, depth) => {
    for (let index = 0; index < count; index += 1) {
      const folders = Array.from(
        { length: depth },
        (_, level) => `L${level}N${index % (level + 2)}`,
      );
      const name = `F${depth}${String(index).padStart(4, "0")}.DAT`;
      paths.push([...folders, name].join("/"));
    }
  });
  const each = Math.floor(FOLDER_BYTES / paths.length);
  const last = FOLDER_BYTES - each * (paths.length - 1);
  const marked = [0, 444, 861, 905, paths.length - 1];
  const ats = [0, 1000, each - 1, 77777, last - 1];
  paths.forEach((path, index) => {
    const n = marked.indexOf(index);
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeSparse(
      join(root, path),
      index === paths.length - 1 ? last : each,
      n < 0 ? null : { at: ats[n], byte: MARKER_BYTES[n] },
    );
  });
  const places = marked.map((index, n) => ({
    path: paths[index].replaceAll("/", "\\"),
    at: ats[n],
  }));
  return { count: paths.length, bytes: FOLDER_BYTES, places };
}

/** True when the 5 bytes read are the 5 markers planted. */
function marksRight(bytes) {
  return JSON.stringify(bytes) === JSON.stringify(MARKER_BYTES);
}

/** The status line's states a pick's check, and a whole pick's keep, end in. */
const PICK_ENDS = [
  "kept",
  "no-room",
  "keep-failed",
  "wrong",
  "too-long",
  "check-failed",
];

/** The status line's states a press of Start ends in. */
const START_ENDS = [
  "started",
  "wrong",
  "too-long",
  "check-failed",
  "ready",
  "held-failed",
];

/** The status line's states a load's check of the kept copy ends in. */
const LOAD_ENDS = ["ready", "held", "wrong", "held-failed"];

/** What the page showed, its list cut to a count: a wrong folder lists hundreds. */
function brief({ findings, ...rest }) {
  return { ...rest, findings: findings.length };
}

/**
 * A synthetic folder of the disc's expected files, by their names alone:
 * 1 byte each, and 0 for those the disc holds empty. No byte is the game's.
 */
function writeNamed(root) {
  for (const path of DISC_FILES) {
    const file = join(root, path);
    mkdirSync(dirname(file), { recursive: true });
    writeFileSync(file, EMPTY_AT_SOURCE.includes(path) ? "" : "x");
  }
}

/**
 * The 4 files the `checking` check breaks, taken by their places in the expected
 * list, spread through it and none empty at source, so the driver names
 * none itself: removed, emptied, case changed, moved, in the list's order.
 */
function faultFiles() {
  const kept = DISC_FILES.filter((path) => !EMPTY_AT_SOURCE.includes(path));
  return [0, 1, 2, 3].map(
    (quarter) => kept[Math.floor((kept.length * quarter) / 4) + 1],
  );
}

/** The path with its last part's letters lowered: the name's case changed. */
function lowered(path) {
  return path.replace(/[^/]+$/, (name) => name.toLowerCase());
}

/** Waits for the status line to reach one of `ends`, then reads what the page shows. */
async function shown(page, ends) {
  await page.waitForFunction(
    (wanted) =>
      wanted.includes(document.getElementById("status").dataset.state),
    ends,
  );
  return page.evaluate(() => ({
    state: document.getElementById("status").dataset.state,
    sentence: document.getElementById("status").textContent,
    listed: !document.getElementById("findings").hidden,
    findings: [...document.querySelectorAll("#findings li")].map(
      (item) => item.textContent,
    ),
    start:
      document.getElementById("start").getAttribute("aria-disabled") !==
      "true",
  }));
}

/** The status line, and whether the disc's controls and Start are on. */
async function controls(page) {
  return page.evaluate(() => ({
    state: document.getElementById("status").dataset.state,
    sentence: document.getElementById("status").textContent,
    pick: !document.getElementById("pick").disabled,
    start:
      document.getElementById("start").getAttribute("aria-disabled") !==
      "true",
  }));
}

/** The events the INFO lines name, in order, and the other lines counted. */
function tally(text) {
  const lines = text.split("\n").filter((line) => line.length > 0);
  const parts = lines.map((line) => line.split(" "));
  const named = (level) =>
    parts.filter(([at]) => at === level).map(([, event]) => event);
  return {
    lines: lines.length,
    info: named("INFO"),
    warn: named("WARN"),
    debug: named("DEBUG").length,
    imp: lines.filter((line) => line.startsWith("DEBUG imp.call ")),
  };
}

/** The page's trace as its logger holds it now. */
async function traceOf(page) {
  return page.evaluate(async () => {
    const { logger } = await import("./track-logger.mjs");
    return logger.trace();
  });
}

/** Presses `press`, which saves the trace, and reads the file the browser was given. */
async function saved(page, press) {
  const [download] = await Promise.all([page.waitForEvent("download"), press()]);
  return {
    name: download.suggestedFilename(),
    text: readFileSync(await download.path(), "utf8"),
  };
}

/** The INFO lines one pick of an image, a folder, Start and the save write, in order. */
const PRESSES = [
  "launcher.image.picked",
  "launcher.folder.picked",
  "launcher.start",
  "launcher.log.saved",
];

/** How long a closed tab's hold may take to end in the browser before the `holding` check fails. */
const FREE_WAIT_MS = 5000;

/** The keys' order through the page's controls once a disc can be picked. */
const TAB_ORDER = ["image", "folder", "start", "save"];

/** The id of the element the keys are on, or null on the page itself. */
async function focused(page) {
  return page.evaluate(() => document.activeElement?.id || null);
}

/** Presses `key` on the focused control, which opens the file chooser, and picks `path`. */
async function chooseByKey(page, key, path) {
  const [chooser] = await Promise.all([
    page.waitForEvent("filechooser"),
    page.keyboard.press(key),
  ]);
  await chooser.setFiles(path);
}

/**
 * True when the page lists one sentence a `[path, mark]`, in order, each
 * naming its file and holding its kind's mark from the check's wording.
 */
function namesEach(seen, marks) {
  return (
    seen.findings.length === marks.length &&
    marks.every(
      ([path, mark], index) =>
        seen.findings[index].includes(path) &&
        seen.findings[index].includes(mark),
    )
  );
}

const CHECKS = {
  /**
   * Passes when the player's server sends both headers and the page reads
   * `crossOriginIsolated` true; the bare server sends neither and the page
   * shows the NOT_ISOLATED sentence. Either missing fails.
   */
  async isolation(run) {
    const ours = await readIsolation(
      await open(run.browser, `${run.served}/launcher.html`),
    );
    const bare = await readIsolation(
      await open(run.browser, `${run.bare}/launcher.html`),
    );
    const pass =
      ours.opener === "same-origin" &&
      ours.embedder === "require-corp" &&
      ours.isolated === true &&
      ours.state === "ready" &&
      ours.errors.length === 0 &&
      bare.opener === null &&
      bare.embedder === null &&
      bare.isolated === false &&
      bare.sentence === NOT_ISOLATED &&
      bare.errors.length === 0;
    return { pass, ours, bare };
  },

  /**
   * Passes when a synthetic 300 MiB disc image and a synthetic folder of
   * 1,066 nested files, 225 MiB in all, each picked through the page's own
   * input and read 5 markers right through `logical_disc`, the folder by its
   * paths, on a page opened with `?log=debug`, whose trace then holds the
   * hold's one `imp.call` line and the reader's DEBUG lines beside it. One
   * wrong read fails.
   */
  async reading(run) {
    const made = mkdtempSync(join(run.scratch, "launcher-discs-"));
    try {
      const image = join(made, "disc.iso");
      writeImage(image);
      const folder = join(made, "disc");
      const tree = writeFolder(folder);
      const opened = await open(
        run.browser,
        `${run.served}/launcher.html?log=debug`,
      );
      await opened.page.setInputFiles("#image", image);
      await opened.page.setInputFiles("#folder", folder);
      const asked = {
        image: IMAGE_PLACES.map((at) => ({ path: "MARKERS.BIN", at })),
        folder: tree.places,
      };
      const read = await opened.page.evaluate(async (places) => {
        const { readPlaces } = await import("./disc-reader.mjs");
        const image = document.getElementById("image").files;
        const picked = document.getElementById("folder").files;
        return {
          count: picked.length,
          first: picked[0]?.webkitRelativePath ?? null,
          image: await readPlaces(image, false, places.image),
          folder: await readPlaces(picked, true, places.folder),
        };
      }, asked);
      const trace = tally(await traceOf(opened.page));
      await opened.page.close();
      const pass =
        read.count === tree.count &&
        read.image.opened === 0 &&
        marksRight(read.image.bytes) &&
        read.folder.opened === 0 &&
        marksRight(read.folder.bytes) &&
        trace.imp.length === 1 &&
        trace.debug > trace.imp.length &&
        opened.errors.length === 0;
      return { pass, tree, ...read, trace, errors: opened.errors };
    } finally {
      rmSync(made, { recursive: true, force: true });
    }
  },

  /**
   * Passes when the page's check passes a complete synthetic tree, the disc's
   * expected names at 1 byte each, and Start then starts. A file moved on
   * disk after the pick gives its own sentence at Start, which then does
   * not start; a removed file, an emptied file and a name's case changed,
   * in a second tree, each give their own at the pick. Each sentence names
   * its file; a blank page or a wrong sentence fails.
   */
  async checking(run) {
    const made = mkdtempSync(join(run.scratch, "launcher-trees-"));
    try {
      const [removed, emptied, cased, moved] = faultFiles();
      const whole = join(made, "whole");
      writeNamed(whole);
      const wrong = join(made, "wrong");
      writeNamed(wrong);
      rmSync(join(wrong, removed));
      writeFileSync(join(wrong, emptied), "");
      renameSync(join(wrong, cased), join(wrong, lowered(cased)));
      const pages = [];
      const fresh = async () => {
        const opened = await open(run.browser, `${run.served}/launcher.html`);
        pages.push(opened);
        return opened.page;
      };

      let page = await fresh();
      await page.setInputFiles("#folder", whole);
      const wholePick = await shown(page, PICK_ENDS);
      await page.click("#start");
      const wholeStart = await shown(page, START_ENDS);

      page = await fresh();
      await page.setInputFiles("#folder", whole);
      const movedPick = await shown(page, PICK_ENDS);
      renameSync(join(whole, moved), join(whole, `${moved}.MOVED`));
      await page.click("#start");
      const movedStart = await shown(page, START_ENDS);

      page = await fresh();
      await page.setInputFiles("#folder", wrong);
      const wrongPick = await shown(page, PICK_ENDS);

      const errors = pages.flatMap((opened) => opened.errors);
      await Promise.all(pages.map((opened) => opened.page.close()));
      const pass =
        lowered(cased) !== cased &&
        wholePick.state === "kept" &&
        wholePick.sentence === KEPT &&
        !wholePick.listed &&
        wholePick.start &&
        wholeStart.state === "started" &&
        wholeStart.sentence === STARTED &&
        !wholeStart.start &&
        movedPick.state === "kept" &&
        movedStart.state === "wrong" &&
        movedStart.sentence === wrongFiles(1) &&
        movedStart.listed &&
        !movedStart.start &&
        namesEach(movedStart, [[moved, "can no longer be read"]]) &&
        wrongPick.state === "wrong" &&
        wrongPick.sentence === wrongFiles(3) &&
        wrongPick.listed &&
        !wrongPick.start &&
        namesEach(wrongPick, [
          [removed, "is not in the picked disc"],
          [emptied, "is empty"],
          [cased, `was picked as ${lowered(cased)}`],
        ]) &&
        errors.length === 0;
      return {
        pass,
        faults: { removed, emptied, cased, moved },
        wholePick,
        wholeStart,
        movedPick,
        movedStart,
        wrongPick,
        errors,
      };
    } finally {
      rmSync(made, { recursive: true, force: true });
    }
  },

  /**
   * Passes when, after a reload, the page reads a kept copy's 5 markers right
   * from page storage, or the disc must be picked again; the quota at 225 MiB
   * is filed either way. The `reading` check's folder, which the page's own check
   * refuses, is kept through the reader, reloaded, and read from the kept
   * copy, and the page must call that copy not whole. Then a whole synthetic
   * tree is picked through the page and kept, and after a reload, its folder
   * input empty, the page holds it and Start starts on it. A held copy
   * reading wrong, or a page claiming a disc it does not hold, fails.
   */
  async storage(run) {
    const made = mkdtempSync(join(run.scratch, "launcher-kept-"));
    try {
      const big = join(made, "disc");
      const tree = writeFolder(big);
      const whole = join(made, "whole");
      writeNamed(whole);

      const first = await open(run.browser, `${run.served}/launcher.html`);
      await first.page.setInputFiles("#folder", big);
      const bigPick = await shown(first.page, PICK_ENDS);
      const kept = await first.page.evaluate(async () => {
        const { keepPick } = await import("./disc-reader.mjs");
        return keepPick(document.getElementById("folder").files, true);
      });
      await first.page.reload();
      const bigLoad = await shown(first.page, LOAD_ENDS);
      const bigRead = await first.page.evaluate(async (places) => {
        const { readKept } = await import("./disc-reader.mjs");
        return readKept(places);
      }, tree.places);

      const second = await open(run.browser, `${run.served}/launcher.html`);
      await second.page.setInputFiles("#folder", whole);
      const wholePick = await shown(second.page, PICK_ENDS);
      await second.page.reload();
      const wholeLoad = await shown(second.page, LOAD_ENDS);
      const inputAfter = await second.page.evaluate(
        () => document.getElementById("folder").files.length,
      );
      await second.page.click("#start");
      const wholeStart = await shown(second.page, START_ENDS);

      const errors = [...first.errors, ...second.errors];
      await Promise.all([first.page.close(), second.page.close()]);
      const keptRight =
        kept.kept === 0 &&
        bigLoad.state === "wrong" &&
        bigLoad.sentence === heldWrong(bigLoad.findings.length) &&
        !bigLoad.start &&
        bigRead.held === true &&
        bigRead.opened === 0 &&
        marksRight(bigRead.bytes);
      const notKept =
        kept.kept === 1 && bigLoad.state === "ready" && bigRead.held === false;
      const pass =
        bigPick.state === "wrong" &&
        (keptRight || notKept) &&
        Number.isFinite(kept.usage) &&
        Number.isFinite(kept.quota) &&
        wholePick.state === "kept" &&
        wholePick.sentence === KEPT &&
        wholePick.start &&
        wholeLoad.state === "held" &&
        wholeLoad.sentence === HELD &&
        wholeLoad.start &&
        inputAfter === 0 &&
        wholeStart.state === "started" &&
        wholeStart.sentence === STARTED &&
        errors.length === 0;
      return {
        pass,
        tree: { count: tree.count, bytes: tree.bytes },
        kept,
        bigPick: brief(bigPick),
        bigLoad: brief(bigLoad),
        bigRead,
        wholePick: brief(wholePick),
        wholeLoad: brief(wholeLoad),
        inputAfter,
        wholeStart: brief(wholeStart),
        errors,
      };
    } finally {
      rmSync(made, { recursive: true, force: true });
    }
  },

  /**
   * Passes when, of two tabs on one origin in one browser, the first holds
   * the disc and checks its kept copy, and the second says HELD_ELSEWHERE,
   * its disc controls and Start off, having started no disc Worker at all;
   * once the first tab closes and the browser ends its hold, within
   * FREE_WAIT_MS, a reload of the second holds the disc. A Worker in each
   * tab at once fails.
   */
  async holding(run) {
    const context = await run.browser.newContext();
    const errors = [];
    const workers = [];
    const tab = async () => {
      const page = await context.newPage();
      const index = workers.push(0) - 1;
      page.on("worker", (worker) => {
        if (worker.url().includes("disc-worker.mjs")) workers[index] += 1;
      });
      page.on("pageerror", (error) => errors.push(error.message));
      await page.goto(`${run.served}/launcher.html`);
      await settle(page);
      return page;
    };
    try {
      const first = await tab();
      const second = await tab();
      const firstSeen = await controls(first);
      const secondSeen = await controls(second);
      const workersBoth = [...workers];
      await first.close();
      const freedMs = await second.evaluate(async (waitMs) => {
        const began = performance.now();
        while (performance.now() - began < waitMs) {
          const { held } = await navigator.locks.query();
          if (held.length === 0) return Math.round(performance.now() - began);
          await new Promise((wake) => setTimeout(wake, 20));
        }
        return null;
      }, FREE_WAIT_MS);
      await second.reload();
      await settle(second);
      const afterSeen = await controls(second);
      const pass =
        firstSeen.state === "ready" &&
        firstSeen.pick &&
        workersBoth[0] === 1 &&
        secondSeen.state === "held-elsewhere" &&
        secondSeen.sentence === HELD_ELSEWHERE &&
        !secondSeen.pick &&
        !secondSeen.start &&
        workersBoth[1] === 0 &&
        freedMs !== null &&
        afterSeen.state === "ready" &&
        afterSeen.pick &&
        workers[1] === 1 &&
        errors.length === 0;
      return {
        pass,
        firstSeen,
        secondSeen,
        freedMs,
        afterSeen,
        workersBoth,
        workersAfter: workers,
        errors,
      };
    } finally {
      await context.close();
    }
  },

  /**
   * Passes when each press writes one INFO line, in order, and the file the
   * save button gives holds the page's trace whole, read back from the
   * browser's download. A plain run's 4 presses, an image, a whole folder,
   * Start and the save, give 4 INFO lines, no DEBUG and no `imp.call`. The
   * same presses with `?log=debug`, a file moved before Start, add the
   * hold's one `imp.call` line, the reader's DEBUG lines, and the moved
   * file's `file.read.failed` WARN. Any other count fails.
   */
  async logging(run) {
    const made = mkdtempSync(join(run.scratch, "launcher-logs-"));
    try {
      const image = join(made, "disc.iso");
      writeFileSync(image, "x");
      const pages = [];
      const presses = async (query, whole, moved) => {
        const opened = await open(
          run.browser,
          `${run.served}/launcher.html${query}`,
        );
        pages.push(opened);
        const { page } = opened;
        await page.setInputFiles("#image", image);
        const imageSeen = await shown(page, ["image"]);
        await page.setInputFiles("#folder", whole);
        const pickSeen = await shown(page, PICK_ENDS);
        if (moved) renameSync(join(whole, moved), join(whole, `${moved}.MOVED`));
        await page.click("#start");
        const startSeen = await shown(page, START_ENDS);
        const file = await saved(page, () => page.click("#save"));
        return {
          imageSeen: brief(imageSeen),
          pickSeen: brief(pickSeen),
          startSeen: brief(startSeen),
          name: file.name,
          trace: tally(file.text),
          same: file.text === (await traceOf(page)),
        };
      };
      const plainTree = join(made, "plain");
      writeNamed(plainTree);
      const plain = await presses("", plainTree, null);
      const debugTree = join(made, "debug");
      writeNamed(debugTree);
      const [, , , moved] = faultFiles();
      const debug = await presses("?log=debug", debugTree, moved);
      const errors = pages.flatMap((opened) => opened.errors);
      await Promise.all(pages.map((opened) => opened.page.close()));
      const pressed = (seen) =>
        JSON.stringify(seen.trace.info) === JSON.stringify(PRESSES) &&
        seen.imageSeen.sentence === IMAGE_NOT_CHECKED &&
        seen.pickSeen.state === "kept" &&
        seen.name === "launcher-log.txt" &&
        seen.same;
      const pass =
        pressed(plain) &&
        plain.startSeen.state === "started" &&
        plain.trace.debug === 0 &&
        plain.trace.imp.length === 0 &&
        plain.trace.warn.length === 0 &&
        pressed(debug) &&
        debug.startSeen.state === "wrong" &&
        debug.trace.imp.length === 1 &&
        debug.trace.imp[0].includes('"c":"USER32!FindWindowA","v":0') &&
        debug.trace.debug > debug.trace.imp.length &&
        JSON.stringify(debug.trace.warn) ===
          JSON.stringify(["file.read.failed"]) &&
        errors.length === 0;
      return { pass, moved, plain, debug, errors };
    } finally {
      rmSync(made, { recursive: true, force: true });
    }
  },

  /**
   * Passes when axe finds no violation at each state the page shows (ready,
   * an image picked, a whole folder kept, started, a moved file's list, a
   * second tab's held-elsewhere, the bare server's not-isolated); when the
   * keys alone move through the image, the folder, Start and the save in
   * that order, pick a whole folder through the file chooser, start, and
   * save; and when Start keeps the focus through its own press's check,
   * whole and wrong. A test tool only: it claims no conformance, as a
   * person's walk with a screen reader has not been made.
   */
  async keys(run) {
    if (!run.axe) return { pass: false, axe: null };
    const made = mkdtempSync(join(run.scratch, "launcher-keys-"));
    const context = await run.browser.newContext();
    const errors = [];
    const scans = {};
    const scan = async (page, name) => {
      if (!(await page.evaluate(() => "axe" in globalThis)))
        await page.addScriptTag({ path: run.axe });
      scans[name] = await page.evaluate(async () =>
        (await globalThis.axe.run(document)).violations.map((violation) => ({
          id: violation.id,
          nodes: violation.nodes.length,
        })),
      );
    };
    const tabTo = async (page) => {
      await page.keyboard.press("Tab");
      return focused(page);
    };
    try {
      const image = join(made, "disc.iso");
      writeFileSync(image, "x");
      const whole = join(made, "whole");
      writeNamed(whole);
      // A second folder for the pick after a start: Chromium fires no input
      // or change event when the same folder is picked again unchanged, so
      // the chooser's setFiles would wait for an event that never comes.
      const again = join(made, "again");
      writeNamed(again);
      const [, , , moved] = faultFiles();

      const page = await context.newPage();
      page.on("pageerror", (error) => errors.push(error.message));
      await page.goto(`${run.served}/launcher.html`);
      await settle(page);
      const ready = await controls(page);
      await scan(page, "ready");
      const order = [];
      for (const _ of TAB_ORDER) order.push(await tabTo(page));
      await page.keyboard.press("Shift+Tab");
      await page.keyboard.press("Shift+Tab");
      await page.keyboard.press("Shift+Tab");
      const back = await focused(page);
      await chooseByKey(page, "Enter", image);
      const imageSeen = await shown(page, ["image"]);
      await scan(page, "image");
      const toFolder = await tabTo(page);
      await chooseByKey(page, "Enter", whole);
      const kept = await shown(page, PICK_ENDS);
      await scan(page, "kept");
      const toStart = await tabTo(page);
      await page.keyboard.press("Enter");
      const started = await shown(page, START_ENDS);
      const focusStarted = await focused(page);
      await scan(page, "started");
      const toSave = await tabTo(page);
      const file = await saved(page, () => page.keyboard.press("Enter"));
      await page.keyboard.press("Shift+Tab");
      await page.keyboard.press("Shift+Tab");
      const backToFolder = await focused(page);
      await chooseByKey(page, "Enter", again);
      const keptAgain = await shown(page, PICK_ENDS);
      const backToStart = await tabTo(page);
      renameSync(join(again, moved), join(again, `${moved}.MOVED`));
      await page.keyboard.press("Enter");
      const wrong = await shown(page, START_ENDS);
      const focusWrong = await focused(page);
      await scan(page, "wrong");

      const second = await context.newPage();
      second.on("pageerror", (error) => errors.push(error.message));
      await second.goto(`${run.served}/launcher.html`);
      await settle(second);
      const elsewhere = await controls(second);
      await scan(second, "held-elsewhere");

      const bare = await open(run.browser, `${run.bare}/launcher.html`);
      errors.push(...bare.errors);
      const notIsolated = await controls(bare.page);
      await scan(bare.page, "not-isolated");
      await bare.page.close();

      const clean = Object.values(scans).every((found) => found.length === 0);
      const pass =
        clean &&
        Object.keys(scans).length === 7 &&
        ready.state === "ready" &&
        JSON.stringify(order) === JSON.stringify(TAB_ORDER) &&
        back === "image" &&
        imageSeen.state === "image" &&
        toFolder === "folder" &&
        kept.state === "kept" &&
        toStart === "start" &&
        started.state === "started" &&
        focusStarted === "start" &&
        toSave === "save" &&
        JSON.stringify(tally(file.text).info) === JSON.stringify(PRESSES) &&
        backToFolder === "folder" &&
        keptAgain.state === "kept" &&
        backToStart === "start" &&
        wrong.state === "wrong" &&
        focusWrong === "start" &&
        elsewhere.state === "held-elsewhere" &&
        notIsolated.state === "not-isolated" &&
        errors.length === 0;
      return {
        pass,
        axe: run.axe,
        scans,
        order,
        back,
        imageSeen: brief(imageSeen),
        toFolder,
        kept: brief(kept),
        toStart,
        started: brief(started),
        focusStarted,
        toSave,
        saved: tally(file.text).info,
        backToFolder,
        keptAgain: brief(keptAgain),
        backToStart,
        wrong: brief(wrong),
        focusWrong,
        elsewhere,
        notIsolated,
        errors,
      };
    } finally {
      await context.close();
      rmSync(made, { recursive: true, force: true });
    }
  },
};

async function main() {
  const [outputPath, ...wanted] = process.argv.slice(2);
  const { SERVE, KIT, PLAYWRIGHT, SCRATCH, DISC_WASM } = process.env;
  if (!outputPath || !SERVE || !KIT || !PLAYWRIGHT || !SCRATCH || !DISC_WASM) {
    process.stderr.write(
      "call: SERVE= KIT= PLAYWRIGHT= SCRATCH= DISC_WASM= node launcher-browser.mjs <output.json> [check ...]\n",
    );
    process.exit(2);
  }
  const unknown = wanted.filter((name) => !(name in CHECKS));
  if (unknown.length > 0) {
    process.stderr.write(`no check named ${unknown.join(", ")}\n`);
    process.exit(2);
  }
  const root = assemble(KIT, DISC_WASM, SCRATCH);
  const serve = await startServe(SERVE, root);
  const bare = await startBare(root);
  const { chromium } = await import(PLAYWRIGHT);
  const browser = await chromium.launch();
  const run = {
    browser,
    served: serve.address,
    bare: bare.address,
    scratch: SCRATCH,
    axe: process.env.AXE ?? null,
  };
  const answers = {};
  const output = { chromium: browser.version(), served: serve.address };
  try {
    for (const name of wanted.length > 0 ? wanted : Object.keys(CHECKS)) {
      try {
        answers[name] = await CHECKS[name](run);
      } catch (error) {
        answers[name] = { pass: false, threw: error.message };
      }
    }
  } finally {
    await browser.close();
    bare.server.close();
    serve.child.kill();
    rmSync(root, { recursive: true, force: true });
    output.answers = answers;
    output.serveLog = serve.log.join("");
    writeFileSync(outputPath, `${JSON.stringify(output, null, 1)}\n`);
  }
  process.exit(0);
}

await main();
