// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_1002f3a7(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
    fn FUN_1002f727(param_1: i32) -> u8;
    fn FUN_100399ec(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003ad1e(param_1: i32) -> bool {
    mech_trace::enter!("1003ad1e", param_1);
    let base = param_1 as usize as *mut u8;
    // SAFETY: param_1 points at the game's object; the offsets read and written
    // are those of the original, and the callees and globals are the game's own.
    unsafe {
        let local_c = mech_trace::rd!(
            "1003ad1e",
            base.byte_add(0x1f6).cast::<*mut i32>(),
            read_unaligned
        );
        let mut local_10: u32 = 0;
        let mut local_14: i32 = 0;
        let mut local_18: u32 = 0;
        mech_trace::cr!(
            "1003ad1e",
            "1002f322",
            FUN_1002f322(
                mech_trace::rd!(
                    "1003ad1e",
                    base.byte_add(0x1d2).cast::<i32>(),
                    read_unaligned
                ),
                &raw mut local_10,
                (&raw mut local_14).cast::<u32>(),
                &raw mut local_18,
            )
        );
        if local_14 < 1 {
            let p = base.byte_add(0x10).cast::<u16>();
            mech_trace::wr!(
                "1003ad1e",
                p,
                write_unaligned,
                mech_trace::rd!("1003ad1e", p, read_unaligned) & 0xfffe
            );
            mech_trace::cr!(
                "1003ad1e",
                "1002f727",
                FUN_1002f727(mech_trace::rd!(
                    "1003ad1e",
                    base.byte_add(0x1d2).cast::<i32>(),
                    read_unaligned
                ))
            );
            mech_trace::cr!(
                "1003ad1e",
                "100399ec",
                FUN_100399ec(
                    mech_trace::rd!("1003ad1e", local_c.add(8), read_unaligned),
                    mech_trace::rd!("1003ad1e", local_c, read_unaligned)
                )
            );
        } else {
            let scale = mech_trace::gr!("1003ad1e", "100b3904", DAT_100b3904) as i32;
            let i_var2 = mech_trace::rd!("1003ad1e", local_c.add(5), read_unaligned)
                .wrapping_add(mech_trace::rd!(
                    "1003ad1e",
                    base.byte_add(0x1ea).cast::<i32>(),
                    read_unaligned
                ))
                .wrapping_mul(scale);
            let i_var1 = mech_trace::rd!(
                "1003ad1e",
                base.byte_add(0x1ee).cast::<i32>(),
                read_unaligned
            );
            let i_var3 = mech_trace::cr!("1003ad1e", "10051f43", FUN_10051f43(10));
            let local_8 = mech_trace::rd!(
                "1003ad1e",
                base.byte_add(0x1f2).cast::<i32>(),
                read_unaligned
            )
            .wrapping_add(mech_trace::rd!("1003ad1e", local_c.add(7), read_unaligned))
            .wrapping_mul(scale)
                / 0xb5;
            let third = i_var1
                .wrapping_sub(i_var3)
                .wrapping_add(mech_trace::rd!("1003ad1e", local_c.add(6), read_unaligned))
                .wrapping_mul(scale)
                / 0xb5;
            mech_trace::cr!(
                "1003ad1e",
                "1002f3a7",
                FUN_1002f3a7(
                    mech_trace::rd!(
                        "1003ad1e",
                        base.byte_add(0x1d2).cast::<i32>(),
                        read_unaligned
                    ),
                    i_var2 / 0xb5,
                    third,
                    local_8,
                )
            );
            mech_trace::cr!(
                "1003ad1e",
                "1002fbc0",
                FUN_1002fbc0(mech_trace::rd!(
                    "1003ad1e",
                    base.byte_add(0x1d2).cast::<*mut i32>(),
                    read_unaligned
                ))
            );
            mech_trace::wr!(
                "1003ad1e",
                base.byte_add(0x1e6).cast::<u32>(),
                write_unaligned,
                0xdf
            );
        }
        mech_trace::rt!("1003ad1e", 0 < local_14)
    }
}
