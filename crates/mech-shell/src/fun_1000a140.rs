// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a140(value: i32) -> i32 {
    mech_trace::enter!("1000a140", value);
    let rem = value % 100;
    mech_trace::rt!("1000a140", value.wrapping_add(rem.wrapping_mul(-1)))
}
