// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a120(value: i32) -> i32 {
    let rem = value % 100;
    if rem != 0 {
        return (value as u32).wrapping_sub(rem as u32).wrapping_add(100) as i32;
    }
    value
}
