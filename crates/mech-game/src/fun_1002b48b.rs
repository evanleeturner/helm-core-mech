// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002b48b(param_1: i32) -> i32 {
    mech_trace::enter!("1002b48b", param_1);
    mech_trace::rt!(
        "1002b48b",
        if param_1 == 1 {
            2
        } else if param_1 == 2 {
            1
        } else {
            0
        }
    )
}
