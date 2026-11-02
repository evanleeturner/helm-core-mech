// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10036142(param_1: i32) -> i32;
    fn FUN_1002ae60(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10036872() {
    // SAFETY: the callees are the game's own functions, called in the original order.
    unsafe {
        let i_var1 = FUN_10036142(4);
        if i_var1 != 0 {
            FUN_1002ae60(4, 2);
        }
    }
}
