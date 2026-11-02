// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10218074: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10002415() -> u32 {
    // SAFETY: DAT_10218074 is the game's own global, read by value.
    unsafe { DAT_10218074 }
}
