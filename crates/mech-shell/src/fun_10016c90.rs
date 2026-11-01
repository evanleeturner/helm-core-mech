// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 10016c90, from `src/mw2shell/10016c90.c` (`SetGlobalValue`):
//! `__cdecl`, one `unsigned int` argument (the C's `newValue`), stored at
//! `DAT_1008edfc`, here [`ShellState::dat_1008edfc`].

use log::debug;

use crate::ShellState;

/// Stores `new_value` at `DAT_1008edfc`.
pub fn FUN_10016c90(state: &mut ShellState, new_value: u32) {
    debug!("FUN_10016c90(new_value={new_value:#x})");
    state.dat_1008edfc = new_value;
    debug!("FUN_10016c90: dat_1008edfc = {:#x}", state.dat_1008edfc);
}
