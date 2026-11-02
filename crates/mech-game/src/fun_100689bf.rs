// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10114ec8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005d8ae(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100689bf(param_1: i32) {
    mech_trace::enter!("100689bf", param_1);
    // SAFETY: param_1 is the address of the caller's object; fields are read
    // and written at the original's offsets, and the callee and global are the game's own.
    unsafe {
        let base = param_1 as *mut u8;
        if mech_trace::rd!("100689bf", base.byte_add(4).cast::<i16>(), read_unaligned) != 0
            && mech_trace::rd!(
                "100689bf",
                base.byte_add(0x30).cast::<i32>(),
                read_unaligned
            ) != 0
        {
            mech_trace::wr!(
                "100689bf",
                base.byte_add(0xb0).cast::<u32>(),
                write_unaligned,
                mech_trace::gr!("100689bf", "10114ec8", DAT_10114ec8)
            );
            mech_trace::cr!(
                "100689bf",
                "1005d8ae",
                FUN_1005d8ae(
                    mech_trace::rd!(
                        "100689bf",
                        base.byte_add(0xa4).cast::<*mut i32>(),
                        read_unaligned
                    ),
                    0,
                    0,
                    0,
                )
            );
        }
    }
}
