// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002fc2b(param_1: *mut u32) -> u32 {
    // SAFETY: param_1 points at a readable u32 supplied by the caller.
    unsafe { param_1.read_unaligned() }
}
