// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005db1f(param_1: i32) -> i32 {
    mech_trace::enter!("1005db1f", param_1);
    mech_trace::rt!("1005db1f", param_1.wrapping_add(4))
}
