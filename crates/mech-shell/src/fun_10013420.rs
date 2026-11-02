// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008ec74: u32;
    static mut DAT_1008ec78: u8;
    static mut DAT_100b0f3c: u8;
    static mut DAT_1008ec7c: u32;
    static mut DAT_1008ec80: u32;
    static mut DAT_1008ec84: u32;
    static mut DAT_1008ec88: u32;
    static mut DAT_1008ec8c: u32;
    static mut DAT_1008ec90: u32;
    static mut DAT_1008ec94: u32;
    static mut DAT_1008ec9c: u32;
    static mut DAT_1008eca0: u32;
    static mut DAT_1008eca4: u32;
    static mut DAT_1008eca8: u32;
    static mut DAT_100b0f38: u8;
    static mut DAT_1008ecac: u32;
    static mut DAT_1008ecb0: u32;
    static mut DAT_1008ecb4: u32;
    static mut DAT_1008ecb8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10013420() {
    // SAFETY: the globals are the game's own memory; each is written once
    // in the original's order.
    unsafe {
        DAT_1008ec74 = 0;
        DAT_1008ec78 = 0;
        DAT_100b0f3c = 0;
        DAT_1008ec7c = 0;
        DAT_1008ec80 = 0;
        DAT_1008ec84 = 0;
        DAT_1008ec88 = 0;
        DAT_1008ec8c = 0;
        DAT_1008ec90 = 0;
        DAT_1008ec94 = 0;
        DAT_1008ec9c = 0;
        DAT_1008eca0 = 0;
        DAT_1008eca4 = 0;
        DAT_1008eca8 = 0;
        DAT_100b0f38 = 0;
        DAT_1008ecac = 0;
        DAT_1008ecb0 = 0;
        DAT_1008ecb4 = 0;
        DAT_1008ecb8 = 0;
    }
}
