// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10064d15(param_1: u32) -> i32 {
    let mut param_1 = param_1;
    let mut bits: i32 = 1;
    let was_zero = param_1 == 0;
    param_1 = param_1.wrapping_sub(1);
    if was_zero {
        bits = 0;
    } else {
        loop {
            param_1 = param_1 >> 1;
            if param_1 == 0 {
                break;
            }
            bits = bits.wrapping_add(1);
        }
    }
    bits
}
