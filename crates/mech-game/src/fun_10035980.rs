// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10035980(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) -> i32 {
    let a = (param_1 as i64).wrapping_mul(param_2 as i64) as u64;
    let b = (param_3 as i64).wrapping_mul(param_4 as i64) as u64;
    let c = (param_6 as i64).wrapping_mul(param_5 as i64) as u64;
    let dot_sum = a.wrapping_add(b).wrapping_add(c);

    let low = dot_sum as u32;
    let high = (dot_sum >> 32) as u32;

    let rounded = (low.wrapping_shr(0x1d) | high.wrapping_mul(8))
        .wrapping_add(((low.wrapping_shr(0x1c)) & 1 != 0) as u32);
    rounded as i32
}
