// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004521e(param_1: *mut i32) -> u32 {
    mech_trace::enter!("1004521e", param_1);
    // SAFETY: param_1 points at the game's object; the pointers read from it
    // are the game's own structures, read at the same offsets as the original.
    unsafe {
        let base = mech_trace::rd!("1004521e", param_1.add(0xf6), read_unaligned);
        let p = mech_trace::rd!("1004521e", param_1, read_unaligned) as usize;
        mech_trace::rt!(
            "1004521e",
            if base == 0
                || mech_trace::rd!(
                    "1004521e",
                    ((mech_trace::rd!("1004521e", ((p + 0x4c) as *const i32), read_unaligned)
                        as usize
                        + 0x30) as *const i8),
                    read
                ) == 1
                || mech_trace::rd!("1004521e", ((p + 0x1ea) as *const i32), read_unaligned) != 0
                || mech_trace::rd!("1004521e", ((p + 0x1c4) as *const i16), read_unaligned) == 10
            {
                0
            } else {
                1
            }
        )
    }
}
