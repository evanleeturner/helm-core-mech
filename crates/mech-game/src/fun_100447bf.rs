// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100447bf(param_1: i32, limit: i32) -> i32 {
    let mut param_1 = param_1;
    if limit < param_1 {
        param_1 = limit;
    } else if (param_1 as u32).wrapping_neg() as i32 != limit
        && param_1 <= (limit as u32).wrapping_neg() as i32
    {
        param_1 = (limit as u32).wrapping_neg() as i32;
    }
    param_1
}
