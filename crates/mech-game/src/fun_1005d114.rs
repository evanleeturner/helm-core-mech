// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: u32;
    static mut DAT_100ab090: u32;
    static mut DAT_100ab094: u32;
    static mut DAT_100adcd0: u32;
    static mut DAT_100adf34: u32;
    static mut DAT_100adf38: u32;
    static mut DAT_100b09f8: u32;
    static mut DAT_100b3214: u32;
    static mut DAT_10114da0: u32;
    static mut DAT_10114da4: u32;
    static mut DAT_10114db0: [u32; 34];
    static mut DAT_10114e00: u32;
    static mut DAT_10114ec8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10004475(param_1: *mut i32);
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16);
    fn FUN_100082c4(param_1: i32, param_2: i32);
    fn FUN_1005d6f3(param_1: i32);
    fn FUN_10033a28(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: i32,
    );
    fn FUN_1003c92a(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100086a2();
    fn FUN_1000a4d6();
    fn FUN_1003c807();
    fn FUN_1000a86e();
    fn FUN_1003c9e2();
    fn FUN_1003ca49();
}

unsafe fn rd(a: i32) -> i32 {
    // SAFETY: the caller passes an address inside the game's own objects.
    unsafe {
        mech_trace::rd!(
            "1005d114",
            (a as u32 as usize as *const i32),
            read_unaligned
        )
    }
}

unsafe fn rd8(a: i32) -> u8 {
    // SAFETY: the caller passes an address inside the game's own objects.
    unsafe { mech_trace::rd!("1005d114", (a as u32 as usize as *const u8), read) }
}

