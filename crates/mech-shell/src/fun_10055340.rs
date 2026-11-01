// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 10055340, from `src/mw2shell/10055340.c` (`FUN_10055340`): `__stdcall`, no
//! arguments, returns at once.

use log::debug;

/// Does nothing: the C returns at once.
pub fn FUN_10055340() {
    debug!("FUN_10055340()");
}
