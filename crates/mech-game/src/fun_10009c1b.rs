// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10215650: i32;
    static mut DAT_10215658: i32;
    static mut DAT_10215660: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10009c1b(param_1: i32) {
    // SAFETY: the globals are the game's own memory, read and written where
    // the original does.
    unsafe {
        if param_1 != DAT_10215658 {
            if DAT_10215658 < 3 && param_1 < 3 {
                DAT_10215660 = param_1;
            } else if DAT_10215658 < 3 {
                DAT_10215660 = 3;
            } else {
                DAT_10215660 = 5;
                DAT_10215650 = param_1;
            }
        }
    }
}
