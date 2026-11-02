// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 10059870, from `src/mw2shell/10059870.c` (`FUN_10059870`):
//! `__stdcall`, no arguments, zeroes the `int`s at `DAT_100a4b10` and
//! `DAT_100a4b14`, here [`ShellState::dat_100a4b10`] and
//! [`ShellState::dat_100a4b14`].

use crate::ShellState;

/// Stores 0 at `DAT_100a4b10`, then at `DAT_100a4b14`.
pub fn FUN_10059870(state: &mut ShellState) {
    mech_trace::enter!("10059870");
    mech_trace::gw!("10059870", "100a4b10", state.dat_100a4b10, 0);
    mech_trace::gw!("10059870", "100a4b14", state.dat_100a4b14, 0);
}
