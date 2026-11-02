// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa558: i32;
    static mut DAT_100aa55c: i32;
    static mut DAT_100aa560: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007f35b(a: *mut i32, b: *mut i32, c: *mut i32, d: *mut i32, e: *mut i32) -> i32;
    fn FUN_1007f876(a: i32, b: *mut i32, c: *mut i32, d: *mut i32, e: *mut i32) -> i32;
    fn FUN_100261f0(a: *mut i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32);
    fn FUN_1002634e(a: i32) -> i32;
    fn FUN_10003d60(a: i32, b: i32) -> i32;
    fn FUN_1002650e(a: *mut i32, b: i32);
    fn FUN_10014991(a: *mut i32, b: *mut i32) -> i32;
    fn FUN_1002637d(a: i32) -> i32;
    fn FUN_1000fc30(a: i32, b: i32, c: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007ee80(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: *mut i32,
    param_8: *mut i32,
    param_9: *mut i32,
) -> i32 {
    mech_trace::enter!(
        "1007ee80", param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8, param_9
    );
    // SAFETY: all pointers are the game's own structures and globals; reads,
    // writes and calls happen in the original's order.
    unsafe {
        let mut local_b4 = [0i32; 12];
        let mut local_74 = [0i32; 12];
        let mut local_2c: i32 = 0;
        let mut local_24: i32;

        let base = mech_trace::rd!("1007ee80", param_1, read) as usize;
        let local_18 = mech_trace::rd!("1007ee80", ((base + 0x50) as *const i32), read_unaligned);
        let local_20 = mech_trace::rd!("1007ee80", ((base + 0x54) as *const i32), read_unaligned);
        let local_28 = mech_trace::rd!("1007ee80", ((base + 0x58) as *const i32), read_unaligned);
        let local_30 = local_18.wrapping_add(param_4);
        let local_34 = local_20.wrapping_add(param_5);
        let local_3c = param_6.wrapping_add(local_28);
        mech_trace::wr!("1007ee80", param_7, write, local_30);
        mech_trace::wr!("1007ee80", param_8, write, local_34);
        mech_trace::wr!("1007ee80", param_9, write, local_3c);
        local_24 = 0;
        let state = mech_trace::rd!("1007ee80", param_1.add(0xf5), read);
        mech_trace::rt!(
            "1007ee80",
            if state == 2 || state == 4 {
                let r = mech_trace::cr!(
                    "1007ee80",
                    "1007f35b",
                    FUN_1007f35b(param_1, param_7, param_8, param_9, param_3)
                );
                if r == 0 {
                    let r2 = mech_trace::cr!(
                        "1007ee80",
                        "1007f876",
                        FUN_1007f876(param_1 as i32, param_7, param_8, param_9, &raw mut local_2c)
                    );
                    if r2 == 0 {
                        if param_4 == 0 && param_5 == 0 && param_6 == 0 {
                            if mech_trace::rd!("1007ee80", param_1.add(0xf6), read) != 0 {
                                let v = mech_trace::rd!("1007ee80", param_1.add(0xf6), read)
                                    .wrapping_add(1);
                                mech_trace::wr!("1007ee80", param_1.add(0xf6), write, v);
                                return mech_trace::rt!(
                                    "1007ee80",
                                    mech_trace::rd!("1007ee80", param_1.add(0xf6), read)
                                );
                            }
                            return mech_trace::rt!("1007ee80", 0);
                        }
                        let p74 = local_74.as_mut_ptr();
                        FUN_100261f0(
                            p74, local_18, local_20, local_28, local_30, local_34, local_3c,
                        );
                        let local_38 =
                            mech_trace::cr!("1007ee80", "1002634e", FUN_1002634e(p74 as i32));
                        let mut local_78 = local_74[10];
                        if local_78 < 0 {
                            local_78 = local_78.wrapping_neg();
                        }
                        let i1 = mech_trace::cr!(
                            "1007ee80",
                            "10003d60",
                            FUN_10003d60(
                                mech_trace::rd!("1007ee80", param_1.add(0x107), read),
                                0x10000i32.wrapping_sub(local_78)
                            )
                        );
                        local_78 = mech_trace::cr!(
                            "1007ee80",
                            "10003d60",
                            FUN_10003d60(
                                mech_trace::rd!("1007ee80", param_1.add(0x100), read),
                                local_78
                            )
                        );
                        local_78 = i1.wrapping_add(local_78);
                        FUN_1002650e(p74, local_78.wrapping_add(local_38));
                        let r3 = mech_trace::cr!(
                            "1007ee80",
                            "10014991",
                            FUN_10014991(p74, &raw mut local_2c)
                        );
                        if r3 != 0 {
                            local_24 = 1;
                            let v = mech_trace::rd!("1007ee80", param_1.add(0xf6), read)
                                .wrapping_add(1);
                            mech_trace::wr!("1007ee80", param_1.add(0xf6), write, v);
                            mech_trace::wr!("1007ee80", param_2, write, local_2c);
                            if mech_trace::gr!("1007ee80", "100aa558", DAT_100aa558) == 0
                                && mech_trace::gr!("1007ee80", "100aa55c", DAT_100aa55c) == 0
                                && mech_trace::gr!("1007ee80", "100aa560", DAT_100aa560) == 0
                                && local_2c != 0
                            {
                                let m = local_2c as usize;
                                FUN_100261f0(
                                    local_b4.as_mut_ptr(),
                                    mech_trace::rd!(
                                        "1007ee80",
                                        ((m + 0x34) as *const i32),
                                        read_unaligned
                                    ),
                                    mech_trace::rd!(
                                        "1007ee80",
                                        ((m + 0x38) as *const i32),
                                        read_unaligned
                                    ),
                                    mech_trace::rd!(
                                        "1007ee80",
                                        ((m + 0x3c) as *const i32),
                                        read_unaligned
                                    ),
                                    local_74[3],
                                    local_74[4],
                                    local_74[5],
                                );
                                mech_trace::cr!(
                                    "1007ee80",
                                    "1002637d",
                                    FUN_1002637d(local_b4.as_mut_ptr() as i32)
                                );
                                mech_trace::gw!("1007ee80", "100aa558", DAT_100aa558, local_b4[9]);
                                mech_trace::gw!("1007ee80", "100aa55c", DAT_100aa55c, local_b4[10]);
                                mech_trace::gw!("1007ee80", "100aa560", DAT_100aa560, local_b4[11]);
                            }
                            let a = mech_trace::cr!(
                                "1007ee80",
                                "10003d60",
                                FUN_10003d60(
                                    mech_trace::gr!("1007ee80", "100aa558", DAT_100aa558)
                                        .wrapping_shl(1),
                                    local_74[9]
                                )
                            );
                            let b = mech_trace::cr!(
                                "1007ee80",
                                "10003d60",
                                FUN_10003d60(
                                    mech_trace::gr!("1007ee80", "100aa55c", DAT_100aa55c)
                                        .wrapping_shl(1),
                                    local_74[10]
                                )
                            );
                            let mut local_7c = mech_trace::cr!(
                                "1007ee80",
                                "10003d60",
                                FUN_10003d60(
                                    mech_trace::gr!("1007ee80", "100aa560", DAT_100aa560)
                                        .wrapping_shl(1),
                                    local_74[11]
                                )
                            );
                            local_7c = a.wrapping_add(b).wrapping_add(local_7c);
                            let mut local_c = mech_trace::cr!(
                                "1007ee80",
                                "10003d60",
                                FUN_10003d60(
                                    local_7c,
                                    mech_trace::gr!("1007ee80", "100aa558", DAT_100aa558)
                                )
                            );
                            local_c = local_74[9].wrapping_sub(local_c);
                            let mut local_10 = mech_trace::cr!(
                                "1007ee80",
                                "10003d60",
                                FUN_10003d60(
                                    local_7c,
                                    mech_trace::gr!("1007ee80", "100aa55c", DAT_100aa55c)
                                )
                            );
                            local_10 = local_74[10].wrapping_sub(local_10);
                            let mut local_14 = mech_trace::cr!(
                                "1007ee80",
                                "10003d60",
                                FUN_10003d60(
                                    local_7c,
                                    mech_trace::gr!("1007ee80", "100aa560", DAT_100aa560)
                                )
                            );
                            local_14 = local_74[11].wrapping_sub(local_14);
                            let len = mech_trace::cr!(
                                "1007ee80",
                                "1000fc30",
                                FUN_1000fc30(
                                    mech_trace::rd!("1007ee80", param_1.add(0x10d), read),
                                    mech_trace::rd!("1007ee80", param_1.add(0x10e), read),
                                    mech_trace::rd!("1007ee80", param_1.add(0x10f), read),
                                )
                            );
                            let local_8 = len.wrapping_neg() >> 2;
                            let t = mech_trace::cr!(
                                "1007ee80",
                                "10003d60",
                                FUN_10003d60(local_c, local_8)
                            );
                            mech_trace::wr!("1007ee80", param_1.add(0x10a), write, t);
                            let t = mech_trace::cr!(
                                "1007ee80",
                                "10003d60",
                                FUN_10003d60(local_10, local_8)
                            );
                            mech_trace::wr!("1007ee80", param_1.add(0x10b), write, t);
                            let t = mech_trace::cr!(
                                "1007ee80",
                                "10003d60",
                                FUN_10003d60(local_14, local_8)
                            );
                            mech_trace::wr!("1007ee80", param_1.add(0x10c), write, t);
                            let t =
                                mech_trace::cr!("1007ee80", "1002634e", FUN_1002634e(p74 as i32));
                            FUN_1002650e(p74, t.wrapping_sub(local_78));
                            let i1 = local_38;
                            let mut local_1c =
                                mech_trace::cr!("1007ee80", "1002634e", FUN_1002634e(p74 as i32));
                            local_1c = i1.wrapping_sub(local_1c);
                            if local_1c < 1 {
                                mech_trace::wr!("1007ee80", param_7, write, local_74[3]);
                                mech_trace::wr!("1007ee80", param_8, write, local_74[4]);
                                mech_trace::wr!("1007ee80", param_9, write, local_74[5]);
                            } else {
                                local_c = mech_trace::cr!(
                                    "1007ee80",
                                    "10003d60",
                                    FUN_10003d60(local_c, local_1c)
                                );
                                local_c >>= 2;
                                local_10 = mech_trace::cr!(
                                    "1007ee80",
                                    "10003d60",
                                    FUN_10003d60(local_10, local_1c)
                                );
                                local_10 >>= 2;
                                let i1 = mech_trace::cr!(
                                    "1007ee80",
                                    "10003d60",
                                    FUN_10003d60(local_14, local_1c)
                                );
                                mech_trace::wr!(
                                    "1007ee80",
                                    param_7,
                                    write,
                                    local_74[3].wrapping_add(local_c)
                                );
                                mech_trace::wr!(
                                    "1007ee80",
                                    param_8,
                                    write,
                                    local_74[4].wrapping_add(local_10)
                                );
                                mech_trace::wr!(
                                    "1007ee80",
                                    param_9,
                                    write,
                                    local_74[5].wrapping_add(i1 >> 2)
                                );
                            }
                        }
                    } else {
                        local_24 = 1;
                        let v =
                            mech_trace::rd!("1007ee80", param_1.add(0xf6), read).wrapping_add(1);
                        mech_trace::wr!("1007ee80", param_1.add(0xf6), write, v);
                        mech_trace::wr!("1007ee80", param_2, write, local_2c);
                    }
                } else {
                    let b = mech_trace::rd!("1007ee80", param_1, read) as usize;
                    let src =
                        (mech_trace::rd!("1007ee80", param_3, read) as usize + 4) as *const i32;
                    mech_trace::wr!(
                        "1007ee80",
                        ((b + 0x7c) as *mut i32),
                        write_unaligned,
                        mech_trace::rd!("1007ee80", src, read_unaligned)
                    );
                    local_24 = 1;
                    let v = mech_trace::rd!("1007ee80", param_1.add(0xf6), read).wrapping_add(1);
                    mech_trace::wr!("1007ee80", param_1.add(0xf6), write, v);
                    mech_trace::wr!("1007ee80", param_2, write, 0);
                }
                if local_24 == 0 {
                    mech_trace::wr!("1007ee80", param_1.add(0xf6), write, 0);
                    mech_trace::wr!("1007ee80", param_2, write, 0);
                }
                mech_trace::rd!("1007ee80", param_1.add(0xf6), read)
            } else {
                mech_trace::wr!("1007ee80", param_1.add(0xf6), write, 0);
                0
            }
        )
    }
}
