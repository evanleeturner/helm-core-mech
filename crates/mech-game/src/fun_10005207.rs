// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10005228(param_1: i32, param_2: *mut i32, param_3: u32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10005207(param_1: i32, param_2: *mut i32, param_3: u32) {
    // SAFETY: the callee is the game's own function; arguments are passed through unchanged.
    unsafe {
        FUN_10005228(param_1, param_2, param_3, 0);
    }
}
