// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: i32;
    static mut _DAT_100a1220: f64;
    static mut DAT_100adf38: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_100444ea(
        param_1: i32,
        param_2: *mut i32,
        param_3: i32,
        param_4: i16,
        param_5: i32,
        param_6: i32,
    );
    fn FUN_10014768(param_1: *mut i32, param_2: *mut i32, param_3: i32) -> i32;
    fn FUN_1004448b(param_1: i32) -> i32;
    fn FUN_100446b5(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32) -> u32;
    fn __ftol() -> i64;
    fn FUN_1002a132(param_1: i32) -> u32;
    fn FUN_100447bf(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10044054(param_1: *mut i32) -> i32 {
    mech_trace::enter!("10044054", param_1);
    // SAFETY: param_1 points at the game's object; all offsets, globals and
    // callees are the game's own, accessed in the original's order.
    unsafe {
        let rd = |i: usize| -> i32 { mech_trace::rd!("10044054", param_1.add(i), read_unaligned) };
        let wr = |i: usize, v: i32| mech_trace::wr!("10044054", param_1.add(i), write_unaligned, v);
        let p1e6 = (param_1 as *mut u8).byte_add(0x1e6).cast::<u32>();

        let mut local_50: i32 = 0;
        let mut local_44: [i32; 14] = [0; 14];
        let mut local_c: i32 = 0;
        let b_var5: i32;
        let i_var3 = rd(8);
        let p13 = rd(0x13);

        if mech_trace::rd!(
            "10044054",
            ((i_var3 as u32 as usize).wrapping_add(0x3b8) as *const i32),
            read_unaligned
        ) == 0
            || ((rd(0x79) as i16 == 0
                && mech_trace::rd!(
                    "10044054",
                    ((p13 as u32 as usize).wrapping_add(8) as *const i32),
                    read_unaligned
                ) == 0)
                || mech_trace::rd!(
                    "10044054",
                    ((p13 as u32 as usize).wrapping_add(0x30) as *const i8),
                    read_unaligned
                ) != 0)
        {
            b_var5 = 0;
        } else if mech_trace::gr!("10044054", "100b38fc", DAT_100b38fc) < rd(0x78) {
            b_var5 = (rd(0x77) != 0) as i32;
        } else {
            if rd(0x79) as i16 == 0 {
                let a = mech_trace::rd!(
                    "10044054",
                    ((i_var3 as u32 as usize).wrapping_add(0x428) as *const i32),
                    read_unaligned
                );
                let b = mech_trace::rd!(
                    "10044054",
                    ((i_var3 as u32 as usize).wrapping_add(0x430) as *const i32),
                    read_unaligned
                );
                let local_8 = mech_trace::cr!("10044054", "1000fc30", FUN_1000fc30(a, 0, b));
                let u_var2 = mech_trace::cr!("10044054", "10002570", FUN_10002570(local_8, 500000));
                mech_trace::wr!("10044054", p1e6, write_unaligned, u_var2);
                if mech_trace::rd!("10044054", param_1, read_unaligned) != 1 {
                    let mut l = mech_trace::rd!("10044054", p1e6, read_unaligned);
                    l >>= 1;
                    mech_trace::wr!("10044054", p1e6, write_unaligned, l);
                }
                let l = mech_trace::rd!("10044054", p1e6, read_unaligned);
                if (l as f64) < mech_trace::gr!("10044054", "100a1220", _DAT_100a1220) {
                    mech_trace::wr!("10044054", p1e6, write_unaligned, 0x4ccc);
                }
            }
            let mut s_var1: i16 = 0;
            while s_var1 < 4 {
                let mut i_var7: i32 = 0;
                let l = mech_trace::rd!("10044054", p1e6, read_unaligned);
                let mut i_var3 =
                    mech_trace::cr!("10044054", "10003d60", FUN_10003d60(l as i32, 0x13880000));
                FUN_100444ea(
                    param_1 as i32,
                    local_44.as_mut_ptr(),
                    (rd(0x79) as i16) as i32,
                    s_var1,
                    i_var3 >> 0x10,
                    i_var7,
                );
                i_var3 = mech_trace::cr!(
                    "10044054",
                    "10014768",
                    FUN_10014768(local_44.as_mut_ptr(), &raw mut local_c, rd(1))
                );
                if i_var3 == 0 {
                    if s_var1 == 0 {
                        let u_var4 =
                            mech_trace::cr!("10044054", "1002a132", FUN_1002a132(param_1 as i32));
                        let sg = ((u_var4 as i32) >> 0x1f) as u32;
                        if ((u_var4 ^ sg).wrapping_sub(sg) as i32) < 0x10001 {
                            wr(0x77, 0);
                            mech_trace::wr!(
                                "10044054",
                                param_1.add(0x79).cast::<u16>(),
                                write_unaligned,
                                0
                            );
                        }
                    }
                    if s_var1 == 0 && rd(0x79) as i16 != 0 {
                        i_var7 = 1;
                        let l = mech_trace::rd!("10044054", p1e6, read_unaligned);
                        i_var3 = mech_trace::cr!(
                            "10044054",
                            "10003d60",
                            FUN_10003d60(l as i32, 0x13880000)
                        );
                        FUN_100444ea(
                            param_1 as i32,
                            local_44.as_mut_ptr(),
                            ((rd(0x79) as i16) as i32).wrapping_neg(),
                            1,
                            i_var3 >> 0x10,
                            i_var7,
                        );
                        i_var3 = mech_trace::cr!(
                            "10044054",
                            "10014768",
                            FUN_10014768(local_44.as_mut_ptr(), &raw mut local_c, rd(1))
                        );
                        if i_var3 != 0 {
                            let b = mech_trace::cr!("10044054", "1004448b", FUN_1004448b(local_c));
                            if b == 0 {
                                let l6 = mech_trace::cr!("10044054", "__ftol", __ftol());
                                local_50 = local_50.wrapping_add(l6 as i32);
                            }
                        }
                    }
                    break;
                }
                let b = mech_trace::cr!("10044054", "1004448b", FUN_1004448b(local_c));
                if b != 0 {
                    break;
                }
                if rd(0x79) as i16 == 0 {
                    let u_var2 = mech_trace::cr!(
                        "10044054",
                        "100446b5",
                        FUN_100446b5(param_1 as i32, local_c, rd(0x14), rd(0x15), rd(0x16))
                    );
                    mech_trace::wr!(
                        "10044054",
                        param_1.add(0x79).cast::<u16>(),
                        write_unaligned,
                        u_var2 as u16
                    );
                    wr(0x77, local_c);
                }
                let l6 = mech_trace::cr!("10044054", "__ftol", __ftol());
                local_50 = local_50.wrapping_add(l6 as i32);
                s_var1 = s_var1.wrapping_add(1);
            }
            if local_50 != 0 {
                local_50 =
                    mech_trace::cr!("10044054", "100447bf", FUN_100447bf(local_50, 0x3333333));
                let p13 = rd(0x13) as u32 as usize;
                mech_trace::wr!(
                    "10044054",
                    ((p13.wrapping_add(0xc)) as *mut i32),
                    write_unaligned,
                    local_50
                );
                let local_54: i32 =
                    if rd(1) == mech_trace::gr!("10044054", "100adf38", DAT_100adf38) {
                        rd(0x75)
                    } else {
                        0x400
                    };
                let p13 = rd(0x13) as u32 as usize;
                if s_var1 == 0 {
                    mech_trace::wr!(
                        "10044054",
                        ((p13.wrapping_add(8)) as *mut i32),
                        write_unaligned,
                        local_54
                    );
                } else {
                    mech_trace::wr!(
                        "10044054",
                        ((p13.wrapping_add(8)) as *mut u32),
                        write_unaligned,
                        0x100
                    );
                }
                if mech_trace::rd!("10044054", param_1, read_unaligned) == 6 {
                    let p13 = rd(0x13) as u32 as usize;
                    mech_trace::wr!(
                        "10044054",
                        ((p13.wrapping_add(8)) as *mut u32),
                        write_unaligned,
                        0
                    );
                    let p13 = rd(0x13) as u32 as usize;
                    mech_trace::wr!(
                        "10044054",
                        ((p13.wrapping_add(0xc)) as *mut u32),
                        write_unaligned,
                        0
                    );
                }
            }
            if rd(1) == mech_trace::gr!("10044054", "100adf38", DAT_100adf38) {
                wr(
                    0x78,
                    mech_trace::gr!("10044054", "100b38fc", DAT_100b38fc).wrapping_add(10),
                );
            } else {
                let m = (0u32.wrapping_sub((rd(0x77) == 0) as u32)) & 0x5b;
                wr(
                    0x78,
                    m.wrapping_add(0x5a).wrapping_add(mech_trace::gr!(
                        "10044054",
                        "100b38fc",
                        DAT_100b38fc
                    ) as u32) as i32,
                );
            }
            b_var5 = (local_50 != 0) as i32;
        }
        mech_trace::rt!("10044054", b_var5)
    }
}
