// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a41ec: *mut c_void;
    static mut DAT_100a41f0: i8;
    static mut DAT_100a4340: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fopen(param_1: *const i8, param_2: *const i8) -> *mut c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100071f6() {
    // SAFETY: the callee and the globals are the game's own memory; the two
    // string buffers are passed by address as the original does.
    unsafe {
        DAT_100a41ec = _fopen(
            (&raw mut DAT_100a41f0).cast_const(),
            (&raw mut DAT_100a4340).cast_const(),
        );
    }
}
