// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ae750: u32;
    static mut DAT_100ae758: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f783(param_1: i32) -> u8;
    fn FUN_1002f727(param_1: i32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10039649(param_1: i32) {
    // SAFETY: the globals and callees are the game's own; the pointer in
    // DAT_100ae758 is checked for null before use.
    unsafe {
        if !DAT_100ae758.is_null() {
            if param_1 == 0 {
                FUN_1002f727(DAT_100ae758 as i32);
            } else {
                FUN_1002f783(DAT_100ae758 as i32);
            }
            FUN_1002fbc0(DAT_100ae758);
            DAT_100ae750 = param_1 as u32;
        }
    }
}
