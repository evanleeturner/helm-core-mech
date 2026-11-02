// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b2c10: [i32; 24];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10056fc4(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> i32;
    fn FUN_100574ca(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> i32;
    fn FUN_1005b58c(param_1: i32, param_2: i32) -> u32;
    fn FUN_1005b5c0(param_1: i32, param_2: i32) -> i32;
    fn FUN_10058e74(param_1: *mut i32, param_2: i32);
    fn FUN_1005aa1d(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: u32,
        param_5: i32,
        param_6: i32,
    );
}

unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

unsafe fn div64(n: i64, d: i64, old: i32) -> i32 {
    if d == 0 {
        // SAFETY: raises the machine's divide exception as the original does.
        unsafe { RaiseException(0xC0000094, 0, 0, core::ptr::null()) };
        return old;
    }
    let q = n.wrapping_div(d);
    if q < (i32::MIN as i64) || q > (i32::MAX as i64) {
        // SAFETY: raises the machine's divide exception as the original does.
        unsafe { RaiseException(0xC0000094, 0, 0, core::ptr::null()) };
        return old;
    }
    q as i32
}

fn num16(a: u32, b: u32) -> i64 {
    let diff = a.wrapping_sub(b);
    let v = ((diff.wrapping_shl(16)) as i32) >> 16;
    (((v as u32) as u64) << 32) as i64
}

fn num2(diff: u32) -> i64 {
    let hi = ((diff as i32) >> 16) as u32 as u64;
    let lo = diff.wrapping_shl(16) as u64;
    ((hi << 32) | lo) as i64
}

