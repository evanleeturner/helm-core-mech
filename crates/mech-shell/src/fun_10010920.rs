// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 10010920, from `src/mw2shell/10010920.c` (`FUN_10010920`):
//! `__stdcall`, no arguments, returns the game's `unsigned int` at
//! `DAT_100a67fc`, here [`ShellState::dat_100a67fc`].

use log::debug;

use crate::ShellState;

/// Returns the word at `DAT_100a67fc`.
#[must_use]
pub fn FUN_10010920(state: &ShellState) -> u32 {
    let value = state.dat_100a67fc;
    debug!("FUN_10010920() -> {value:#x}");
    value
}
