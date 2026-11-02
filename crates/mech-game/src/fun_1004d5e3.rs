// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_101600f4: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d5e3(param_1: i32) -> u32 {
    mech_trace::enter!("1004d5e3", param_1);
    // SAFETY: both globals are the game's own tables; the pointer read from
    // the first table points at a record whose field at +8 indexes the second.
    unsafe {
        let p = mech_trace::rd!(
            "1004d5e3",
            (&raw mut DAT_100eef40)
                .cast::<u32>()
                .offset(param_1 as isize),
            read_unaligned
        );
        let idx = mech_trace::rd!(
            "1004d5e3",
            (p as usize as *const u8).byte_add(8).cast::<i32>(),
            read_unaligned
        );
        mech_trace::rt!(
            "1004d5e3",
            mech_trace::rd!(
                "1004d5e3",
                (&raw mut DAT_101600f4)
                    .cast::<u8>()
                    .byte_offset(idx.wrapping_mul(0x3c) as isize)
                    .cast::<u32>(),
                read_unaligned
            )
        )
    }
}
