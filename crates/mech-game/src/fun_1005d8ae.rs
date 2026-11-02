// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10010683(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005d8ae(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32) {
    // SAFETY: the callee is the game's own function; arguments are forwarded unchanged.
    unsafe {
        FUN_10010683(param_1, param_2, param_3, param_4);
    }
}
