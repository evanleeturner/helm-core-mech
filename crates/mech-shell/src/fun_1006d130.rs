// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 1006d130, from `src/mw2shell/1006d130.c` (`FUN_1006d130`):
//! `__stdcall`, no arguments, returns the game's `unsigned int` at
//! `DAT_100a6798`, here [`ShellState::dat_100a6798`].

use crate::ShellState;

/// Returns the word at `DAT_100a6798`.
#[must_use]
pub fn FUN_1006d130(state: &ShellState) -> u32 {
    state.dat_100a6798
}
