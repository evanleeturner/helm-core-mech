// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10094e50: u32;
    static mut DAT_10094e54: u32;
    static mut DAT_10094e58: u32;
    static mut DAT_10094e5c: u32;
    static mut DAT_10094e60: u32;
    static mut DAT_100b9d50: u32;
    static mut DAT_100b9d54: u8;
    static mut DAT_100b9d58: u32;
    static mut DAT_100b9d5c: u32;
    static mut DAT_100b9d64: u32;
    static mut DAT_100b9d68: u32;
    static mut DAT_100b9d6c: u32;
    static mut DAT_100b9d70: u32;
    static mut DAT_100b9d74: u32;
    static mut DAT_100b9d78: u32;
    static mut DAT_100b9d80: u32;
    static mut DAT_100b9d84: u32;
    static mut DAT_100b9d88: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003bc40() {
    // SAFETY: the globals are the game's own memory; written in the original order.
    unsafe {
        DAT_10094e50 = 0;
        DAT_10094e54 = 0;
        DAT_10094e58 = 0;
        DAT_10094e5c = 0;
        DAT_100b9d6c = 0;
        DAT_10094e60 = 0;
        DAT_100b9d68 = 0;
        DAT_100b9d58 = 0;
        DAT_100b9d50 = 0;
        DAT_100b9d80 = 0;
        DAT_100b9d5c = 0;
        DAT_100b9d64 = 0;
        DAT_100b9d88 = 0;
        DAT_100b9d78 = 0;
        DAT_100b9d70 = 0;
        DAT_100b9d74 = 0;
        DAT_100b9d84 = 0;
        DAT_100b9d54 = 0;
    }
}
