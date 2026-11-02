// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001b550(param_1: i32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f6e8(param_1: i32) {
    // SAFETY: param_1 is the address of the game's own structure; the fields
    // read and written are at the offsets the original uses, and the callee
    // is the game's own function.
    unsafe {
        let i_var1 = ((param_1 as usize).wrapping_add(0x20) as *const i32).read_unaligned();
        let p = ((param_1 as usize).wrapping_add(0x18) as *const i32).read_unaligned();
        FUN_1001b550(i_var1, (p as usize).wrapping_add(0x3c) as *mut i32);
        let v = ((param_1 as usize).wrapping_add(0x50) as *const u32).read_unaligned();
        ((i_var1 as usize).wrapping_add(0x10) as *mut u32).write_unaligned(v);
    }
}
