// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007cd68(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007d08d(param_1: i32) {
    // SAFETY: the callee is the game's own function; it is called with the
    // same argument the original passes.
    unsafe {
        FUN_1007cd68(param_1);
    }
}
