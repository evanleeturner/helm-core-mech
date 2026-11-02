// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10026183(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c1a3(param_1: i32) -> i32 {
    if param_1 == 0 {
        return 0;
    }
    // SAFETY: param_1 is an address of the game's own structure; the fields
    // are read where the original reads them, and the callee is the game's.
    unsafe {
        if ((param_1.wrapping_add(0x4c)) as *const i32).read_unaligned() == 0 {
            0
        } else if ((param_1.wrapping_add(0x24)) as *const i32).read_unaligned() == 0 {
            0x78
        } else if ((param_1.wrapping_add(0x24)) as *const i32).read_unaligned() == 5 {
            FUN_10026183(((param_1.wrapping_add(0x4c)) as *const i32).read_unaligned())
        } else {
            0
        }
    }
}
