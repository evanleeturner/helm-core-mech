// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10050b20(this: *mut c_void, method_object: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000c4a0(param_1: *mut c_void, param_2: i32, param_3: *mut i8) -> i32 {
    // SAFETY: the callee is the game's own function; the arguments are passed
    // through unchanged.
    unsafe {
        let method_result = FUN_10050b20(param_1, param_3);
        param_2.wrapping_sub(method_result)
    }
}
