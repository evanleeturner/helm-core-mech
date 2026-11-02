// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001ca17(param_1: *const u16) -> u16 {
    // SAFETY: param_1 points at the caller's u16, read where the original does.
    unsafe { param_1.read_unaligned() & 0x7ef0 }
}
