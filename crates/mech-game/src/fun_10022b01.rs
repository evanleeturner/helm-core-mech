// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10022b01(param_1: i32, param_2: u32) {
    // SAFETY: param_1 is the address of a game object whose field at 0x20 holds
    // a pointer to a structure; the write lands at offset 0x3e0 within it.
    unsafe {
        let p = ((param_1 as usize).wrapping_add(0x20) as *const i32).read_unaligned();
        ((p as usize).wrapping_add(0x3e0) as *mut u32).write_unaligned(param_2);
    }
}
