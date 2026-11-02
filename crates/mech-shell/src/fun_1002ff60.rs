// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009200c: u32;
    static mut DAT_10092010: u32;
    static mut DAT_100a67b8: *mut i32;
    static mut DAT_10093260: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000be30(param_1: *mut u32);
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_1005b1b0(param_1: *const i8);
    fn FUN_1000c770(param_1: *mut i32, param_2: i32);
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
    fn FUN_1000c6b0(param_1: i32, param_2: i32) -> u32;
    fn FUN_10017ec0(param_1: i32, param_2: i32, param_3: i8);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10030090();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002ff60(param_1: *mut u32, param_2: *mut u32) {
    mech_trace::enter!("1002ff60", param_1, param_2);
    // SAFETY: the pointers are the caller's record buffers; globals and callees
    // are the game's own, used in the original's order.
    unsafe {
        let u_var2 = mech_trace::rd!("1002ff60", param_1.add(0xd9), read_unaligned);
        let mut i_var6: i32 = 0;
        let mut puvar7 = param_1;
        let mut src = param_2 as *const u32;
        let mut i_var5: i32 = 0x214;
        while i_var5 != 0 {
            mech_trace::wr!(
                "1002ff60",
                puvar7,
                write_unaligned,
                mech_trace::rd!("1002ff60", src, read_unaligned)
            );
            src = src.add(1);
            puvar7 = puvar7.add(1);
            i_var5 = i_var5.wrapping_sub(1);
        }
        mech_trace::wr!(
            "1002ff60",
            puvar7.cast::<u16>(),
            write_unaligned,
            mech_trace::rd!("1002ff60", (src as *const u16), read_unaligned)
        );
        mech_trace::wr!(
            "1002ff60",
            puvar7.cast::<u8>().byte_add(2),
            write,
            mech_trace::rd!("1002ff60", (src as *const u8).byte_add(2), read)
        );
        mech_trace::wr!("1002ff60", param_1.add(0xd9), write_unaligned, u_var2);
        mech_trace::wr!("1002ff60", param_1.add(0x213).cast::<u8>(), write, 0);
        FUN_1000be30(param_1);

        if 0 < mech_trace::rd!(
            "1002ff60",
            ((mech_trace::gr!("1002ff60", "1009200c", DAT_1009200c).wrapping_add(4)) as *const i32),
            read_unaligned
        ) {
            loop {
                let pvar3 = mech_trace::cr!(
                    "1002ff60",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1002ff60", "1009200c", DAT_1009200c) as i32,
                        i_var6
                    )
                ) as *mut i32;
                let c_var1 = mech_trace::rd!("1002ff60", pvar3.add(2), read_unaligned) as i8;
                if c_var1 == 0 {
                    FUN_1000c770(pvar3, 0);
                } else if c_var1 == 1 {
                    mech_trace::cr!(
                        "1002ff60",
                        "10017df0",
                        FUN_10017df0(
                            mech_trace::gr!("1002ff60", "100a67b8", DAT_100a67b8),
                            pvar3 as u32
                        )
                    );
                } else if c_var1 == 2 {
                    FUN_1000c770(pvar3, 0);
                } else {
                    FUN_1005b1b0((&raw mut DAT_10093260) as *const i8);
                }
                i_var6 = i_var6.wrapping_add(1);
                if !(i_var6
                    < mech_trace::rd!(
                        "1002ff60",
                        ((mech_trace::gr!("1002ff60", "1009200c", DAT_1009200c).wrapping_add(4))
                            as *const i32),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }

        i_var5 = 0;
        if 0 < mech_trace::rd!(
            "1002ff60",
            ((mech_trace::gr!("1002ff60", "10092010", DAT_10092010).wrapping_add(4)) as *const i32),
            read_unaligned
        ) {
            loop {
                let pvar3 = mech_trace::cr!(
                    "1002ff60",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1002ff60", "10092010", DAT_10092010) as i32,
                        i_var5
                    )
                ) as *mut i32;
                let c_var1 = mech_trace::rd!("1002ff60", pvar3.add(2), read_unaligned) as i8;
                if c_var1 == 0 {
                    mech_trace::cr!("1002ff60", "1000c6b0", FUN_1000c6b0(pvar3 as i32, 0));
                } else if c_var1 == 1 {
                    i_var6 = 0;
                    if 0 < mech_trace::rd!(
                        "1002ff60",
                        mech_trace::gr!("1002ff60", "100a67b8", DAT_100a67b8).add(1),
                        read_unaligned
                    ) {
                        loop {
                            let pvar4 = mech_trace::cr!(
                                "1002ff60",
                                "10018060",
                                FUN_10018060(
                                    mech_trace::gr!("1002ff60", "100a67b8", DAT_100a67b8) as i32,
                                    i_var6
                                )
                            ) as *mut i32;
                            if mech_trace::rd!("1002ff60", pvar4, read_unaligned)
                                == mech_trace::rd!("1002ff60", pvar3, read_unaligned)
                            {
                                FUN_10017ec0(
                                    mech_trace::gr!("1002ff60", "100a67b8", DAT_100a67b8) as i32,
                                    pvar4 as i32,
                                    0,
                                );
                            }
                            i_var6 = i_var6.wrapping_add(1);
                            if !(i_var6
                                < mech_trace::rd!(
                                    "1002ff60",
                                    mech_trace::gr!("1002ff60", "100a67b8", DAT_100a67b8).add(1),
                                    read_unaligned
                                ))
                            {
                                break;
                            }
                        }
                    }
                } else if c_var1 == 2 {
                    mech_trace::cr!("1002ff60", "1000c6b0", FUN_1000c6b0(pvar3 as i32, 0));
                }
                i_var5 = i_var5.wrapping_add(1);
                if !(i_var5
                    < mech_trace::rd!(
                        "1002ff60",
                        ((mech_trace::gr!("1002ff60", "10092010", DAT_10092010).wrapping_add(4))
                            as *const i32),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
        FUN_10030090();
    }
}
