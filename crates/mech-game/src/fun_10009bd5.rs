// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10215658: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10009bd5() -> u8 {
    // SAFETY: the global is the game's own memory, read by value.
    unsafe { (DAT_10215658 == 4) as u8 }
}
