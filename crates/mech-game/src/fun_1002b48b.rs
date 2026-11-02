// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002b48b(param_1: i32) -> i32 {
    if param_1 == 1 {
        2
    } else if param_1 == 2 {
        1
    } else {
        0
    }
}
