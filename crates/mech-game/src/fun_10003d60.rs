// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10003d60(param_1: i32, param_2: i32) -> i32 {
    let product: i64 = (param_1 as i64).wrapping_mul(param_2 as i64);
    let product_low: u32 = product as u32;
    let product_high: u32 = ((product as u64) >> 32) as u32;

    let mut rounded: u32 = (product_low >> 16) | product_high.wrapping_shl(16);
    rounded = rounded.wrapping_add(((product_low >> 15) & 1u32 != 0) as u32);
    rounded as i32
}
