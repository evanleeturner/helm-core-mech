// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c537(param_1: i32) -> u32 {
    // SAFETY: param_1 is the address of a game structure; the pointer at
    // offset 0x20 is read, and dereferenced only when non-null.
    unsafe {
        let p = ((param_1 as usize).wrapping_add(0x20) as *const *const u32).read_unaligned();
        if p.is_null() { 0 } else { p.read_unaligned() }
    }
}
