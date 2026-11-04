// SPDX-License-Identifier: MIT
//! mercs's disc row (DS2): what the game's finder and music device ask of
//! its disc, as DS1 read them (wall 1). The marker `OLD_MERC.DRV` at a CD
//! drive's root, looked for by one unported routine a file (`10081830`
//! the game, `10001000` the shell, `00401000` the launcher), the game's own
//! folder's drive tried first; 21 audio tracks, 2 to 22; TMSF time, set by
//! the music's `10046175`. No store install of mercs is measured: no store
//! rule.
use logical_disc::{Row, Time};

#[cfg(test)]
mod tests;

/// The disc the game, the shell and the launcher find.
pub const DISC: Row = Row {
    marker: "OLD_MERC.DRV",
    tracks: 21,
    time: Time::Tmsf,
    store: None,
};
