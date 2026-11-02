// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b67f0: u8;
    static mut DAT_100af114: *mut u8;
    static mut DAT_100af11c: *mut u8;
    static mut DAT_100af120: *mut u8;
    static mut DAT_100af134: *mut u8;
    static mut DAT_100af13c: *mut u8;
    static mut DAT_100af144: *mut u8;
    static mut DAT_100af148: *mut u8;
    static mut DAT_100af14c: *mut u8;
    static mut DAT_100af150: *mut u8;
    static mut DAT_100af15c: *mut u8;
    static mut DAT_100af164: *mut u8;
    static mut DAT_100af168: *mut u8;
    static mut DAT_100af16c: *mut u8;
    static mut DAT_100af174: *mut u8;
    static mut DAT_100af178: *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007bac8(param_1: u32) -> *mut u8 {
    // SAFETY: the globals are the game's own pointer table and data array;
    // they are read by value or addressed without creating references.
    unsafe {
        match param_1 {
            1 => DAT_100af15c,
            3 => DAT_100af144,
            4 => DAT_100af134,
            5 => DAT_100af150,
            6 => DAT_100af14c,
            7 => DAT_100af148,
            8 => DAT_100af164,
            10 => DAT_100af114,
            0xb => DAT_100af168,
            0xc => DAT_100af120,
            0xd => DAT_100af13c,
            0xe => DAT_100af11c,
            0xf => DAT_100af16c,
            0x11 => DAT_100af174,
            0x14 => DAT_100af178,
            _ => &raw mut DAT_100b67f0,
        }
    }
}
