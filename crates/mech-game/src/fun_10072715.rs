// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10072715(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("10072715", param_1, param_2);
    mech_trace::rt!(
        "10072715",
        if param_1 == 1 {
            ((param_2 >> 1) as u32).wrapping_add(param_2 as u32) as i32
        } else if param_1 == 2 {
            param_2
        } else if param_1 == 3 {
            (param_2 as u32).wrapping_sub((param_2 >> 2) as u32) as i32
        } else {
            param_2
        }
    )
}
