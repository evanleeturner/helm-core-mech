// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: u32;
    static mut DAT_100adf38: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100051a2(param_1: u32, param_2: i32, param_3: i32) -> u8;
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_10030304(param_1: *mut i32, param_2: u32) -> u8;
    fn FUN_10030282(param_1: *mut i32, param_2: u32) -> u8;
    fn FUN_10004b19(param_1: i32, param_2: *mut i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10051f89() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10005228(param_1: i32, param_2: *mut i32, param_3: u32, param_4: i32) {
    mech_trace::enter!("10005228", param_1, param_2, param_3, param_4);
    // SAFETY: all pointers are the game's own structures; reads, writes and
    // calls happen in the same order as the original.
    unsafe {
        if param_3 == 0 {
            return;
        }
        let mut i_var1: i32 = mech_trace::rd!("10005228", param_2.add(0x16), read_unaligned)
            .wrapping_add((param_3 as i32).wrapping_mul(0x28));
        let flags = (i_var1.wrapping_sub(2)) as *mut u16;
        if (mech_trace::rd!("10005228", flags, read_unaligned) & 0x2000) != 0 {
            return;
        }
        mech_trace::wr!(
            "10005228",
            (i_var1.wrapping_sub(0x20) as *mut u32),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10005228",
            flags,
            write_unaligned,
            mech_trace::rd!("10005228", flags, read_unaligned) | 0x2000
        );
        mech_trace::cr!(
            "10005228",
            "100051a2",
            FUN_100051a2(param_1 as u32, param_2 as i32, param_3 as i32)
        );
        match param_3 {
            2 => {
                FUN_10005228(param_1, param_2, 5, 0);
                mech_trace::wr!(
                    "10005228",
                    flags,
                    write_unaligned,
                    mech_trace::rd!("10005228", flags, read_unaligned) & 0xdfff
                );
                return;
            }
            3 => {
                FUN_10005228(param_1, param_2, 5, 0);
                FUN_10005228(param_1, param_2, 2, 0);
                FUN_10005228(param_1, param_2, 4, 0);
                FUN_10005228(param_1, param_2, 6, 0);
                FUN_10005228(param_1, param_2, 1, 0);
            }
            4 => {
                FUN_10005228(param_1, param_2, 6, 0);
                mech_trace::wr!(
                    "10005228",
                    flags,
                    write_unaligned,
                    mech_trace::rd!("10005228", flags, read_unaligned) & 0xdfff
                );
                return;
            }
            5 | 6 => {
                let p = mech_trace::rd!(
                    "10005228",
                    ((mech_trace::rd!("10005228", param_2, read_unaligned) as usize)
                        .wrapping_add(0x40) as *const i32),
                    read_unaligned
                ) as *mut i32;
                if param_4 == 0 {
                    mech_trace::cr!("10005228", "10030304", FUN_10030304(p, param_3));
                    return;
                }
                mech_trace::cr!("10005228", "10030282", FUN_10030282(p, param_3));
                return;
            }
            7 | 8 => {
                let fl2 = param_2.add(0x110) as *mut u16;
                if (mech_trace::rd!("10005228", fl2, read_unaligned) & 0x20) == 0 {
                    let i_var2 = mech_trace::cr!("10005228", "10051f43", FUN_10051f43(100));
                    if i_var2 < 0x5f
                        && param_4 == 0
                        && (mech_trace::rd!(
                            "10005228",
                            (mech_trace::rd!("10005228", param_2, read_unaligned) as *const i32),
                            read_unaligned
                        ) == 1)
                    {
                        mech_trace::wr!(
                            "10005228",
                            (i_var1.wrapping_sub(0x20) as *mut u32),
                            write_unaligned,
                            0x10000
                        );
                        mech_trace::wr!(
                            "10005228",
                            flags,
                            write_unaligned,
                            mech_trace::rd!("10005228", flags, read_unaligned) & 0xdfff
                        );
                        mech_trace::wr!("10005228", param_2.add(0xf9), write_unaligned, 0x3333);
                        return;
                    }
                    let p = mech_trace::rd!(
                        "10005228",
                        ((mech_trace::rd!("10005228", param_2, read_unaligned) as usize)
                            .wrapping_add(0x40) as *const i32),
                        read_unaligned
                    ) as *mut i32;
                    if param_4 == 0 {
                        mech_trace::cr!("10005228", "10030304", FUN_10030304(p, param_3));
                    } else {
                        mech_trace::cr!("10005228", "10030282", FUN_10030282(p, param_3));
                    }
                    mech_trace::wr!("10005228", param_2.add(0xf9), write_unaligned, 0);
                    mech_trace::wr!(
                        "10005228",
                        fl2,
                        write_unaligned,
                        mech_trace::rd!("10005228", fl2, read_unaligned) | 0x20
                    );
                    if mech_trace::rd!(
                        "10005228",
                        (mech_trace::rd!("10005228", param_2, read_unaligned) as *const i32),
                        read_unaligned
                    ) != 1
                    {
                        return;
                    }
                    let f3 = param_2.byte_add(0x442) as *mut u16;
                    mech_trace::wr!(
                        "10005228",
                        f3,
                        write_unaligned,
                        mech_trace::rd!("10005228", f3, read_unaligned) | 4
                    );
                    i_var1 = mech_trace::cr!("10005228", "10051f89", FUN_10051f89()) as i32;
                    let prod = i_var1.wrapping_mul(0x10000);
                    let s = prod.wrapping_add((prod >> 31) & 0x3ff);
                    mech_trace::wr!("10005228", param_2.add(0x114), write_unaligned, s >> 10);
                    return;
                }
                FUN_10005228(param_1, param_2, 3, 0);
            }
            _ => {
                let i_var2 = mech_trace::cr!("10005228", "10051f43", FUN_10051f43(100));
                let t = mech_trace::rd!(
                    "10005228",
                    ((mech_trace::rd!("10005228", param_2, read_unaligned) as usize).wrapping_add(4)
                        as *const u32),
                    read_unaligned
                );
                if 0x13 < i_var2
                    || (t != mech_trace::gr!("10005228", "100adf38", DAT_100adf38)
                        && mech_trace::gr!("10005228", "100aa1ac", DAT_100aa1ac) != 0)
                {
                    mech_trace::wr!(
                        "10005228",
                        (i_var1.wrapping_sub(0x20) as *mut u32),
                        write_unaligned,
                        0x10000
                    );
                    mech_trace::wr!(
                        "10005228",
                        flags,
                        write_unaligned,
                        mech_trace::rd!("10005228", flags, read_unaligned) & 0xdfff
                    );
                    return;
                }
                mech_trace::cr!("10005228", "10004b19", FUN_10004b19(param_1, param_2));
            }
        }
        let p = mech_trace::rd!(
            "10005228",
            ((mech_trace::rd!("10005228", param_2, read_unaligned) as usize).wrapping_add(0x40)
                as *const i32),
            read_unaligned
        ) as *mut i32;
        if param_4 == 0 {
            mech_trace::cr!("10005228", "10030304", FUN_10030304(p, param_3));
            if mech_trace::rd!("10005228", param_2.add(0xf5), read_unaligned) != 4
                && mech_trace::rd!("10005228", param_2.add(0xf5), read_unaligned) != 5
            {
                let t = mech_trace::rd!(
                    "10005228",
                    ((mech_trace::rd!("10005228", param_2, read_unaligned) as usize).wrapping_add(4)
                        as *const u32),
                    read_unaligned
                );
                if t == mech_trace::gr!("10005228", "100adf38", DAT_100adf38)
                    || mech_trace::gr!("10005228", "100aa1ac", DAT_100aa1ac) == 0
                {
                    mech_trace::cr!("10005228", "10004b19", FUN_10004b19(param_1, param_2));
                }
            }
        } else {
            mech_trace::cr!("10005228", "10030282", FUN_10030282(p, param_3));
        }
    }
}
