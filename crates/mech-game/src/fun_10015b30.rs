// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10015b30(param_1: i32, param_2: i32) -> i32 {
    let product: u64 = (param_1 as i64 as u64).wrapping_mul(param_2 as i64 as u64);
    let product_low: u32 = product as u32;
    let product_high: u32 = (product >> 32) as u32;
    ((product_low >> 0x1d) | product_high.wrapping_shl(3))
        .wrapping_add(((product_low >> 0x1c) & 1 != 0) as u32) as i32
}
