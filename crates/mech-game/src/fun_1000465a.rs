// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100adf34: i32;
    static mut DAT_100b3904: i32;
    static mut DAT_100b38fc: i32;
    static mut _DAT_100a1030: f64;
    static mut _DAT_100a1038: f64;
    static mut _DAT_100a1040: f64;
    static mut DAT_100a4118: i32;
    static mut _DAT_100c4c8c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100082c4(param_1: i32, param_2: i32);
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_100044c6(param_1: i32, param_2: *mut i32);
    fn FUN_10004b19(param_1: i32, param_2: *mut i32);
    fn FUN_10003742(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000465a(param_1: *mut i32) {
    mech_trace::enter!("1000465a", param_1);
    // SAFETY: param_1 points at the game's object; globals and callees are the
    // game's own; all reads and writes follow the original's offsets and order.
    unsafe {
        let p = param_1;
        let obj = || mech_trace::rd!("1000465a", p, read_unaligned) as *const u8;
        let w14 = || {
            mech_trace::rd!(
                "1000465a",
                obj().byte_add(0x14).cast::<u16>(),
                read_unaligned
            )
        };
        let id = || mech_trace::rd!("1000465a", obj().byte_add(4).cast::<i32>(), read_unaligned);
        let g = |i: usize| mech_trace::rd!("1000465a", p.add(i), read_unaligned);
        let s = |i: usize, v: i32| mech_trace::wr!("1000465a", p.add(i), write_unaligned, v);
        let fp = p.add(0x110).cast::<u16>();
        let fl = || mech_trace::rd!("1000465a", fp, read_unaligned);

        if (w14() & 2) != 0 || (w14() & 4) != 0 {
            return;
        }
        if !(mech_trace::gr!("1000465a", "100aa1ac", DAT_100aa1ac) == 0
            || id() == mech_trace::gr!("1000465a", "100adf38", DAT_100adf38))
        {
            return;
        }
        if (mech_trace::dr!(
            "1000465a",
            (mech_trace::gr!("1000465a", "100adf34", DAT_100adf34).wrapping_add(4) as *const u8)
        ) == 0)
            && id() == mech_trace::gr!("1000465a", "100adf38", DAT_100adf38)
        {
            s(0xf3, 0);
            s(0xf1, 0);
            return;
        }
        let prod =
            (g(0xf4) as u32)
                .wrapping_mul(mech_trace::gr!("1000465a", "100b3904", DAT_100b3904) as u32);
        let shifted = prod.wrapping_shl((g(0xf5) == 3) as u32);
        let mut i_var1 = (g(0xf1) as u32).wrapping_sub(shifted) as i32;
        s(0xf3, (g(0xf3) as u32).wrapping_add(i_var1 as u32) as i32);
        if g(0xf3) < 0 {
            s(0xf3, 0);
        }
        let i_var2 = g(0xf3) >> 0x10;
        if (fl() & 4) != 0
            && (fl() & 8) == 0
            && g(0xf5) != 3
            && 0x43e
                < (mech_trace::gr!("1000465a", "100b38fc", DAT_100b38fc) as u32)
                    .wrapping_sub(g(0xef) as u32) as i32
        {
            s(0xf5, 3);
            s(0xef, mech_trace::gr!("1000465a", "100b38fc", DAT_100b38fc));
            if id() == mech_trace::gr!("1000465a", "100adf38", DAT_100adf38) {
                FUN_100082c4(0xd, -1);
            }
        }
        if i_var2 < 0x65 {
            if (i_var2 as f64) <= mech_trace::gr!("1000465a", "100a1030", _DAT_100a1030) {
                if (i_var2 as f64) <= mech_trace::gr!("1000465a", "100a1038", _DAT_100a1038) {
                    if (fl() & 4) == 0 {
                        if mech_trace::gr!("1000465a", "100a4118", DAT_100a4118) != 0
                            && 0x2d4
                                < (mech_trace::gr!("1000465a", "100b38fc", DAT_100b38fc) as u32)
                                    .wrapping_sub(mech_trace::gr!(
                                        "1000465a",
                                        "100c4c8c",
                                        _DAT_100c4c8c
                                    ) as u32) as i32
                            && id() == mech_trace::gr!("1000465a", "100adf38", DAT_100adf38)
                            && (i_var2 as f64)
                                < mech_trace::gr!("1000465a", "100a1040", _DAT_100a1040)
                        {
                            mech_trace::gw!("1000465a", "100a4118", DAT_100a4118, 0);
                        }
                        mech_trace::wr!("1000465a", fp, write_unaligned, fl() & 0xefff);
                    } else {
                        if g(0xf5) == 3
                            && 0x2d4
                                < (mech_trace::gr!("1000465a", "100b38fc", DAT_100b38fc) as u32)
                                    .wrapping_sub(g(0xef) as u32)
                                    as i32
                        {
                            mech_trace::wr!("1000465a", fp, write_unaligned, fl() & 0xfffb);
                            s(0xf5, 0);
                        }
                        if (fl() & 8) != 0 {
                            mech_trace::wr!("1000465a", fp, write_unaligned, fl() & 0xfff7);
                            mech_trace::wr!("1000465a", fp, write_unaligned, fl() & 0xfffb);
                        }
                    }
                } else if id() == mech_trace::gr!("1000465a", "100adf38", DAT_100adf38)
                    && mech_trace::gr!("1000465a", "100a4118", DAT_100a4118) == 0
                    && 0 < i_var1
                {
                    mech_trace::gw!(
                        "1000465a",
                        "100c4c8c",
                        _DAT_100c4c8c,
                        mech_trace::gr!("1000465a", "100b38fc", DAT_100b38fc)
                    );
                    mech_trace::gw!("1000465a", "100a4118", DAT_100a4118, 1);
                    FUN_100082c4(0, -1);
                }
            } else if (fl() & 4) == 0 {
                s(0xef, mech_trace::gr!("1000465a", "100b38fc", DAT_100b38fc));
                mech_trace::wr!("1000465a", fp, write_unaligned, fl() | 4);
                if (fl() & 0x1000) == 0
                    && id() == mech_trace::gr!("1000465a", "100adf38", DAT_100adf38)
                {
                    mech_trace::wr!("1000465a", fp, write_unaligned, fl() | 0x1000);
                    FUN_100082c4(3, -1);
                    FUN_10003742(0xfe, 100, 0x40, 5, 0x50);
                }
            }
        } else if id() != mech_trace::gr!("1000465a", "100adf38", DAT_100adf38)
            || mech_trace::dr!(
                "1000465a",
                (mech_trace::gr!("1000465a", "100adf34", DAT_100adf34).wrapping_add(1)
                    as *const u8)
            ) == 0
        {
            if (fl() & 8) == 0 {
                if 0xe24
                    < (mech_trace::gr!("1000465a", "100b38fc", DAT_100b38fc) as u32)
                        .wrapping_sub(g(0xef) as u32) as i32
                {
                    FUN_10004b19(id(), p);
                }
            } else if mech_trace::gr!("1000465a", "100b3904", DAT_100b3904) != 0 {
                i_var1 = mech_trace::cr!(
                    "1000465a",
                    "10051f43",
                    FUN_10051f43(
                        (4000i64 / (mech_trace::gr!("1000465a", "100b3904", DAT_100b3904) as i64))
                            as i32
                    )
                );
                if i_var1 < 3 {
                    FUN_100044c6(id(), p);
                }
            }
        }
    }
}
