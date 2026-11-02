// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004521e(param_1: *mut i32) -> u32 {
    // SAFETY: param_1 points at the game's object; the pointers read from it
    // are the game's own structures, read at the same offsets as the original.
    unsafe {
        let base = param_1.add(0xf6).read_unaligned();
        let p = param_1.read_unaligned() as usize;
        if base == 0
            || ((((p + 0x4c) as *const i32).read_unaligned() as usize + 0x30) as *const i8).read()
                == 1
            || ((p + 0x1ea) as *const i32).read_unaligned() != 0
            || ((p + 0x1c4) as *const i16).read_unaligned() == 10
        {
            0
        } else {
            1
        }
    }
}
