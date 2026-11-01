// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 1004b060, from `src/mw2shell/1004b060.c` (`FUN_1004b060`): `__stdcall`, no
//! arguments, returns 0 as an `unsigned char`.

use log::debug;

/// Returns 0.
#[must_use]
pub fn FUN_1004b060() -> u8 {
    let result = 0;
    debug!("FUN_1004b060() -> {result}");
    result
}
