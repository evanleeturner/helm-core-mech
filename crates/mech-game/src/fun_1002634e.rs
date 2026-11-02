// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002637d(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002634e(param_1: i32) -> u32 {
    // SAFETY: param_1 is the address of the game's own structure; the fields
    // are read where the original reads them and the callee is the game's.
    unsafe {
        if ((param_1 as usize).wrapping_add(0x34) as *const i32).read_unaligned() == 0 {
            FUN_1002637d(param_1);
        }
        ((param_1 as usize).wrapping_add(0x30) as *const u32).read_unaligned()
    }
}
