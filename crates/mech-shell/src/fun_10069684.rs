// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10069684(param_1: i32) -> u32 {
    // SAFETY: param_1 is an address the caller guarantees readable at offset 8.
    unsafe { ((param_1 as usize).wrapping_add(8) as *const u32).read_unaligned() }
}