unsafe fn edge(to: *const i32, from: *const i32, len: i32, a: &mut i32, b: &mut i32, c: &mut i32) {
    let d = len.wrapping_shl(16) as i64;
    // SAFETY: to and from point at vertices of the game's quad table.
    unsafe {
        mech_trace::dw!(
            "10057ad5",
            a,
            div64(
                num16(
                    mech_trace::dr!("10057ad5", to) as u32,
                    mech_trace::dr!("10057ad5", from) as u32
                ),
                d,
                mech_trace::dr!("10057ad5", a)
            )
        );
        mech_trace::dw!(
            "10057ad5",
            b,
            div64(
                num16(
                    mech_trace::dr!("10057ad5", to.add(3)) as u32,
                    mech_trace::dr!("10057ad5", from.add(3)) as u32
                ),
                d,
                mech_trace::dr!("10057ad5", b)
            )
        );
        mech_trace::dw!(
            "10057ad5",
            c,
            div64(
                num16(
                    mech_trace::dr!("10057ad5", to.add(4)) as u32,
                    mech_trace::dr!("10057ad5", from.add(4)) as u32
                ),
                d,
                mech_trace::dr!("10057ad5", c)
            )
        );
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
#[allow(unused_assignments, unused_variables, unused_mut)]
pub unsafe extern "C" fn FUN_10057ad5(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: u32,
    param_8: i32,
    param_9: i32,
    param_10: u32,
) -> *mut i32 {
    mech_trace::enter!(
        "10057ad5", param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8,
        param_9, param_10
    );
    // SAFETY: the callees and the quad table are the game's own; all pointers
    // are the caller's surface descriptors and bitmap bytes, accessed where
    // the original does.
    unsafe {
        let g: *mut i32 = (&raw mut DAT_100b2c10) as *mut i32;
        let mut local_2c: [i32; 8] = [0; 8];
        let mut local_44: [i32; 2] = [0; 2];
        let mut local_3c: [i32; 2] = [0; 2];
        let mut local_34: [i32; 2] = [0; 2];

        if param_8 == 0x10000 && param_9 == 0x10000 && param_7 == 0 {
            if (param_10 & 1) == 0 {
                return mech_trace::rt!(
                    "10057ad5",
                    mech_trace::cr!(
                        "10057ad5",
                        "10056fc4",
                        FUN_10056fc4(param_1, param_2, param_3, param_4, param_5)
                    ) as u32 as usize as *mut i32
                );
            }
            return mech_trace::rt!(
                "10057ad5",
                mech_trace::cr!(
                    "10057ad5",
                    "100574ca",
                    FUN_100574ca(param_1, param_2, param_3, param_4, param_5)
                ) as u32 as usize as *mut i32
            );
        }

        let mut u_var5 = mech_trace::cr!("10057ad5", "1005b58c", FUN_1005b58c(param_2, param_3));
        let i_var6 = ((u_var5 >> 16).wrapping_sub(1)) as i32;
        let i_var14 = ((u_var5 & 0xffff).wrapping_sub(1)) as i32;
        let lp = local_2c.as_mut_ptr();
        local_2c[0] = (lp.add(5) as usize) as i32;
        local_2c[5] = param_6;
        local_2c[1] = 0;
        mech_trace::dw!("10057ad5", g.add(3), 0);
        mech_trace::dw!("10057ad5", g.add(18), 0);
        local_2c[2] = 0;
        mech_trace::dw!("10057ad5", g.add(4), 0);
        mech_trace::dw!("10057ad5", g.add(9), 0);
        mech_trace::dw!("10057ad5", g.add(8), i_var6);
        mech_trace::dw!("10057ad5", g.add(13), i_var6);
        mech_trace::dw!("10057ad5", g.add(14), i_var14);
        mech_trace::dw!("10057ad5", g.add(19), i_var14);
        local_2c[3] = i_var6;
        local_2c[4] = i_var14;
        local_2c[6] = i_var6;
        local_2c[7] = i_var14;
        let i_var7 = mech_trace::cr!("10057ad5", "1005b5c0", FUN_1005b5c0(param_2, param_3));
        local_44[1] = ((i_var7 as i16) as i32).wrapping_neg();
        local_44[0] = (i_var7 >> 16).wrapping_neg();
        if (param_10 & 2) == 0 {
            FUN_10058e74(lp, 0xff);
            if (param_10 & 1) == 0 {
                mech_trace::cr!(
                    "10057ad5",
                    "10056fc4",
                    FUN_10056fc4(lp, param_2, param_3, local_44[0], local_44[1])
                );
            } else {
                mech_trace::cr!(
                    "10057ad5",
                    "100574ca",
                    FUN_100574ca(lp, param_2, param_3, local_44[0], local_44[1])
                );
            }
        }

        let p_var25 = (mech_trace::dr!("10057ad5", param_1)) as u32 as usize as *mut i32;
        let mut i_var7 = mech_trace::dr!("10057ad5", p_var25.add(1));
        let i_var8 = i_var7.wrapping_add(1);
        if i_var8 == 0 || i_var7 < -1 {
            return mech_trace::rt!("10057ad5", 0xffff_ffff_u32 as usize as *mut i32);
        }
        let mut i_var12 = mech_trace::dr!("10057ad5", p_var25.add(2));
        if i_var12 < 0 {
            return mech_trace::rt!("10057ad5", 0xffff_ffff_u32 as usize as *mut i32);
        }
        let i_var2 = mech_trace::dr!("10057ad5", param_1.add(1));
        let mut i_var9 = i_var2;
        if i_var2 < 1 {
            i_var9 = 0;
        }
        let i_var21 = mech_trace::dr!("10057ad5", param_1.add(2));
        let mut i_var10 = i_var21;
        if i_var21 < 1 {
            i_var10 = 0;
        }
        let mut i_var11 = mech_trace::dr!("10057ad5", param_1.add(3));
        if i_var7 <= mech_trace::dr!("10057ad5", param_1.add(3)) {
            i_var11 = i_var7;
        }
        i_var7 = mech_trace::dr!("10057ad5", param_1.add(4));
        if i_var12 <= mech_trace::dr!("10057ad5", param_1.add(4)) {
            i_var7 = i_var12;
        }
        if i_var11 < i_var9 || i_var7 < i_var10 {
            return mech_trace::rt!("10057ad5", 0xffff_fffe_u32 as usize as *mut i32);
        }

        let mut local_60 = mech_trace::dr!("10057ad5", p_var25);
        i_var12 = param_4.wrapping_sub(local_44[0]);
        let i_var13 = param_5.wrapping_sub(local_44[1]);

        let p34 = local_34.as_mut_ptr();
        let p3c = local_3c.as_mut_ptr();
        let p44 = local_44.as_mut_ptr();

        local_34[0] = 0;
        local_34[1] = 0;
        FUN_1005aa1d(p34, p3c, p44, param_7, param_8, param_9);
        mech_trace::dw!("10057ad5", g, local_3c[0].wrapping_add(i_var12));
        mech_trace::dw!("10057ad5", g.add(1), local_3c[1].wrapping_add(i_var13));
        local_34[1] = 0;
        local_34[0] = i_var6;
        FUN_1005aa1d(p34, p3c, p44, param_7, param_8, param_9);
        mech_trace::dw!(
            "10057ad5",
            g,
            (mech_trace::dr!("10057ad5", g)).wrapping_add(i_var2)
        );
        mech_trace::dw!(
            "10057ad5",
            g.add(5),
            local_3c[0].wrapping_add(i_var12).wrapping_add(i_var2)
        );
        mech_trace::dw!(
            "10057ad5",
            g.add(1),
            (mech_trace::dr!("10057ad5", g.add(1))).wrapping_add(i_var21)
        );
        mech_trace::dw!(
            "10057ad5",
            g.add(6),
            local_3c[1].wrapping_add(i_var13).wrapping_add(i_var21)
        );
        local_34[0] = i_var6;
        local_34[1] = i_var14;
        FUN_1005aa1d(p34, p3c, p44, param_7, param_8, param_9);
        mech_trace::dw!("10057ad5", g.add(10), local_3c[0].wrapping_add(i_var12));
        mech_trace::dw!("10057ad5", g.add(11), local_3c[1].wrapping_add(i_var13));
        local_34[0] = 0;
        local_34[1] = i_var14;
        FUN_1005aa1d(p34, p3c, p44, param_7, param_8, param_9);
        mech_trace::dw!(
            "10057ad5",
            g.add(10),
            (mech_trace::dr!("10057ad5", g.add(10))).wrapping_add(i_var2)
        );
        mech_trace::dw!(
            "10057ad5",
            g.add(15),
            local_3c[0].wrapping_add(i_var12).wrapping_add(i_var2)
        );
        mech_trace::dw!(
            "10057ad5",
            g.add(11),
            (mech_trace::dr!("10057ad5", g.add(11))).wrapping_add(i_var21)
        );
        mech_trace::dw!(
            "10057ad5",
            g.add(16),
            local_3c[1].wrapping_add(i_var13).wrapping_add(i_var21)
        );

        local_2c[6] = local_2c[6].wrapping_add(1);

        let mut p_var21: *mut i32 = g;
        let mut local_90: i32 = 0x7fff;
        let mut i_var25: i32 = -0x8000;
        u_var5 = 0xf;
        let mut local_70: *mut i32 = g;
        let mut local_94: u32;
        loop {
            local_94 = mech_trace::dr!("10057ad5", p_var21.add(1)) as u32;
            if (local_94 as i32) <= local_90 {
                local_90 = local_94 as i32;
                local_70 = p_var21;
            }
            if i_var25 <= (local_94 as i32) {
                i_var25 = local_94 as i32;
            }
            let x = mech_trace::dr!("10057ad5", p_var21) as u32;
            let y = mech_trace::dr!("10057ad5", p_var21.add(1)) as u32;
            u_var5 &= ((((((x.wrapping_sub(i_var9 as u32)) >> 31) << 1
                | ((i_var11 as u32).wrapping_sub(x) >> 31))
                << 1
                | (y.wrapping_sub(i_var10 as u32) >> 31))
                << 1)
                | ((i_var7 as u32).wrapping_sub(y) >> 31));
            p_var21 = p_var21.wrapping_add(5);
            if p_var21 == g.add(20) {
                break;
            }
        }

        if u_var5 != 0 {
            return mech_trace::rt!("10057ad5", local_94 as usize as *mut i32);
        }

        let mut local_7c = local_70;
        let mut local_80 = local_70;
        local_94 = local_70 as usize as u32;
        p_var21 = local_7c;
        if i_var25 == local_90 {
            return mech_trace::rt!("10057ad5", local_94 as usize as *mut i32);
        }

        let mut local_84: i32 = 0;
        let mut local_88: i32 = 0;
        let mut local_ac: i32 = 0;
        let mut local_b0: i32 = 0;
        let mut local_b4: i32 = 0;
        let mut local_b8: i32 = 0;
        let mut local_bc: i32 = 0;
        let mut local_c0: i32 = 0;

        loop {
            local_7c = p_var21;
            p_var21 = local_7c.wrapping_sub(5);
            if (p_var21 as usize as i32) < (g as usize as i32) {
                p_var21 = g.add(15);
            }
            if mech_trace::dr!("10057ad5", local_7c.add(1)) < i_var10
                && mech_trace::dr!("10057ad5", p_var21.add(1)) <= i_var10
            {
                continue;
            }
            local_84 = (mech_trace::dr!("10057ad5", p_var21.add(1)))
                .wrapping_sub(mech_trace::dr!("10057ad5", local_7c.add(1)));
            if local_84 == 0 {
                continue;
            }
            break;
        }
        edge(
            p_var21,
            local_7c,
            local_84,
            &mut local_ac,
            &mut local_b4,
            &mut local_bc,
        );
        local_94 = (mech_trace::dr!("10057ad5", local_7c) as u32)
            .wrapping_mul(0x10000)
            .wrapping_add(0x8000);
        let mut local_9c = (mech_trace::dr!("10057ad5", local_7c.add(3)) as u32)
            .wrapping_mul(0x10000)
            .wrapping_add(0x8000);
        let mut local_a4 = (mech_trace::dr!("10057ad5", local_7c.add(4)) as u32)
            .wrapping_mul(0x10000)
            .wrapping_add(0x8000);

        let mut p_var23 = local_80;
        loop {
            local_80 = p_var23;
            p_var23 = local_80.wrapping_add(5);
            if ((g.add(20) as usize as i32).wrapping_sub(1)) < (p_var23 as usize as i32) {
                p_var23 = g;
            }
            if mech_trace::dr!("10057ad5", local_80.add(1)) < i_var10
                && mech_trace::dr!("10057ad5", p_var23.add(1)) <= i_var10
            {
                continue;
            }
            local_88 = (mech_trace::dr!("10057ad5", p_var23.add(1)))
                .wrapping_sub(mech_trace::dr!("10057ad5", local_80.add(1)));
            if local_88 == 0 {
                continue;
            }
            break;
        }
        edge(
            p_var23,
            local_80,
            local_88,
            &mut local_b0,
            &mut local_b8,
            &mut local_c0,
        );
        let mut local_98 = (mech_trace::dr!("10057ad5", local_80) as u32)
            .wrapping_mul(0x10000)
            .wrapping_add(0x8000);
        let mut local_a0 = (mech_trace::dr!("10057ad5", local_80.add(3)) as u32)
            .wrapping_mul(0x10000)
            .wrapping_add(0x8000);
        let mut local_a8 = (mech_trace::dr!("10057ad5", local_80.add(4)) as u32)
            .wrapping_mul(0x10000)
            .wrapping_add(0x8000);

        let mut local_8c = i_var7.wrapping_sub(local_90);
        if i_var25.wrapping_sub(i_var7) == 0 || i_var25 < i_var7 {
            local_8c = local_8c.wrapping_add(i_var25.wrapping_sub(i_var7));
        }
        if i_var10.wrapping_sub(local_90) != 0 && local_90 <= i_var10 {
            local_8c = local_8c.wrapping_sub(i_var10.wrapping_sub(local_90));
            local_84 = local_84
                .wrapping_sub(i_var10.wrapping_sub(mech_trace::dr!("10057ad5", local_7c.add(1))));
            let i6 = i_var10
                .wrapping_sub(mech_trace::dr!("10057ad5", local_7c.add(1)))
                .wrapping_shl(16);
            let l = (local_ac as i64).wrapping_mul(i6 as i64);
            local_94 = local_94.wrapping_add(((l as u64) >> 16) as u32);
            let l = (local_b4 as i64).wrapping_mul(i6 as i64);
            local_9c = local_9c.wrapping_add(((l as u64) >> 16) as u32);
            let l = (local_bc as i64).wrapping_mul(i6 as i64);
            local_a4 = local_a4.wrapping_add(((l as u64) >> 16) as u32);
            local_88 = local_88
                .wrapping_sub(i_var10.wrapping_sub(mech_trace::dr!("10057ad5", local_80.add(1))));
            let i6 = i_var10
                .wrapping_sub(mech_trace::dr!("10057ad5", local_80.add(1)))
                .wrapping_shl(16);
            let l = (local_b0 as i64).wrapping_mul(i6 as i64);
            local_98 = local_98.wrapping_add(((l as u64) >> 16) as u32);
            let l = (local_b8 as i64).wrapping_mul(i6 as i64);
            local_a0 = local_a0.wrapping_add(((l as u64) >> 16) as u32);
            let l = (local_c0 as i64).wrapping_mul(i6 as i64);
            local_a8 = local_a8.wrapping_add(((l as u64) >> 16) as u32);
            local_90 = i_var10;
        }
        local_60 = (local_90 as u32)
            .wrapping_mul(i_var8 as u32)
            .wrapping_add(local_60 as u32) as i32;
        local_80 = p_var23;
        local_7c = p_var21;

        let mut local_cc: u32 = 0;
        let mut local_d0: u32 = 0;

        loop {
            let mut xl = local_94 as i32;
            let mut local_d4 = local_9c;
            let mut u_var17 = local_a0;
            let mut xr = local_98 as i32;
            let mut u5 = local_a4;
            let mut u_var22 = local_a8;
            if (local_98 as i32) <= (local_94 as i32) {
                xl = local_98 as i32;
                local_d4 = local_a0;
                u_var17 = local_9c;
                xr = local_94 as i32;
                u5 = local_a8;
                u_var22 = local_a4;
            }
            let mut local_c4 = xl >> 16;
            let mut local_c8: i32 = 0;
            if local_c4 <= i_var11 && {
                local_c8 = xr >> 16;
                i_var9 <= local_c8
            } {
                let i6 = local_c8.wrapping_sub(local_c4);
                if i6 != 0 {
                    let ll_d = i6.wrapping_shl(16) as i64;
                    let diff = u_var17.wrapping_sub(local_d4);
                    local_cc = div64(num2(diff), ll_d, local_cc as i32) as u32;
                    let mut u17b = local_cc >> 16;
                    let mut i7: i32 = 1;
                    if (u17b & 0x8000) != 0 {
                        i7 = -1;
                        u17b = (u17b | 0xffff_0000)
                            .wrapping_add(1)
                            .wrapping_sub(((local_cc & 0xffff) == 0) as u32);
                    }
                    let diff = u_var22.wrapping_sub(u5);
                    local_d0 = div64(num2(diff), ll_d, local_d0 as i32) as u32;
                    let mut s_var16 = (local_d0 >> 16) as i16;
                    mech_trace::dw!("10057ad5", g.add(23), local_2c[6]);
                    if ((local_d0 >> 16) & 0x8000) != 0 {
                        mech_trace::dw!("10057ad5", g.add(23), 0i32.wrapping_sub(local_2c[6]));
                        s_var16 = ((s_var16 as i32)
                            .wrapping_add(1)
                            .wrapping_sub(((local_d0 & 0xffff) == 0) as i32))
                            as i16;
                    }
                    mech_trace::dw!(
                        "10057ad5",
                        g.add(22),
                        ((local_2c[6] as i16 as i32).wrapping_mul(s_var16 as i32)) as i16 as i32
                    );
                    mech_trace::dw!(
                        "10057ad5",
                        g.add(20),
                        u17b.wrapping_add(mech_trace::dr!("10057ad5", g.add(22)) as u32) as i32
                    );
                    mech_trace::dw!(
                        "10057ad5",
                        g.add(21),
                        (mech_trace::dr!("10057ad5", g.add(20)) as u32)
                            .wrapping_add(mech_trace::dr!("10057ad5", g.add(23)) as u32)
                            as i32
                    );
                    mech_trace::dw!(
                        "10057ad5",
                        g.add(22),
                        (i7 as u32).wrapping_add(u17b).wrapping_add(mech_trace::dr!(
                            "10057ad5",
                            g.add(22)
                        )
                            as u32) as i32
                    );
                    mech_trace::dw!(
                        "10057ad5",
                        g.add(23),
                        (mech_trace::dr!("10057ad5", g.add(22)) as u32)
                            .wrapping_add(mech_trace::dr!("10057ad5", g.add(23)) as u32)
                            as i32
                    );
                    let i6 = i_var9.wrapping_sub(local_c4);
                    if i6 != 0 && local_c4 <= i_var9 {
                        local_c4 = local_c4.wrapping_add(i6);
                        let l = (local_cc as i32 as i64).wrapping_mul(i6.wrapping_shl(16) as i64);
                        local_d4 = local_d4.wrapping_add(((l as u64) >> 16) as u32);
                        let l = (local_d0 as i32 as i64).wrapping_mul(i6.wrapping_shl(16) as i64);
                        u5 = u5.wrapping_add(((l as u64) >> 16) as u32);
                    }
                    if local_c8.wrapping_sub(i_var11) != 0 && i_var11 <= local_c8 {
                        local_c8 = local_c8.wrapping_sub(local_c8.wrapping_sub(i_var11));
                    }
                }
                let mut p24 = ((local_d4 >> 16)
                    .wrapping_add((u5 >> 16).wrapping_mul(local_2c[6] as u32))
                    .wrapping_add(local_2c[5] as u32)) as usize
                    as *const i8;
                let mut p26 = ((local_60 as u32).wrapping_add(local_c4 as u32)) as usize as *mut i8;
                local_c8 = local_c8.wrapping_sub(local_c4);
                u_var17 = local_cc;
                if (local_cc as i32) < 0 {
                    u_var17 = 0u32.wrapping_sub(local_cc);
                    local_d4 = !local_d4;
                }
                u_var17 = u_var17.wrapping_shl(16);
                local_d4 = local_d4.wrapping_shl(16);
                u_var22 = local_d0;
                if (local_d0 as i32) < 0 {
                    u_var22 = 0u32.wrapping_sub(local_d0);
                    u5 = !u5;
                }
                u_var22 = u_var22.wrapping_shl(16);
                u5 = u5.wrapping_shl(16);

                macro_rules! step {
                    () => {
                        let car = (((local_d4.wrapping_add(u_var17) < local_d4) as u32) << 1)
                            | ((u5.wrapping_add(u_var22) < u5) as u32);
                        local_d4 = local_d4.wrapping_add(u_var17);
                        u5 = u5.wrapping_add(u_var22);
                        p24 = p24.wrapping_offset(*g.add(20 + car as usize) as isize);
                    };
                }
                macro_rules! put {
                    ($k:expr) => {
                        if *p24 != -1 {
                            *p26.add($k) = *p24;
                        }
                    };
                }

                loop {
                    if local_c8 < 5 {
                        put!(0);
                        step!();
                        if -1 < local_c8.wrapping_sub(1) {
                            put!(1);
                            step!();
                            if -1 < local_c8.wrapping_sub(2) {
                                put!(2);
                                step!();
                                if -1 < local_c8.wrapping_sub(3) {
                                    put!(3);
                                    step!();
                                    if -1 < local_c8.wrapping_sub(4) {
                                        put!(4);
                                    }
                                }
                            }
                        }
                        break;
                    }
                    put!(0);
                    step!();
                    put!(1);
                    step!();
                    put!(2);
                    step!();
                    put!(3);
                    step!();
                    put!(4);
                    step!();
                    put!(5);
                    step!();
                    p26 = p26.add(6);
                    local_c8 = local_c8.wrapping_sub(6);
                    if !(-1 < local_c8) {
                        break;
                    }
                }
            }

            local_60 = local_60.wrapping_add(i_var8);
            local_8c = local_8c.wrapping_sub(1);
            if local_8c < 0 {
                break;
            }
            if local_8c == 0 {
                local_94 = local_94.wrapping_add(local_ac as u32);
                local_9c = local_9c.wrapping_add(local_b4 as u32);
                local_a4 = local_a4.wrapping_add(local_bc as u32);
                local_98 = local_98.wrapping_add(local_b0 as u32);
                local_a0 = local_a0.wrapping_add(local_b8 as u32);
                local_a8 = local_a8.wrapping_add(local_c0 as u32);
            } else {
                local_84 = local_84.wrapping_sub(1);
                if local_84 == 0 {
                    let mut p25 = local_7c.wrapping_sub(5);
                    if (p25 as usize as i32) < (g as usize as i32) {
                        p25 = g.add(15);
                    }
                    let dy = (mech_trace::dr!("10057ad5", p25.add(1)))
                        .wrapping_sub(mech_trace::dr!("10057ad5", local_7c.add(1)));
                    local_84 = dy.wrapping_add((dy == 0) as i32);
                    edge(
                        p25,
                        local_7c,
                        local_84,
                        &mut local_ac,
                        &mut local_b4,
                        &mut local_bc,
                    );
                    local_94 = (mech_trace::dr!("10057ad5", local_7c) as u32)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000);
                    local_9c = (mech_trace::dr!("10057ad5", local_7c.add(3)) as u32)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000);
                    local_a4 = (mech_trace::dr!("10057ad5", local_7c.add(4)) as u32)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000);
                    local_7c = p25;
                } else {
                    local_94 = local_94.wrapping_add(local_ac as u32);
                    local_9c = local_9c.wrapping_add(local_b4 as u32);
                    local_a4 = local_a4.wrapping_add(local_bc as u32);
                }
                local_88 = local_88.wrapping_sub(1);
                if local_88 == 0 {
                    let mut p25 = local_80.wrapping_add(5);
                    if ((g.add(20) as usize as i32).wrapping_sub(1)) < (p25 as usize as i32) {
                        p25 = g;
                    }
                    let dy = (mech_trace::dr!("10057ad5", p25.add(1)))
                        .wrapping_sub(mech_trace::dr!("10057ad5", local_80.add(1)));
                    local_88 = dy.wrapping_add((dy == 0) as i32);
                    edge(
                        p25,
                        local_80,
                        local_88,
                        &mut local_b0,
                        &mut local_b8,
                        &mut local_c0,
                    );
                    local_a0 = (mech_trace::dr!("10057ad5", local_80.add(3)) as u32)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000);
                    local_a8 = (mech_trace::dr!("10057ad5", local_80.add(4)) as u32)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000);
                    local_98 = (mech_trace::dr!("10057ad5", local_80) as u32)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000);
                    local_80 = p25;
                } else {
                    local_98 = local_98.wrapping_add(local_b0 as u32);
                    local_a0 = local_a0.wrapping_add(local_b8 as u32);
                    local_a8 = local_a8.wrapping_add(local_c0 as u32);
                }
            }
        }
        mech_trace::rt!("10057ad5", local_94 as usize as *mut i32)
    }
}
