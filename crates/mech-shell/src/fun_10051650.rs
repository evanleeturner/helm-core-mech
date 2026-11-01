// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 10051650, from `src/mw2shell/10051650.c` (`FUN_10051650`): `__stdcall`, no
//! arguments, returns 1 as an `unsigned int`.

use log::debug;

/// Returns 1.
#[must_use]
pub fn FUN_10051650() -> u32 {
    let result = 1;
    debug!("FUN_10051650() -> {result}");
    result
}
