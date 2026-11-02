// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063f34(param_1: i32) -> u32 {
    // SAFETY: param_1 is the address of a caller-owned structure; the u32 at
    // offset 0xa0 is read where the original does.
    unsafe { ((param_1.wrapping_add(0xa0)) as *const u32).read_unaligned() }
}
