// SPDX-License-Identifier: MIT
/**
 * The launcher page's main thread: it says whether the page can run, takes
 * the disc's pick, checks it, keeps a copy, and starts. Until phase 2
 * exists, Start says in a sentence that the menus come next.
 *
 * The game's own pages block a Worker on shared memory, which a browser
 * allows only on a cross-origin isolated page. So the launcher refuses to
 * take a disc on a page that could not start the game: without the two
 * headers it stays on the NOT_ISOLATED sentence with every control off.
 * The status line's `data-state` names the state the sentence is for.
 *
 * A folder's pick is checked against the disc's expected files at the pick
 * and again at Start: a file moved after the pick shows only when the check
 * runs again, and Start must not run on a disc that moved. Each wrong
 * file's sentence comes whole from the disc's check, one item under the
 * status line. An image's pick is not checked yet, so it leaves Start off.
 *
 * A whole folder pick is copied into the browser's page storage, Start off
 * while it copies, so a reload need not pick the disc again; a store with
 * no room, or a copy that failed, says the disc must be picked again after
 * a reload. At load the kept copy, if any, is checked like a pick, and a
 * whole one turns Start on. After a reload the folder input is empty, so
 * Start checks the disc it was given: this page's pick, or the kept copy.
 */
import { checkKept, checkPick, keepPick } from "./disc-reader.mjs";
import { DISC_FILES, EMPTY_AT_SOURCE } from "./launcher-disc.mjs";
import {
  CHECK_FAILED,
  CHECKING,
  CHECKING_HELD,
  HELD,
  HELD_FAILED,
  KEEP_FAILED,
  KEEPING,
  KEPT,
  NO_ROOM,
  NOT_ISOLATED,
  READY,
  STARTED,
  TOO_LONG,
  heldWrong,
  wrongFiles,
} from "./launcher-words.mjs";

const status = document.getElementById("status");
const findings = document.getElementById("findings");
const pick = document.getElementById("pick");
const folder = document.getElementById("folder");
const start = document.getElementById("start");

/** Every expected file, marked where the disc itself holds it empty. */
const EXPECTED = DISC_FILES.map((path) => ({
  path,
  empty: EMPTY_AT_SOURCE.includes(path),
}));

/** The `kept` answer the reader gives for each way a keep can end. */
const KEPT_WHOLE = 0;
const KEPT_NO_ROOM = 1;

/** Counts the checks begun, so only the latest one's answer is shown. */
let checks = 0;

/**
 * The disc Start checks once it is on: "picked" for this page's folder
 * pick, "held" for the copy kept in page storage.
 */
let disc = null;

/** The keeps in order, so a second pick's copy never runs beside the first's. */
let keeping = Promise.resolve();

/** Shows one sentence and names the state it is for. */
function say(state, sentence) {
  status.dataset.state = state;
  status.textContent = sentence;
}

/** Lists one item a sentence under the status line; none hides the list. */
function list(sentences) {
  findings.replaceChildren(
    ...sentences.map((sentence) => {
      const item = document.createElement("li");
      item.textContent = sentence;
      return item;
    }),
  );
  findings.hidden = sentences.length === 0;
}

/**
 * Checks `source`, "picked" or "held", Start off while it runs, and shows
 * what was wrong. Resolves true when the disc holds every expected file
 * readable, the status line then the caller's to set; false otherwise, and
 * false too when a later check began meanwhile, whose answer is the one
 * shown. A held copy that is not there says READY: the disc is to be
 * picked.
 */
async function check(source) {
  checks += 1;
  const mine = checks;
  const held = source === "held";
  start.disabled = true;
  list([]);
  say("checking", held ? CHECKING_HELD : CHECKING);
  let answer = null;
  try {
    answer = held
      ? await checkKept(EXPECTED)
      : await checkPick(folder.files, EXPECTED);
  } catch {
    // The reader's Worker stopped before answering: the failed sentence says so.
  }
  if (mine !== checks) return false;
  if (held && answer?.held === false) {
    say("ready", READY);
    return false;
  }
  if (answer === null || (held && answer.checked !== 0)) {
    say(
      held ? "held-failed" : "check-failed",
      held ? HELD_FAILED : CHECK_FAILED,
    );
    return false;
  }
  if (answer.checked !== 0) {
    say("too-long", TOO_LONG);
    return false;
  }
  if (answer.sentences.length > 0) {
    const count = answer.sentences.length;
    say("wrong", held ? heldWrong(count) : wrongFiles(count));
    list(answer.sentences);
    return false;
  }
  return true;
}

/**
 * Copies the whole pick into page storage after any keep still running,
 * Start off until it ends, and says how it ended; Start then runs on the
 * pick either way. A later check begun meanwhile shows its own answer.
 */
async function keep() {
  const mine = checks;
  const picked = folder.files;
  say("keeping", KEEPING);
  const copied = keeping.then(() => keepPick(picked, true));
  keeping = copied.catch(() => null);
  let answer = null;
  try {
    answer = await copied;
  } catch {
    // The reader's Worker stopped before answering: KEEP_FAILED says so.
  }
  if (mine !== checks) return;
  if (answer?.kept === KEPT_WHOLE) say("kept", KEPT);
  else if (answer?.kept === KEPT_NO_ROOM) say("no-room", NO_ROOM);
  else say("keep-failed", KEEP_FAILED);
  start.disabled = false;
}

folder.addEventListener("change", async () => {
  if (!(await check("picked"))) return;
  disc = "picked";
  await keep();
});

start.addEventListener("click", async () => {
  if (await check(disc)) say("started", STARTED);
});

if (globalThis.crossOriginIsolated) {
  pick.disabled = false;
  if (await check("held")) {
    disc = "held";
    say("held", HELD);
    start.disabled = false;
  }
} else {
  say("not-isolated", NOT_ISOLATED);
}
