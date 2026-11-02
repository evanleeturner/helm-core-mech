// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001caae(param_1: i32) -> u16 {
    // SAFETY: param_1 is an address of a caller-owned structure; the u16 at
    // offset 0x14 is read as the original does.
    unsafe { (param_1.wrapping_add(0x14) as *const u16).read_unaligned() }
}
