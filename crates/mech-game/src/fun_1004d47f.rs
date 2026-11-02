// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101600e0: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d47f(param_1: i32) -> u32 {
    mech_trace::enter!("1004d47f", param_1);
    let mut local_8: u32 = 0;
    if param_1 < 0x10 {
        // SAFETY: the table is the game's own memory; the entry's first
        // dword is read at the computed byte offset.
        unsafe {
            local_8 = mech_trace::rd!(
                "1004d47f",
                (&raw mut DAT_101600e0)
                    .cast::<u8>()
                    .byte_offset(param_1.wrapping_mul(0x3c) as isize)
                    .cast::<u32>(),
                read_unaligned
            );
        }
    }
    mech_trace::rt!("1004d47f", local_8)
}
