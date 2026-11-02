// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: u32;
    static mut DAT_100b3c40: u32;
    static mut DAT_100b3c58: u8;
    static mut DAT_100b3ca0: u32;
    static mut DAT_100b3ca8: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10046928() -> u32;
    fn FUN_1004663a() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10046b4d(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006ac6a() {
    // SAFETY: the globals and callees are the game's own; accessed in the
    // same order as the original.
    unsafe {
        DAT_100b3ca8 = DAT_100b3ca8.wrapping_add(DAT_100b3904);
        if 0x169 < DAT_100b3ca8 {
            DAT_100b3ca8 = 0;
            if DAT_100b3ca0 == 0 && (DAT_100b3c58 & 8) != 0 {
                let i_var1 = FUN_10046928();
                if i_var1 != 0 && DAT_100b3c40 != 0xFFFF_FFFF {
                    FUN_10046b4d(DAT_100b3c40 as i32);
                    FUN_1004663a();
                }
            }
        }
    }
}
