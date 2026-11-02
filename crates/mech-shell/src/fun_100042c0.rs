// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a590(param_1: i32) -> i32;
    fn FUN_1000a3f0(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100042c0(param_1: i32) {
    if 0 < param_1 {
        // SAFETY: the callees are the game's own functions; arguments are passed as the original does.
        unsafe {
            let i_var1 = FUN_1000a590(param_1);
            if i_var1 != 0 {
                FUN_1000a3f0(param_1, i_var1);
            }
        }
    }
}
