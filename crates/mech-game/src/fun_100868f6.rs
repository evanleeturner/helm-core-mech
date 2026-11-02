// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100868f6(param_1: u32, param_2: *mut i32) {
    // SAFETY: the callee is the game's own function; arguments are passed through unchanged.
    unsafe {
        FUN_1008668b(0, param_1, param_2);
    }
}
