// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006bbb(param_1: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10006d19(param_1: u32) -> i32 {
    // SAFETY: the callee is the game's own function.
    unsafe {
        let i_var1 = FUN_10006bbb(param_1);
        0x5a0000i32.wrapping_sub(i_var1)
    }
}
