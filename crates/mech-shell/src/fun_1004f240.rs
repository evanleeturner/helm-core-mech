// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 1004f240, from `src/mw2shell/1004f240.c` (`FUN_1004f240`):
//! `__cdecl`, one `int` argument; stores it at `DAT_1009bdf8`, here
//! [`ShellState::dat_1009bdf8`], and returns 0 when it lies in 0 to 4, else
//! returns 2 and stores nothing.

use crate::ShellState;

/// Stores `param_1` at `DAT_1009bdf8` and returns 0 for 0 to 4; returns 2
/// otherwise. The C's test, `-1 < param_1 && param_1 < 5`, is the range.
pub fn FUN_1004f240(state: &mut ShellState, param_1: i32) -> i32 {
    if (0..5).contains(&param_1) {
        state.dat_1009bdf8 = param_1;
        return 0;
    }
    2
}