unsafe fn call_slot(obj: u32, off: i32) {
    // SAFETY: obj is a game object whose slot at off holds a function pointer or zero.
    unsafe {
        let f = rd((obj as i32).wrapping_add(off));
        if f != 0 {
            let func: unsafe extern "C" fn(u32) = core::mem::transmute(f as u32 as usize);
            mech_trace::cr!("1005d114", "ind", func(obj));
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005d114(param_1: *mut i32) {
    mech_trace::enter!("1005d114", param_1);
    // SAFETY: param_1 and everything reached through it, the globals and the
    // callees are the game's own memory, accessed in the original's order.
    unsafe {
        let base = mech_trace::rd!("1005d114", param_1, read);
        if (rd(base.wrapping_add(4)) as u32
            == mech_trace::gr!("1005d114", "100adf38", DAT_100adf38))
            && (mech_trace::gr!("1005d114", "100adcd0", DAT_100adcd0) != 0)
        {
            mech_trace::gw!(
                "1005d114",
                "10114ec8",
                DAT_10114ec8,
                mech_trace::rd!("1005d114", param_1.add(0xf5), read) as u32
            );
            if mech_trace::gr!("1005d114", "10114ec8", DAT_10114ec8) != 2 {
                call_slot(mech_trace::gr!("1005d114", "10114e00", DAT_10114e00), 0xf4);
            }
            if (mech_trace::gr!("1005d114", "10114ec8", DAT_10114ec8) != 5)
                && (mech_trace::gr!("1005d114", "10114ec8", DAT_10114ec8) != 4)
                && (mech_trace::gr!("1005d114", "100adcd0", DAT_100adcd0) != 0)
            {
                let t0 = rd(base.wrapping_add(0x60)) >> 0x10;
                let t1 = (t0 % 0x168) % 0x168;
                mech_trace::gw!(
                    "1005d114",
                    "10114da4",
                    DAT_10114da4,
                    (t1.wrapping_add(0x168) % 0x168) as u32
                );
                let t2 = mech_trace::rd!("1005d114", param_1.add(3), read) >> 0x10;
                mech_trace::gw!(
                    "1005d114",
                    "10114da0",
                    DAT_10114da0,
                    ((t2 % 0x168) % 0x168) as u32
                );
                let i_var1 = rd(base.wrapping_add(0xd8));
                let i_var2 = mech_trace::rd!("1005d114", param_1.add(7), read);
                let t3 = rd(base.wrapping_add(0xd4)) >> 0x10;
                let mut local_18 =
                    (t3 % 0x168)
                        .wrapping_sub(mech_trace::gr!("1005d114", "10114da4", DAT_10114da4) as i32);
                if local_18 < 0xb5 {
                    if local_18 < -0xb4 {
                        local_18 = local_18.wrapping_add(0x168);
                    }
                } else {
                    local_18 = local_18.wrapping_add(-0x168);
                }
                let mut local_c =
                    local_18
                        .wrapping_sub(mech_trace::gr!("1005d114", "10114da0", DAT_10114da0) as i32);
                if local_c < 0xb5 {
                    if local_c < -0xb4 {
                        local_c = local_c.wrapping_add(0x168);
                    }
                } else {
                    local_c = local_c.wrapping_add(-0x168);
                }
                let i_var3 = mech_trace::cr!(
                    "1005d114",
                    "1000fc30",
                    FUN_1000fc30(
                        rd(base.wrapping_add(200)).wrapping_sub(rd(base.wrapping_add(0x50))),
                        rd(base.wrapping_add(0xcc)).wrapping_sub(rd(base.wrapping_add(0x54))),
                        rd(base.wrapping_add(0xd0)).wrapping_sub(rd(base.wrapping_add(0x58))),
                    )
                );
                if mech_trace::gr!("1005d114", "100b3214", DAT_100b3214) != 0 {
                    if mech_trace::gr!("1005d114", "10114ec8", DAT_10114ec8) == 2 {
                        FUN_10004475(param_1);
                    }
                    mech_trace::gw!("1005d114", "100b3214", DAT_100b3214, 0);
                }
                let flags = param_1.add(0x110) as *mut u16;
                if (mech_trace::gr!("1005d114", "100b09f8", DAT_100b09f8) != 0)
                    && (mech_trace::gr!("1005d114", "10114ec8", DAT_10114ec8) != 3)
                    && ((mech_trace::rd!("1005d114", flags, read_unaligned) & 4) != 0
                        && (mech_trace::rd!("1005d114", flags, read_unaligned) & 8) == 0)
                {
                    FUN_10003742(0xde, 100, 0x40, 5, 0x32);
                    FUN_100082c4(2, -1);
                    mech_trace::wr!(
                        "1005d114",
                        flags,
                        write_unaligned,
                        mech_trace::rd!("1005d114", flags, read_unaligned) | 8
                    );
                }
                mech_trace::gw!("1005d114", "100b09f8", DAT_100b09f8, 0);
                FUN_1005d6f3(param_1 as i32);
                if mech_trace::gr!("1005d114", "10114ec8", DAT_10114ec8) == 1 {
                    if (rd8(
                        (mech_trace::gr!("1005d114", "100adf34", DAT_100adf34) as i32)
                            .wrapping_add(9),
                    ) != 0)
                        || (mech_trace::gr!("1005d114", "100aa1ac", DAT_100aa1ac) == 0)
                    {
                        FUN_1000a4d6();
                    }
                    let mut i = 0usize;
                    while i < 0x22 {
                        call_slot(
                            mech_trace::gr!("1005d114", "10114db0", DAT_10114db0[i]),
                            0xf0,
                        );
                        i += 1;
                    }
                    FUN_1003c807();
                } else if mech_trace::gr!("1005d114", "10114ec8", DAT_10114ec8) == 2 {
                    if (rd8(
                        (mech_trace::gr!("1005d114", "100adf34", DAT_100adf34) as i32)
                            .wrapping_add(9),
                    ) != 0)
                        || (mech_trace::gr!("1005d114", "100aa1ac", DAT_100aa1ac) == 0)
                    {
                        FUN_100086a2();
                    }
                    let mut i = 0usize;
                    while i < 0x22 {
                        call_slot(
                            mech_trace::gr!("1005d114", "10114db0", DAT_10114db0[i]),
                            0xf4,
                        );
                        i += 1;
                    }
                    FUN_10033a28(
                        param_1,
                        mech_trace::gr!("1005d114", "10114da4", DAT_10114da4) as i32,
                        mech_trace::gr!("1005d114", "10114da0", DAT_10114da0) as i32,
                        local_18,
                        local_c,
                        i_var1.wrapping_add(i_var2) % 0x1680000,
                        i_var3,
                        mech_trace::gr!("1005d114", "100ab090", DAT_100ab090) as i32,
                    );
                    FUN_1003c92a(mech_trace::rd!("1005d114", param_1.add(0x13), read));
                } else {
                    if (rd8(
                        (mech_trace::gr!("1005d114", "100adf34", DAT_100adf34) as i32)
                            .wrapping_add(9),
                    ) != 0)
                        || (mech_trace::gr!("1005d114", "100aa1ac", DAT_100aa1ac) == 0)
                    {
                        FUN_1000a86e();
                    }
                    let mut i = 0usize;
                    while i < 0x22 {
                        call_slot(
                            mech_trace::gr!("1005d114", "10114db0", DAT_10114db0[i]),
                            0xf8,
                        );
                        i += 1;
                    }
                    FUN_1003c9e2();
                }
                if mech_trace::gr!("1005d114", "100ab094", DAT_100ab094) == 0 {
                    FUN_1003ca49();
                }
            }
        }
    }
}
