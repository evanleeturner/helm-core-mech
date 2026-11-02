// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003c0e4(param_1: *const u16) -> i32 {
    mech_trace::enter!("1003c0e4", param_1);
    // SAFETY: param_1 points at a readable u16 supplied by the caller.
    let v = unsafe { mech_trace::rd!("1003c0e4", param_1, read_unaligned) };
    mech_trace::rt!("1003c0e4", ((v & 0x1000) != 0) as i32)
}
