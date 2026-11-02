// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b36d4: u32;
    static mut DAT_100b36d8: u32;
    static mut DAT_100b36e4: u32;
    static mut DAT_100b36e8: u32;
    static mut DAT_100b36ec: u32;
    static mut DAT_100b36f0: u32;
    static mut DAT_100b36f4: u32;
    static mut DAT_100c7e40: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10071a19(param_1: *mut i32);
    fn FUN_1006065d(param_1: i32, param_2: u32, param_3: i32) -> u32;
    fn FUN_100717ee(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100605d6() {
    // SAFETY: the globals and callees are the game's own memory and code;
    // accesses occur in the original's order.
    unsafe {
        if DAT_100b36f0 != 0 {
            FUN_10071a19((&raw mut DAT_100c7e40).cast::<i32>());
            DAT_100b36f0 = DAT_100b36f0.wrapping_sub(1);
            if DAT_100b36f0 == 0 {
                DAT_100b36d4 = DAT_100b36e4;
                if DAT_100b36ec == 0 {
                    DAT_100b36d8 = 1;
                } else {
                    FUN_1006065d(DAT_100b36e8 as i32, DAT_100b36ec, 0);
                }
            }
        }
        if DAT_100b36f4 != 0 {
            FUN_100717ee(0x100c7e60);
        }
    }
}
