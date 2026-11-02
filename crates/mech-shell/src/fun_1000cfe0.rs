// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000cfe0(value: i32) -> i32 {
    mech_trace::enter!("1000cfe0", value);
    mech_trace::rt!("1000cfe0", ((value as u32).wrapping_mul(0x271)) as i32)
}
