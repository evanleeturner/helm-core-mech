// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a46f8: i32;
    static mut DAT_100a46fc: i32;
    static mut DAT_100a4700: i32;
    static mut DAT_100a4704: i32;
    static mut DAT_100a4708: i32;
    static mut DAT_100a470c: i32;
    static mut DAT_100a4710: i32;
    static mut DAT_100a4714: i32;
    static mut DAT_100a4718: i32;
    static mut DAT_100a471c: u8;
    static mut DAT_100a4720: i32;
    static mut DAT_100a4724: i32;
    static mut DAT_100a4728: i32;
    static mut DAT_100a472c: i32;
    static mut DAT_100a4730: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10057a40() {
    // SAFETY: the globals are the game's own memory; each is written once, in the original's order.
    unsafe {
        DAT_100a46f8 = 0;
        DAT_100a46fc = 0;
        DAT_100a4700 = 0;
        DAT_100a4704 = 0;
        DAT_100a4708 = 0;
        DAT_100a470c = 0;
        DAT_100a4710 = 0;
        DAT_100a4714 = 0;
        DAT_100a4718 = 0;
        DAT_100a471c = 0;
        DAT_100a4720 = 0;
        DAT_100a4724 = 0;
        DAT_100a4728 = 0;
        DAT_100a472c = 0;
        DAT_100a4730 = 0;
    }
}
