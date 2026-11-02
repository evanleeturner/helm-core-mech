// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002b7de(param_1: i32, param_2: *mut i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002b107(param_1: i32) {
    // SAFETY: the callee is the game's own function; a null second
    // argument is what the original passes.
    unsafe {
        FUN_1002b7de(param_1, core::ptr::null_mut());
    }
}
