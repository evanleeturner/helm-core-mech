// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 1002f31a, from `src/mw2shell/1002f31a.c` (`FUN_1002f31a`):
//! `__cdecl`, two `unsigned int` arguments, stored at `DAT_10090410` and
//! `DAT_10090414`, here [`ShellState::dat_10090410`] and
//! [`ShellState::dat_10090414`].

use log::debug;

use crate::ShellState;

/// Stores `param_1` at `DAT_10090410` and `param_2` at `DAT_10090414`.
pub fn FUN_1002f31a(state: &mut ShellState, param_1: u32, param_2: u32) {
    debug!("FUN_1002f31a(param_1={param_1:#x}, param_2={param_2:#x})");
    state.dat_10090410 = param_1;
    debug!("FUN_1002f31a: dat_10090410 = {:#x}", state.dat_10090410);
    state.dat_10090414 = param_2;
    debug!("FUN_1002f31a: dat_10090414 = {:#x}", state.dat_10090414);
}
