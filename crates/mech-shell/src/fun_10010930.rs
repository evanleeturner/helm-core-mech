// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 10010930, from `src/mw2shell/10010930.c` (`GetGlobalVolumeScale_Likely`):
//! `__stdcall`, no arguments, returns the game's `unsigned int` at
//! `DAT_100a67f4`, here [`ShellState::dat_100a67f4`].

use log::debug;

use crate::ShellState;

/// Returns the word at `DAT_100a67f4`.
#[must_use]
pub fn FUN_10010930(state: &ShellState) -> u32 {
    let value = state.dat_100a67f4;
    debug!("FUN_10010930() -> {value:#x}");
    value
}
