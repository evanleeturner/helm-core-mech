// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003658(param_1: u32, param_2: u32, param_3: u32, param_4: u32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100037f0(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: i32,
) {
    // SAFETY: the callee is the game's own function; arguments are passed through unchanged.
    unsafe {
        FUN_10003658(param_1, param_2, param_3, param_4, param_5);
    }
}
