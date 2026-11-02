// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083710: u32;
    static mut DAT_10083744: u32;
    static mut DAT_10083748: u32;
    static mut DAT_10083790: u32;
    static mut DAT_10083798: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10004620() -> u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a180(param_1: i32) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10004760() {
    // SAFETY: the callees and the globals are the game's own memory; they are
    // read and written in the same order as the original.
    unsafe {
        FUN_10004620();
        if DAT_10083790 < DAT_10083798 {
            loop {
                FUN_1000a180(DAT_10083798.wrapping_add(0x2134) as i32);
                DAT_10083798 = DAT_10083798.wrapping_sub(1);
                if !(DAT_10083790 < DAT_10083798) {
                    break;
                }
            }
        }
        if DAT_10083710 < 0x157d {
            DAT_10083744 = 0x32;
        } else {
            DAT_10083744 = 100;
            if 0x2134 < DAT_10083710 {
                DAT_10083744 = 200;
            }
        }
        DAT_10083748 = DAT_10083798.wrapping_mul(DAT_10083744);
    }
}
