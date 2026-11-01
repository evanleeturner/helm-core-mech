// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 10059e60, from `src/mw2shell/10059e60.c` (`FUN_10059e60`):
//! `__cdecl`, two `int` arguments, stored at `DAT_100a4bb8` and
//! `DAT_100a4bbc`, here [`ShellState::dat_100a4bb8`] and
//! [`ShellState::dat_100a4bbc`].

use crate::ShellState;

/// Stores `param_1` at `DAT_100a4bb8` and `param_2` at `DAT_100a4bbc`.
pub fn FUN_10059e60(state: &mut ShellState, param_1: i32, param_2: i32) {
    state.dat_100a4bb8 = param_1;
    state.dat_100a4bbc = param_2;
}
