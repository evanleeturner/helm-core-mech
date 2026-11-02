// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a267(param_1: i32) -> u32 {
    // SAFETY: param_1 is the address of the caller's SHP data; the u32 at
    // offset 4 is read as the original does.
    unsafe { ((param_1 as usize).wrapping_add(4) as *const u32).read_unaligned() }
}
