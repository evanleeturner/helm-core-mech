// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7848: i32;
    static mut DAT_100a7f18: i32;
    static mut DAT_100a7f1c: i32;
    static mut DAT_100a7f20: i32;
    static mut DAT_100a7f24: i32;
    static mut DAT_100a7f28: i32;
    static mut DAT_100a7f2c: i32;
    static mut DAT_100a7f30: i32;
    static mut DAT_100a7f34: i32;
    static mut DAT_100a7f38: u8;
    static mut DAT_100a7f3c: i32;
    static mut DAT_100a7f40: i32;
    static mut DAT_100a7f44: i32;
    static mut DAT_100a7f48: i32;
    static mut DAT_100a7f4c: i32;
    static mut DAT_100a7f50: i32;
    static mut DAT_100a784c: i32;
    static mut DAT_100a7850: i32;
    static mut DAT_100a7854: i32;
    static mut DAT_100c4ae8: i32;
    static mut DAT_100c4adc: i32;
    static mut DAT_100c4af4: i32;
    static mut DAT_100a7858: i32;
    static mut DAT_100a785c: i32;
    static mut DAT_100a7860: i32;
    static mut DAT_100c4ad8: i32;
    static mut DAT_100a7864: i32;
    static mut DAT_100a7868: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10060bc0() {
    // SAFETY: the globals are the game's own memory; each is written once
    // in the original's order.
    unsafe {
        DAT_100a7848 = 0;
        DAT_100a7f18 = 0;
        DAT_100a7f1c = 0;
        DAT_100a7f20 = 0;
        DAT_100a7f24 = 0;
        DAT_100a7f28 = 0;
        DAT_100a7f2c = 0;
        DAT_100a7f30 = 0;
        DAT_100a7f34 = 0;
        DAT_100a7f38 = 0;
        DAT_100a7f3c = 0;
        DAT_100a7f40 = 0;
        DAT_100a7f44 = 0;
        DAT_100a7f48 = 0;
        DAT_100a7f4c = 0;
        DAT_100a7f50 = 0;
        DAT_100a784c = 0;
        DAT_100a7850 = 0;
        DAT_100a7854 = 0;
        DAT_100c4ae8 = 0;
        DAT_100c4adc = 0;
        DAT_100c4af4 = 0;
        DAT_100a7858 = 0;
        DAT_100a785c = 0;
        DAT_100a7860 = 0;
        DAT_100c4ad8 = 0;
        DAT_100a7864 = 0;
        DAT_100a7868 = 0;
    }
}
