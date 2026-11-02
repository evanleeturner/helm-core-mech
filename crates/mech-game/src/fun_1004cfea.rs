// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101600f8: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004cfea(param_1: i32) -> u32 {
    mech_trace::enter!("1004cfea", param_1);
    mech_trace::rt!(
        "1004cfea",
        if param_1 < 0x10 {
            // SAFETY: DAT_101600f8 is the game's own table; the entry is read
            // at the same byte offset the original uses.
            unsafe {
                let off = param_1.wrapping_mul(0x3c);
                mech_trace::rd!(
                    "1004cfea",
                    (&raw mut DAT_101600f8)
                        .byte_add(off as isize as usize)
                        .cast::<u32>(),
                    read_unaligned
                )
            }
        } else {
            0
        }
    )
}
