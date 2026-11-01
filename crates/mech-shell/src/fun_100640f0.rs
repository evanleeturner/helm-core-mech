// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 100640f0, from `src/mw2shell/100640f0.c` (`FUN_100640f0`): `__stdcall`, no
//! arguments, returns at once.

use log::debug;

/// Does nothing: the C returns at once.
pub fn FUN_100640f0() {
    debug!("FUN_100640f0()");
}
