// SPDX-License-Identifier: MIT
//! mercs's sound rows, as `sound_mix`'s front serves them: the module each
//! file's own import table names for Miles, which differ.
use sound_mix::table::Row;

/// The game file imports `wail32.dll`.
pub const GAME: Row = Row { file: "mercsw", module: "WAIL32" };

/// The shell imports `mss32.dll`. No front holds this row yet: whether the 2 files
/// share one is the first flight's to pin.
pub const SHELL: Row = Row { file: "mw2shell", module: "MSS32" };
