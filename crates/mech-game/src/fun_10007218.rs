// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a41ec: *mut c_void;
    static mut DAT_100a41f0: [i8; 0];
    static mut DAT_100a4344: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_fopen(path: *const i8, mode: *const i8) -> *mut c_void;
    fn FUN_fputs(s: *const i8, f: *mut c_void) -> i32;
    fn FUN_fflush(f: *mut c_void) -> i32;
    fn FUN_fclose(f: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10007218(str: *mut i8) {
    // SAFETY: the globals and the C runtime functions are the game's own;
    // str points at the caller's NUL-terminated string.
    unsafe {
        if DAT_100a41ec.is_null() {
            DAT_100a41ec = FUN_fopen(
                (&raw mut DAT_100a41f0).cast::<i8>(),
                (&raw mut DAT_100a4344).cast::<i8>(),
            );
        }
        if !DAT_100a41ec.is_null() {
            FUN_fputs(str, DAT_100a41ec);
            FUN_fflush(DAT_100a41ec);
            FUN_fclose(DAT_100a41ec);
            DAT_100a41ec = core::ptr::null_mut();
        }
    }
}
