// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006b002(param_1: i32, param_2: u32, param_3: *mut u32, param_4: i32, param_5: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006afdd(param_1: i32, param_2: u32, param_3: *mut u32, param_4: i32) {
    // SAFETY: the callee is the game's own function; the arguments are passed through unchanged.
    unsafe {
        FUN_1006b002(param_1, param_2, param_3, param_4, core::ptr::null_mut());
    }
}
