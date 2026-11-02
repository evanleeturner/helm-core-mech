// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7cec: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100468d8() -> u32 {
    // SAFETY: the global is the game's own memory, read by value.
    unsafe { DAT_100c7cec }
}
