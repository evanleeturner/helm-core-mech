// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf78: u32;
    static mut DAT_100adf7c: u32;
    static DAT_100ae410: [i8; 0];
    static DAT_100ae41c: [i8; 0];
    static DAT_100ae428: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(str1: *const i8, str2: *const i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10037f18(str1: *const i8) {
    // SAFETY: the callee, strings and globals are the game's own; str1 is a
    // NUL-terminated string supplied by the caller.
    unsafe {
        if __strcmpi(str1, (&raw const DAT_100ae410).cast::<i8>()) == 0 {
            DAT_100adf78 = 0x140;
            DAT_100adf7c = 200;
        } else if __strcmpi(str1, (&raw const DAT_100ae41c).cast::<i8>()) == 0 {
            DAT_100adf78 = 0x280;
            DAT_100adf7c = 0x1e0;
        } else if __strcmpi(str1, (&raw const DAT_100ae428).cast::<i8>()) == 0 {
            DAT_100adf78 = 0x400;
            DAT_100adf7c = 0x300;
        } else {
            DAT_100adf78 = 0x140;
            DAT_100adf7c = 200;
        }
    }
}
