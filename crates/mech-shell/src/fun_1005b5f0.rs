// SPDX-License-Identifier: MIT
//! `MW2SHELL.DLL` 1005b5f0, from `src/mw2shell/1005b5f0.c` (`ReturnRecordCountOfOne_Likely`): `__stdcall`, no
//! arguments, returns 1 as an `unsigned int`.

use log::debug;

/// Returns 1.
#[must_use]
pub fn FUN_1005b5f0() -> u32 {
    let result = 1;
    debug!("FUN_1005b5f0() -> {result}");
    result
}
