// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d000(param_1: i32) -> i32 {
    mech_trace::enter!("1000d000", param_1);
    mech_trace::rt!("1000d000", (param_1 as u32).wrapping_mul(200) as i32)
}
