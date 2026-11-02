// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10070f83(param_1: i32, param_2: *mut i8) -> i32;
    fn FUN_10071045(param_1: *mut u32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100710ad(param_1: *mut u32, param_2: *mut i8) -> u32 {
    // SAFETY: the callees are the game's own; the pointers are passed through unchanged.
    unsafe {
        let i_var1 = FUN_10070f83(param_1 as i32, param_2);
        if i_var1 == -1 {
            0xffffffff
        } else {
            FUN_10071045(param_1, i_var1)
        }
    }
}
