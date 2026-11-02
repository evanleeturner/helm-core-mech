// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c40: u32;
    static mut DAT_100b3c44: u32;
    static mut DAT_100b3c58: u8;
    static mut DAT_100b3ca4: u32;
    static mut DAT_100b3cb4: [i8; 1];
    static mut DAT_10165940: u32;
    static mut DAT_100af104: u32;
    static mut DAT_100b3cac: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn sscanf(src: *const i8, format: *const i8, ...) -> i32;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
    fn FUN_100108a0(param_1: u32, param_2: *const i8) -> u8;
    fn FUN_10046fc9(param_1: i32) -> u32;
    fn FUN_10046b4d(param_1: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10046fb4() -> u32;
    fn FUN_10046563() -> u32;
    fn FUN_1004663a() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006aa37() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // accesses happen in the same order as the original.
    unsafe {
        if DAT_100b3ca4 == 0 {
            DAT_100b3ca4 = 1;
            if (DAT_10165940 as i32) < 1 {
                DAT_100b3ca4 = 1;
                return;
            }
            let src = FUN_1008668b(0, DAT_10165940, DAT_100af104 as *mut i32);
            if src.is_null() {
                return;
            }
            sscanf(
                src as *const i8,
                (&raw mut DAT_100b3cac) as *const i8,
                &raw mut DAT_100b3c40,
                &raw mut DAT_100b3c44,
            );
            FUN_1008664f(DAT_10165940, DAT_100af104 as *mut i32);
        }
        let mut i_var1 = FUN_10046fb4();
        if i_var1 == 0 {
            DAT_100b3c40 = 0xffff_ffff;
        } else {
            i_var1 = FUN_10046563();
            if i_var1 == 1 {
                DAT_100b3c40 = 0xffff_ffff;
                FUN_100108a0(0x53, (&raw mut DAT_100b3cb4) as *const i8);
            } else {
                i_var1 = FUN_10046fc9(DAT_100b3c40 as i32);
                if i_var1 == 0 {
                    DAT_100b3c40 = 0xffff_ffff;
                } else if (DAT_100b3c58 & 8) != 0 {
                    FUN_10046b4d(DAT_100b3c40 as i32);
                    i_var1 = FUN_1004663a();
                    if i_var1 != 3 {
                        DAT_100b3c40 = 0xffff_ffff;
                    }
                }
            }
        }
        DAT_100b3c44 = 0xffff_ffff;
    }
}
