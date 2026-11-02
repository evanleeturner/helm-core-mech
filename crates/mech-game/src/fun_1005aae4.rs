// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005aae4(param_1: i32) -> u32 {
    // SAFETY: param_1 is an address the caller supplies; the u32 at offset 8 is read as the original does.
    unsafe { ((param_1 as u32).wrapping_add(8) as *const u32).read_unaligned() }
}
