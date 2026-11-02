// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10049844(param_1: i32) -> bool {
    // SAFETY: param_1 is an address of a game structure; the i32 at offset 0xc is read.
    unsafe { ((param_1 as usize).wrapping_add(0xc) as *const i32).read_unaligned() == 0 }
}
