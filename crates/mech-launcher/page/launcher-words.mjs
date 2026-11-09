// SPDX-License-Identifier: MIT
/**
 * The launcher page's own words: each sentence the page itself says, in one
 * place, so a reword is one edit and the browser run checks the very text
 * the page shows. A wrong file's sentence is not here: the disc's check in
 * oxide95 writes it whole, naming no game, and the page shows it as given.
 * Pure, with no DOM, so Node imports it too.
 */

/** Shown, with every control off, when the server sent neither header. */
export const NOT_ISOLATED =
  "This page needs two safety headers from its local server, and this server did not send them, so the disc cannot be picked. Open it through its own local server.";

/** Shown once the page can run, before any disc is picked. */
export const READY =
  "Ready. Pick the disc, as its image or its mounted folder, then press Start.";

/** Shown while the picked folder is checked against the disc's files. */
export const CHECKING = "Checking the picked folder against the disc's files.";

/** Shown at load, and at Start on a kept copy, while that copy is checked. */
export const CHECKING_HELD =
  "Looking for a copy of the disc kept in this browser, and checking it against the disc's files.";

/** Shown while a whole pick is copied into this browser's storage. */
export const KEEPING =
  "The picked folder holds the whole disc. Keeping a copy of it in this browser, so a reload need not pick it again; Start waits for the copy.";

/** Shown when a whole pick was copied whole into this browser's storage. */
export const KEPT =
  "The picked folder holds the whole disc, and this browser has kept a copy, so a reload need not pick it again. Press Start.";

/** Shown when a whole pick could not be kept because the store was full. */
export const NO_ROOM =
  "The picked folder holds the whole disc, but this browser had no room to keep a copy, so after a reload the disc must be picked again. Press Start.";

/** Shown when a whole pick could not be kept for any other reason. */
export const KEEP_FAILED =
  "The picked folder holds the whole disc, but this browser could not keep a copy, so after a reload the disc must be picked again. Press Start.";

/** Shown at load when this browser holds a whole copy from an earlier pick. */
export const HELD =
  "This browser kept a whole copy of the disc from an earlier pick. Press Start, or pick the disc again.";

/** Shown when the kept copy could not be checked; the disc must be picked again. */
export const HELD_FAILED =
  "The copy of the disc this browser kept could not be checked, so it cannot start. Pick the disc again.";

/** `count` files, as the wrong-file sentences say it. */
function filesAre(count) {
  return count === 1 ? "1 file is" : `${count} files are`;
}

/** Shown above the list of a pick's wrong files, `count` of them. */
export function wrongFiles(count) {
  return `The picked disc is not whole: ${filesAre(count)} wrong, each named below. Pick the disc again; Start stays off until then.`;
}

/** Shown above the list of the kept copy's wrong files, `count` of them. */
export function heldWrong(count) {
  return `The copy of the disc this browser kept is not whole: ${filesAre(count)} wrong, each named below. Pick the disc again; Start stays off until then.`;
}

/** Shown when a picked path is too long to be one of the disc's own. */
export const TOO_LONG =
  "A path in the picked folder is too long to be one of the disc's, so the folder cannot be checked. Pick the mounted disc's own folder.";

/** Shown when the check itself stopped before answering. */
export const CHECK_FAILED =
  "The picked folder could not be checked, because the page's disc reader stopped. Reload the page and pick the disc again.";

/** Shown at Start, until the game's menus exist. */
export const STARTED =
  "The disc checked whole. The game's menus come next; they are not built yet.";
