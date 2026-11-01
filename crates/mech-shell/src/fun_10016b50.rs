// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 10016b50, from `src/mw2shell/10016b50.c` (`SetGlobalCallback_Likely`):
//! `__cdecl`, one `unsigned int` argument (the C's `callback_Likely`), stored at
//! `DAT_1008edf4`, here [`ShellState::dat_1008edf4`].

use crate::ShellState;

/// Stores `callback_likely` at `DAT_1008edf4`.
pub fn FUN_10016b50(state: &mut ShellState, callback_likely: u32) {
    state.dat_1008edf4 = callback_likely;
}
