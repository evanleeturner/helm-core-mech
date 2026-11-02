// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10094f70: u32;
    static mut DAT_10094f74: u32;
    static mut DAT_10094f78: u32;
    static mut DAT_10094f7c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003cc60() {
    // SAFETY: the globals are the game's own memory; written in the original's order.
    unsafe {
        DAT_10094f74 = 0;
        DAT_10094f78 = 0;
        DAT_10094f7c = 0;
        DAT_10094f70 = 0;
    }
}
