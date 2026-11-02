// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3998: u32;
    static mut DAT_100b399c: u32;
    static mut DAT_100b39a0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10068d4e(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: the globals are the game's own memory, written in the original's order.
    unsafe {
        DAT_100b3998 = param_1 as u32;
        if param_1 == 0 {
            DAT_100b3998 = 1;
        }
        DAT_100b399c = param_2 as u32;
        if param_2 == 0 {
            DAT_100b399c = 1;
        }
        DAT_100b39a0 = param_3 as u32;
        if param_3 == 0 {
            DAT_100b39a0 = 1;
        }
    }
}
