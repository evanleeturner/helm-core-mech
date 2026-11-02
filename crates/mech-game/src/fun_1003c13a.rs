// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefa8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003c13a(_param_1: u32, enable: i32) {
    // SAFETY: the global is the game's own memory, written by value.
    unsafe {
        if enable == 0 {
            DAT_100aefa8 = 2;
        } else {
            DAT_100aefa8 = 1;
        }
    }
}
