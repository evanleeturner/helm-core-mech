// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c9050: u32;
    static mut DAT_100c9054: u32;
    static mut DAT_100c9058: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1008605a() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10086254() -> u8 {
    // SAFETY: the globals and the callee are the game's own memory and code.
    unsafe {
        DAT_100c9050 = 0;
        DAT_100c9054 = 0;
        DAT_100c9058 = 0;
        FUN_1008605a();
    }
    0
}
