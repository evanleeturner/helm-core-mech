// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003c190(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    let a = (param_1 as i64 as u64).wrapping_mul(param_1 as i64 as u64);
    let b = (param_2 as i64 as u64).wrapping_mul(param_2 as i64 as u64);
    let c = (param_3 as i64 as u64).wrapping_mul(param_3 as i64 as u64);
    let d = (param_4 as i64 as u64).wrapping_mul(param_4 as i64 as u64);

    let sum = a.wrapping_add(b).wrapping_add(c);

    if sum < d {
        return 1;
    }
    if (sum as u32) == (d as u32) {
        return 1;
    }
    0
}
