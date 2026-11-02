// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: u32;
    static mut DAT_100b38fc: u32;
    static mut DAT_100b3904: u32;
    static mut DAT_100b3908: u32;
    static mut DAT_100b390c: u32;
    static mut DAT_100b3914: u32;
    static mut DAT_100b3918: u32;
    static mut DAT_100b391c: u32;
    static mut DAT_100b3920: u32;
    static mut DAT_100b3930: u32;
    static mut DAT_100b3934: u32;
    static mut DAT_100b3938: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10036a37(param_1: u32) -> i32;
    fn FUN_10036b20(param_1: u16, param_2: i16) -> u8;
    fn FUN_10036aad(param_1: u32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10065b2c() {
    // SAFETY: the globals and callees are the game's own memory and code;
    // accessed in the same order as the original.
    unsafe {
        if DAT_100b3938 != 0 {
            if DAT_100b3908 == 0 {
                DAT_100b38fc = FUN_10036a37(DAT_100b3920) as u32;
                if 0 < (DAT_100b391c as i32) {
                    while (DAT_100b38fc.wrapping_sub(DAT_100b3930) as i32) < (DAT_100b391c as i32) {
                        DAT_100b38fc = FUN_10036a37(DAT_100b3920) as u32;
                    }
                }
                if DAT_100b390c != 0 {
                    DAT_100b3934 = DAT_100b3908;
                    DAT_100b3908 = 3;
                    FUN_10036b20(0x80, 1);
                }
            } else if DAT_100b3908 == 3 {
                DAT_100b38fc = DAT_100b38fc.wrapping_add(0xc);
                if DAT_100b390c == 0 {
                    DAT_100b3908 = DAT_100b3934;
                    FUN_10036b20(0x80, 0);
                    FUN_10036aad(DAT_100b3920, DAT_100b38fc as i32);
                }
            }
            DAT_100b3904 = DAT_100b38fc.wrapping_sub(DAT_100b3930);
            if DAT_100aa1ac == 0 {
                if DAT_100b3914 == 0 {
                    if DAT_100b3918 != 0 {
                        DAT_100b3904 = ((DAT_100b3904 as i32) >> 2) as u32;
                        DAT_100b38fc = DAT_100b38fc.wrapping_add(DAT_100b3904);
                        FUN_10036aad(DAT_100b3920, DAT_100b38fc as i32);
                    }
                } else {
                    DAT_100b3904 = DAT_100b3904.wrapping_mul(8);
                    DAT_100b38fc = DAT_100b38fc.wrapping_add(DAT_100b3904);
                    FUN_10036aad(DAT_100b3920, DAT_100b38fc as i32);
                }
            }
            if (DAT_100b3904 as i32) < 1 {
                DAT_100b3904 = 0;
                DAT_100b38fc = DAT_100b3930;
            }
            DAT_100b3930 = DAT_100b38fc;
        }
    }
}
