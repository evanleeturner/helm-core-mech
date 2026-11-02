// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b6f44: u32;
    static mut DAT_100942e8: u32;
    static mut DAT_100942ec: u32;
    static mut DAT_100942f0: u32;
    static mut DAT_100942f4: u32;
    static mut DAT_100942f8: u32;
    static mut DAT_100942fc: u32;
    static mut DAT_10094300: u32;
    static mut DAT_10094304: u32;
    static mut DAT_10094308: u32;
    static mut DAT_1009430c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003a640() {
    // SAFETY: the globals are the game's own memory, written in the original's order.
    unsafe {
        DAT_100b6f44 = 0xffffffff;
        DAT_100942e8 = 0;
        DAT_100942ec = 0;
        DAT_100942f0 = 0;
        DAT_100942f4 = 0;
        DAT_100942f8 = 0;
        DAT_100942fc = 0;
        DAT_10094300 = 0;
        DAT_10094304 = 0;
        DAT_10094308 = 0;
        DAT_1009430c = 0;
    }
}
