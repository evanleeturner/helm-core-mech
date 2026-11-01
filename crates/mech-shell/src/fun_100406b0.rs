// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 100406b0, from `src/mw2shell/100406b0.c` (`FUN_100406b0`): `__stdcall`, no
//! arguments, returns 0 as an `unsigned int`.

use log::debug;

/// Returns 0.
#[must_use]
pub fn FUN_100406b0() -> u32 {
    let result = 0;
    debug!("FUN_100406b0() -> {result}");
    result
}
