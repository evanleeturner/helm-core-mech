// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d5e3(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d69b(param_1: i32, param_2: i32) -> i32 {
    // SAFETY: the callee is the game's own function; calls keep the original order.
    unsafe {
        let i_var1 = FUN_1004d5e3(param_2);
        let i_var2 = FUN_1004d5e3(param_1);
        (i_var1 == i_var2) as i32
    }
}
