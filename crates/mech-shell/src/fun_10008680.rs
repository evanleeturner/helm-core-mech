// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 10008680, from `src/mw2shell/10008680.c`: `__stdcall`, no
//! arguments, returns at once.

use log::debug;

/// Does nothing: the original is a lone `RET`.
pub fn FUN_10008680() {
    debug!("FUN_10008680()");
}
