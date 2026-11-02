// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001a080(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) -> i32 {
    let mut dot_sum: u64 = (param_1 as i64 as u64).wrapping_mul(param_2 as i64 as u64);
    dot_sum = dot_sum.wrapping_add((param_3 as i64 as u64).wrapping_mul(param_4 as i64 as u64));
    dot_sum = dot_sum.wrapping_add((param_6 as i64 as u64).wrapping_mul(param_5 as i64 as u64));

    let low = dot_sum as u32;
    let high = (dot_sum >> 32) as u32;

    ((low >> 0x1b) | high.wrapping_shl(5)).wrapping_add(((low >> 0x1a) & 1 != 0) as u32) as i32
}
