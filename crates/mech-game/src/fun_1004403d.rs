// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004403d(param_1: i32, param_2: u8) {
    // SAFETY: param_1 points at the game's object whose field at 0x4c holds
    // a pointer to a structure; the byte at offset 0x1d is written.
    unsafe {
        let p = ((param_1 as usize).wrapping_add(0x4c) as *const i32).read_unaligned();
        ((p as usize).wrapping_add(0x1d) as *mut u8).write(param_2);
    }
}
