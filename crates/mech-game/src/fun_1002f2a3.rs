// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f2a3(param_1: i32) -> u32 {
    // SAFETY: param_1 is an address of a game object; the field at offset 0x70 is read.
    unsafe { ((param_1 as u32).wrapping_add(0x70) as *const u32).read_unaligned() }
}
