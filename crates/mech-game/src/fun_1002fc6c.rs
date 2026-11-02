// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002fc6c(param_1: i32) -> i32 {
    mech_trace::enter!("1002fc6c", param_1);
    mech_trace::rt!("1002fc6c", (param_1 as u32).wrapping_add(0xc) as i32)
}
