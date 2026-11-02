// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10003cf8(param_1: i32) -> i32 {
    if param_1 < 0xc351 {
        let mut i_var2: i32 = param_1 / 100;
        i_var2 = (i_var2 as u32).wrapping_mul(i_var2 as u32) as i32;
        i_var2 = (i_var2 as u32).wrapping_mul(100u32) as i32;
        i_var2 /= 250000;
        100u32.wrapping_sub(i_var2 as u32) as i32
    } else {
        0
    }
}
