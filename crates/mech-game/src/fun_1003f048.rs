// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af318: i32;
    static mut DAT_100af31c: i32;
    static mut DAT_100af320: i32;
    static mut DAT_100af324: i32;
    static mut DAT_100af334: *mut i8;
    static mut DAT_100af338: u32;
    static mut DAT_100af33c: *mut u32;
    static mut DAT_100af340: *mut u32;
    static mut DAT_100af344: *mut u32;
    static mut DAT_100af348: *mut u32;
    static mut DAT_100af34c: *mut u32;
    static mut DAT_100af350: *mut u32;
    static mut DAT_100af354: *mut u32;
    static mut DAT_100af358: i32;
    static mut DAT_100af35c: i32;
    static mut DAT_100af360: i32;
    static mut DAT_100af364: u32;
    static mut DAT_100af368: i32;
    static mut DAT_100af36c: i32;
    static mut DAT_100af370: u32;
    static mut DAT_100af374: u32;
    static mut DAT_100af388: i32;
    static mut DAT_100af38c: i32;
    static mut DAT_100af390: i32;
    static mut DAT_100af394: i32;
    static mut DAT_100af3a8: i32;
    static mut DAT_100af3ac: i32;
    static mut DAT_100af3b0: i32;
}

unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

unsafe fn ru(p: *const u32, i: usize) -> u32 {
    // SAFETY: caller supplies a valid vertex pointer.
    unsafe { mech_trace::rd!("1003f048", p.add(i), read_unaligned) }
}

unsafe fn dv(num: i64, den: i32, dst: *mut i32) {
    // SAFETY: dst is a valid global or local; RaiseException is the OS API.
    unsafe {
        if den == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        } else {
            let q = num / (den as i64);
            if q != (q as i32 as i64) {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
            } else {
                mech_trace::wr!("1003f048", dst, write_unaligned, q as i32);
            }
        }
    }
}

unsafe fn fill(mut p: *mut u8, mut c: u32, step: u32, mut k: i32) -> u32 {
    // SAFETY: p points into the caller's destination surface.
    unsafe {
        loop {
            if k < 5 {
                let mut n = 0;
                loop {
                    mech_trace::wr!("1003f048", p.byte_add(n), write, ((c >> 16) as u8));
                    c = c.wrapping_add(step);
                    mech_trace::wr!("1003f048", p.byte_add(n + 1), write, ((c >> 16) as u8));
                    c = c.wrapping_add(step);
                    n += 2;
                    if n >= 10 || k.wrapping_sub((n / 2) as i32) < 0 {
                        break;
                    }
                }
                break;
            }
            for n in 0..12 {
                mech_trace::wr!("1003f048", p.byte_add(n), write, ((c >> 16) as u8));
                c = c.wrapping_add(step);
            }
            p = p.byte_add(12);
            k = k.wrapping_sub(6);
            if !(k >= 0) {
                break;
            }
        }
    }
    c
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003f048(param_1: *mut i32, param_2: i32, param_3: *mut u32) {
    mech_trace::enter!("1003f048", param_1, param_2, param_3);
    // SAFETY: all globals are the game's own memory; param pointers are the
    // caller's valid data; surface writes land where the original writes.
    unsafe {
        let pi1 =
            (mech_trace::rd!("1003f048", param_1, read_unaligned) as u32 as usize) as *mut i32;
        let p1_1 = mech_trace::rd!("1003f048", pi1.add(1), read_unaligned);
        let p1_2 = mech_trace::rd!("1003f048", pi1.add(2), read_unaligned);
        let a1 = mech_trace::rd!("1003f048", param_1.add(1), read_unaligned);
        let a2 = mech_trace::rd!("1003f048", param_1.add(2), read_unaligned);
        let a3 = mech_trace::rd!("1003f048", param_1.add(3), read_unaligned);
        let a4 = mech_trace::rd!("1003f048", param_1.add(4), read_unaligned);

        mech_trace::gw!(
            "1003f048",
            "100af324",
            DAT_100af324,
            (p1_1 as u32).wrapping_add(1) as i32
        );
        let mut i8_ = p1_1;
        if a3 <= p1_1 {
            i8_ = a3;
        }
        let mut u17: u32 = 0;
        if (a1 as u32) < 0x80000000 {
            u17 = a1 as u32;
        }
        let mut i5 = (i8_ as u32).wrapping_sub(u17) as i32;
        if (u17 as i32) > i8_ {
            return;
        }
        i8_ = p1_2;
        if a4 <= p1_2 {
            i8_ = a4;
        }
        let mut u20: u32 = 0;
        if (a2 as u32) < 0x80000000 {
            u20 = a2 as u32;
        }
        let mut i6 = (i8_ as u32).wrapping_sub(u20) as i32;
        mech_trace::gw!("1003f048", "100af318", DAT_100af318, i5);
        if (u20 as i32) > i8_ {
            return;
        }

        mech_trace::gw!(
            "1003f048",
            "100af320",
            DAT_100af320,
            u20.wrapping_mul(mech_trace::gr!("1003f048", "100af324", DAT_100af324) as u32)
                .wrapping_add(u17)
                .wrapping_add(mech_trace::rd!("1003f048", pi1, read_unaligned) as u32)
                as i32
        );
        mech_trace::gw!(
            "1003f048",
            "100af340",
            DAT_100af340,
            param_3.wrapping_offset(param_2.wrapping_mul(6) as isize)
        );
        mech_trace::gw!("1003f048", "100af33c", DAT_100af33c, param_3);
        mech_trace::gw!("1003f048", "100af338", DAT_100af338, 0);
        u17 = 0x7fff;
        u20 = 0xffff8000;
        let mut u7: u32 = 0xf;
        let mut p12 = param_3;
        loop {
            let u11 = ((ru(p12, 0) >> 0x1f) << 1) | ((i5 as u32).wrapping_sub(ru(p12, 0)) >> 0x1f);
            mech_trace::gu!("1003f048", "100af338", DAT_100af338, |=, u11);
            let u15 = ru(p12, 1);
            if (u15 as i32) <= (u17 as i32) {
                u17 = u15;
                mech_trace::gw!("1003f048", "100af344", DAT_100af344, p12);
            }
            if (u20 as i32) <= (u15 as i32) {
                u20 = u15;
            }
            u7 &= (((u11 << 1) | (ru(p12, 1) >> 0x1f)) << 1)
                | ((i6 as u32).wrapping_sub(ru(p12, 1)) >> 0x1f);
            p12 = p12.wrapping_add(6);
            if p12 == mech_trace::gr!("1003f048", "100af340", DAT_100af340) {
                break;
            }
        }
        mech_trace::gw!("1003f048", "100af31c", DAT_100af31c, i6);
        if u7 != 0 {
            return;
        }

        mech_trace::gw!(
            "1003f048",
            "100af350",
            DAT_100af350,
            mech_trace::gr!("1003f048", "100af344", DAT_100af344)
        );
        mech_trace::gw!(
            "1003f048",
            "100af354",
            DAT_100af354,
            mech_trace::gr!("1003f048", "100af344", DAT_100af344)
        );
        p12 = mech_trace::gr!("1003f048", "100af350", DAT_100af350);
        mech_trace::gw!("1003f048", "100af364", DAT_100af364, u17);
        if u20 == u17 {
            return;
        }

        // first left edge
        loop {
            mech_trace::gw!("1003f048", "100af350", DAT_100af350, p12);
            p12 = mech_trace::gr!("1003f048", "100af350", DAT_100af350).wrapping_sub(6);
            if ((p12 as usize) as i32) < ((param_3 as usize) as i32) {
                p12 = mech_trace::gr!("1003f048", "100af340", DAT_100af340).wrapping_sub(6);
            }
            let cond = ((ru(mech_trace::gr!("1003f048", "100af350", DAT_100af350), 1) as i32) < 0
                && (ru(p12, 1) as i32) < 1)
                || {
                    mech_trace::gw!(
                        "1003f048",
                        "100af358",
                        DAT_100af358,
                        ru(p12, 1).wrapping_sub(ru(
                            mech_trace::gr!("1003f048", "100af350", DAT_100af350),
                            1
                        )) as i32
                    );
                    mech_trace::gr!("1003f048", "100af358", DAT_100af358) == 0
                };
            if !cond {
                break;
            }
        }

        let num = (ru(p12, 0)
            .wrapping_sub(ru(mech_trace::gr!("1003f048", "100af350", DAT_100af350), 0))
            as i16 as i32 as i64)
            * 4294967296i64;
        let den =
            (mech_trace::gr!("1003f048", "100af358", DAT_100af358) as u32).wrapping_shl(16) as i32;
        dv(num, den, &raw mut DAT_100af388);
        let num = (ru(p12, 2)
            .wrapping_sub(ru(mech_trace::gr!("1003f048", "100af350", DAT_100af350), 2))
            as i32 as i64)
            * 65536i64;
        let den =
            (mech_trace::gr!("1003f048", "100af358", DAT_100af358) as u32).wrapping_shl(16) as i32;
        dv(num, den, &raw mut DAT_100af390);
        mech_trace::gw!(
            "1003f048",
            "100af368",
            DAT_100af368,
            ru(mech_trace::gr!("1003f048", "100af350", DAT_100af350), 0)
                .wrapping_mul(0x10000)
                .wrapping_add(0x8000) as i32
        );
        mech_trace::gw!(
            "1003f048",
            "100af370",
            DAT_100af370,
            ru(mech_trace::gr!("1003f048", "100af350", DAT_100af350), 2).wrapping_add(0x8000)
        );

        // first right edge
        let mut p18 = mech_trace::gr!("1003f048", "100af354", DAT_100af354);
        loop {
            mech_trace::gw!("1003f048", "100af354", DAT_100af354, p18);
            p18 = mech_trace::gr!("1003f048", "100af354", DAT_100af354).wrapping_add(6);
            if ((mech_trace::gr!("1003f048", "100af340", DAT_100af340) as usize) as i32)
                <= ((p18 as usize) as i32)
            {
                p18 = param_3;
            }
            let cond = ((ru(mech_trace::gr!("1003f048", "100af354", DAT_100af354), 1) as i32) < 0
                && (ru(p18, 1) as i32) < 1)
                || {
                    mech_trace::gw!(
                        "1003f048",
                        "100af35c",
                        DAT_100af35c,
                        ru(p18, 1).wrapping_sub(ru(
                            mech_trace::gr!("1003f048", "100af354", DAT_100af354),
                            1
                        )) as i32
                    );
                    mech_trace::gr!("1003f048", "100af35c", DAT_100af35c) == 0
                };
            if !cond {
                break;
            }
        }

        let num = (ru(p18, 0)
            .wrapping_sub(ru(mech_trace::gr!("1003f048", "100af354", DAT_100af354), 0))
            as i16 as i32 as i64)
            * 4294967296i64;
        let den =
            (mech_trace::gr!("1003f048", "100af35c", DAT_100af35c) as u32).wrapping_shl(16) as i32;
        dv(num, den, &raw mut DAT_100af38c);
        let num = (ru(p18, 2)
            .wrapping_sub(ru(mech_trace::gr!("1003f048", "100af354", DAT_100af354), 2))
            as i32 as i64)
            * 65536i64;
        let den =
            (mech_trace::gr!("1003f048", "100af35c", DAT_100af35c) as u32).wrapping_shl(16) as i32;
        dv(num, den, &raw mut DAT_100af394);
        mech_trace::gw!(
            "1003f048",
            "100af36c",
            DAT_100af36c,
            ru(mech_trace::gr!("1003f048", "100af354", DAT_100af354), 0)
                .wrapping_mul(0x10000)
                .wrapping_add(0x8000) as i32
        );
        mech_trace::gw!(
            "1003f048",
            "100af374",
            DAT_100af374,
            ru(mech_trace::gr!("1003f048", "100af354", DAT_100af354), 2).wrapping_add(0x8000)
        );

        mech_trace::gw!(
            "1003f048",
            "100af360",
            DAT_100af360,
            (i6 as u32).wrapping_sub(u17) as i32
        );
        if u20.wrapping_sub(i6 as u32) == 0 || (u20 as i32) < i6 {
            mech_trace::gw!(
                "1003f048",
                "100af360",
                DAT_100af360,
                (mech_trace::gr!("1003f048", "100af360", DAT_100af360) as u32)
                    .wrapping_add(u20.wrapping_sub(i6 as u32)) as i32
            );
        }
        if 0x7fffffff < u17 {
            mech_trace::gw!(
                "1003f048",
                "100af360",
                DAT_100af360,
                (mech_trace::gr!("1003f048", "100af360", DAT_100af360) as u32).wrapping_add(u17)
                    as i32
            );
            mech_trace::gw!("1003f048", "100af364", DAT_100af364, 0);
            mech_trace::gw!(
                "1003f048",
                "100af358",
                DAT_100af358,
                (mech_trace::gr!("1003f048", "100af358", DAT_100af358) as u32)
                    .wrapping_add(ru(mech_trace::gr!("1003f048", "100af350", DAT_100af350), 1))
                    as i32
            );
            i8_ = ru(mech_trace::gr!("1003f048", "100af350", DAT_100af350), 1)
                .wrapping_mul(0xffff0000u32) as i32;
            let l = (mech_trace::gr!("1003f048", "100af388", DAT_100af388) as i64)
                .wrapping_mul(i8_ as i64);
            mech_trace::gw!(
                "1003f048",
                "100af368",
                DAT_100af368,
                (mech_trace::gr!("1003f048", "100af368", DAT_100af368) as u32)
                    .wrapping_add(((l as u64) >> 16) as u32) as i32
            );
            let l = (mech_trace::gr!("1003f048", "100af390", DAT_100af390) as i64)
                .wrapping_mul(i8_ as i64);
            mech_trace::gw!(
                "1003f048",
                "100af370",
                DAT_100af370,
                mech_trace::gr!("1003f048", "100af370", DAT_100af370)
                    .wrapping_add(((l as u64) >> 16) as u32)
            );
            mech_trace::gw!(
                "1003f048",
                "100af35c",
                DAT_100af35c,
                (mech_trace::gr!("1003f048", "100af35c", DAT_100af35c) as u32)
                    .wrapping_add(ru(mech_trace::gr!("1003f048", "100af354", DAT_100af354), 1))
                    as i32
            );
            i8_ = ru(mech_trace::gr!("1003f048", "100af354", DAT_100af354), 1)
                .wrapping_mul(0xffff0000u32) as i32;
            let l = (mech_trace::gr!("1003f048", "100af38c", DAT_100af38c) as i64)
                .wrapping_mul(i8_ as i64);
            mech_trace::gw!(
                "1003f048",
                "100af36c",
                DAT_100af36c,
                (mech_trace::gr!("1003f048", "100af36c", DAT_100af36c) as u32)
                    .wrapping_add(((l as u64) >> 16) as u32) as i32
            );
            let l = (mech_trace::gr!("1003f048", "100af394", DAT_100af394) as i64)
                .wrapping_mul(i8_ as i64);
            mech_trace::gw!(
                "1003f048",
                "100af374",
                DAT_100af374,
                mech_trace::gr!("1003f048", "100af374", DAT_100af374)
                    .wrapping_add(((l as u64) >> 16) as u32)
            );
        }
        i8_ =
            (mech_trace::gr!("1003f048", "100af320", DAT_100af320) as u32).wrapping_add(
                mech_trace::gr!("1003f048", "100af364", DAT_100af364)
                    .wrapping_mul(mech_trace::gr!("1003f048", "100af324", DAT_100af324) as u32),
            ) as i32;
        mech_trace::gw!(
            "1003f048",
            "100af348",
            DAT_100af348,
            mech_trace::gr!("1003f048", "100af350", DAT_100af350)
        );
        mech_trace::gw!(
            "1003f048",
            "100af34c",
            DAT_100af34c,
            mech_trace::gr!("1003f048", "100af354", DAT_100af354)
        );
        mech_trace::gw!("1003f048", "100af354", DAT_100af354, p18);
        mech_trace::gw!("1003f048", "100af350", DAT_100af350, p12);
        i5 = mech_trace::gr!("1003f048", "100af368", DAT_100af368);
        i6 = mech_trace::gr!("1003f048", "100af36c", DAT_100af36c);
        u17 = mech_trace::gr!("1003f048", "100af370", DAT_100af370);
        u20 = mech_trace::gr!("1003f048", "100af374", DAT_100af374);

        loop {
            if mech_trace::gr!("1003f048", "100af338", DAT_100af338) == 0 {
                'done: {
                    let mut i19 = i5;
                    let mut u15 = u17;
                    u7 = u20;
                    let mut i16 = i6;
                    if i6 <= i5 {
                        i19 = i6;
                        u15 = u20;
                        u7 = u17;
                        i16 = i5;
                    }
                    let mut u11 = (i19 >> 0x10) as u32;
                    let mut p21 = (i8_ as u32).wrapping_add(u11) as usize as *mut u8;
                    let u13 = ((i16 >> 0x10) as u32).wrapping_sub(u11);
                    if u13 != 0 {
                        let num = (u7.wrapping_sub(u15) as i32 as i64) * 65536i64;
                        let den = u13.wrapping_shl(16) as i32;
                        let mut t = u11 as i32;
                        dv(num, den, &mut t);
                        u11 = t as u32;
                    }
                    mech_trace::gw!(
                        "1003f048",
                        "100af334",
                        DAT_100af334,
                        p21.wrapping_add(u13 as usize) as *mut i8
                    );
                    let mut u14 = u15;
                    u7 = u13.wrapping_add(1);
                    if ((p21 as usize as u32) & 1) != 0 {
                        mech_trace::wr!("1003f048", p21, write, (u14 >> 16) as u8);
                        if u13 == 0 {
                            break 'done;
                        }
                        p21 = p21.wrapping_add(1);
                        u14 = u14.wrapping_add(u11);
                        u7 = u13;
                    }
                    if u7 != 1 {
                        let k = ((u7 >> 1) as i32).wrapping_sub(1);
                        u14 = fill(p21, u14, u11, k);
                        if (u7 & 1) == 0 {
                            break 'done;
                        }
                    }
                    mech_trace::wr!(
                        "1003f048",
                        mech_trace::gr!("1003f048", "100af334", DAT_100af334),
                        write,
                        (u14 >> 16) as i8
                    );
                }
            } else {
                'cdone: {
                    let mut i19 = i5;
                    u7 = u17;
                    let mut u15 = u20;
                    let mut i16 = i6;
                    if i6 <= i5 {
                        i19 = i6;
                        u7 = u20;
                        u15 = u17;
                        i16 = i5;
                    }
                    i19 >>= 0x10;
                    if i19 <= mech_trace::gr!("1003f048", "100af318", DAT_100af318) && {
                        i16 >>= 0x10;
                        i16 > -1
                    } {
                        let mut p21 = (i8_ as u32).wrapping_add(i19 as u32) as usize as *mut u8;
                        let mut u11 = (i16 as u32).wrapping_sub(i19 as u32).wrapping_add(1);
                        if u11 != 1 {
                            let num = (u15.wrapping_sub(u7) as i32 as i64) * 65536i64;
                            let den = (i16 as u32).wrapping_sub(i19 as u32).wrapping_shl(16) as i32;
                            dv(num, den, &raw mut DAT_100af3b0);
                        }
                        if i19 < 0 {
                            p21 =
                                (p21 as usize as u32).wrapping_sub(i19 as u32) as usize as *mut u8;
                            u11 = u11.wrapping_add(i19 as u32);
                            let l = ((i19 as u32).wrapping_mul(0xffff0000u32) as i32 as i64)
                                .wrapping_mul(
                                    mech_trace::gr!("1003f048", "100af3b0", DAT_100af3b0) as i64,
                                );
                            u7 = u7.wrapping_add(((l as u64) >> 16) as u32);
                        }
                        if i16 > mech_trace::gr!("1003f048", "100af318", DAT_100af318) {
                            u11 = u11.wrapping_sub((i16 as u32).wrapping_sub(mech_trace::gr!(
                                "1003f048",
                                "100af318",
                                DAT_100af318
                            )
                                as u32));
                        }
                        mech_trace::gw!(
                            "1003f048",
                            "100af334",
                            DAT_100af334,
                            p21.wrapping_add(u11.wrapping_sub(1) as usize) as *mut i8
                        );
                        mech_trace::gw!("1003f048", "100af3a8", DAT_100af3a8, i19);
                        mech_trace::gw!("1003f048", "100af3ac", DAT_100af3ac, i16);
                        let u13 = mech_trace::gr!("1003f048", "100af3b0", DAT_100af3b0) as u32;
                        let mut u14 = u7;
                        if ((p21 as usize as u32) & 1) != 0 {
                            mech_trace::wr!("1003f048", p21, write, (u14 >> 16) as u8);
                            u11 = u11.wrapping_sub(1);
                            if u11 == 0 {
                                break 'cdone;
                            }
                            p21 = p21.wrapping_add(1);
                            u14 = u14.wrapping_add(u13);
                        }
                        if u11 != 1 {
                            let k = ((u11 >> 1) as i32).wrapping_sub(1);
                            u14 = fill(p21, u14, u13, k);
                            if (u11 & 1) == 0 {
                                break 'cdone;
                            }
                        }
                        mech_trace::wr!(
                            "1003f048",
                            mech_trace::gr!("1003f048", "100af334", DAT_100af334),
                            write,
                            (u14 >> 16) as i8
                        );
                    }
                }
            }

            // next row
            i8_ = (i8_ as u32)
                .wrapping_add(mech_trace::gr!("1003f048", "100af324", DAT_100af324) as u32)
                as i32;
            mech_trace::gw!(
                "1003f048",
                "100af360",
                DAT_100af360,
                mech_trace::gr!("1003f048", "100af360", DAT_100af360).wrapping_sub(1)
            );
            if mech_trace::gr!("1003f048", "100af360", DAT_100af360) < 0 {
                return;
            }
            if mech_trace::gr!("1003f048", "100af360", DAT_100af360) == 0 {
                i5 = (i5 as u32)
                    .wrapping_add(mech_trace::gr!("1003f048", "100af388", DAT_100af388) as u32)
                    as i32;
                u17 =
                    u17.wrapping_add(mech_trace::gr!("1003f048", "100af390", DAT_100af390) as u32);
                i6 = (i6 as u32)
                    .wrapping_add(mech_trace::gr!("1003f048", "100af38c", DAT_100af38c) as u32)
                    as i32;
                u20 =
                    u20.wrapping_add(mech_trace::gr!("1003f048", "100af394", DAT_100af394) as u32);
                continue;
            }

            mech_trace::gw!(
                "1003f048",
                "100af358",
                DAT_100af358,
                mech_trace::gr!("1003f048", "100af358", DAT_100af358).wrapping_sub(1)
            );
            if mech_trace::gr!("1003f048", "100af358", DAT_100af358) == 0 {
                mech_trace::gw!(
                    "1003f048",
                    "100af348",
                    DAT_100af348,
                    mech_trace::gr!("1003f048", "100af350", DAT_100af350)
                );
                let p12 = {
                    let mut q =
                        mech_trace::gr!("1003f048", "100af350", DAT_100af350).wrapping_sub(6);
                    if ((q as usize) as i32)
                        < ((mech_trace::gr!("1003f048", "100af33c", DAT_100af33c) as usize) as i32)
                    {
                        q = mech_trace::gr!("1003f048", "100af340", DAT_100af340).wrapping_sub(6);
                    }
                    q
                };
                u7 = ru(p12, 1)
                    .wrapping_sub(ru(mech_trace::gr!("1003f048", "100af350", DAT_100af350), 1));
                mech_trace::gw!(
                    "1003f048",
                    "100af358",
                    DAT_100af358,
                    u7.wrapping_add((u7 == 0) as u32) as i32
                );
                let num = (ru(p12, 2)
                    .wrapping_sub(ru(mech_trace::gr!("1003f048", "100af350", DAT_100af350), 2))
                    as i32 as i64)
                    * 65536i64;
                let den = (mech_trace::gr!("1003f048", "100af358", DAT_100af358) as u32)
                    .wrapping_shl(16) as i32;
                dv(num, den, &raw mut DAT_100af390);
                let num = (ru(p12, 0)
                    .wrapping_sub(ru(mech_trace::gr!("1003f048", "100af350", DAT_100af350), 0))
                    as i16 as i32 as i64)
                    * 4294967296i64;
                let den = (mech_trace::gr!("1003f048", "100af358", DAT_100af358) as u32)
                    .wrapping_shl(16) as i32;
                dv(num, den, &raw mut DAT_100af388);
                u17 = ru(mech_trace::gr!("1003f048", "100af350", DAT_100af350), 2)
                    .wrapping_add(0x8000);
                i5 = ru(mech_trace::gr!("1003f048", "100af350", DAT_100af350), 0)
                    .wrapping_mul(0x10000)
                    .wrapping_add(0x8000) as i32;
                mech_trace::gw!("1003f048", "100af350", DAT_100af350, p12);
            } else {
                i5 = (i5 as u32)
                    .wrapping_add(mech_trace::gr!("1003f048", "100af388", DAT_100af388) as u32)
                    as i32;
                u17 =
                    u17.wrapping_add(mech_trace::gr!("1003f048", "100af390", DAT_100af390) as u32);
            }

            mech_trace::gw!(
                "1003f048",
                "100af35c",
                DAT_100af35c,
                mech_trace::gr!("1003f048", "100af35c", DAT_100af35c).wrapping_sub(1)
            );
            if mech_trace::gr!("1003f048", "100af35c", DAT_100af35c) == 0 {
                let mut p12 = mech_trace::gr!("1003f048", "100af354", DAT_100af354).wrapping_add(6);
                if ((mech_trace::gr!("1003f048", "100af340", DAT_100af340) as usize) as i32)
                    <= ((mech_trace::gr!("1003f048", "100af354", DAT_100af354).wrapping_add(6)
                        as usize) as i32)
                {
                    p12 = mech_trace::gr!("1003f048", "100af33c", DAT_100af33c);
                }
                u7 = ru(p12, 1)
                    .wrapping_sub(ru(mech_trace::gr!("1003f048", "100af354", DAT_100af354), 1));
                mech_trace::gw!(
                    "1003f048",
                    "100af35c",
                    DAT_100af35c,
                    u7.wrapping_add((u7 == 0) as u32) as i32
                );
                let num = (ru(p12, 2)
                    .wrapping_sub(ru(mech_trace::gr!("1003f048", "100af354", DAT_100af354), 2))
                    as i32 as i64)
                    * 65536i64;
                let den = (mech_trace::gr!("1003f048", "100af35c", DAT_100af35c) as u32)
                    .wrapping_shl(16) as i32;
                dv(num, den, &raw mut DAT_100af394);
                let num = (ru(p12, 0)
                    .wrapping_sub(ru(mech_trace::gr!("1003f048", "100af354", DAT_100af354), 0))
                    as i16 as i32 as i64)
                    * 4294967296i64;
                let den = (mech_trace::gr!("1003f048", "100af35c", DAT_100af35c) as u32)
                    .wrapping_shl(16) as i32;
                dv(num, den, &raw mut DAT_100af38c);
                u20 = ru(mech_trace::gr!("1003f048", "100af354", DAT_100af354), 2)
                    .wrapping_add(0x8000);
                i6 = ru(mech_trace::gr!("1003f048", "100af354", DAT_100af354), 0)
                    .wrapping_mul(0x10000)
                    .wrapping_add(0x8000) as i32;
                mech_trace::gw!(
                    "1003f048",
                    "100af34c",
                    DAT_100af34c,
                    mech_trace::gr!("1003f048", "100af354", DAT_100af354)
                );
                mech_trace::gw!("1003f048", "100af354", DAT_100af354, p12);
            } else {
                i6 = (i6 as u32)
                    .wrapping_add(mech_trace::gr!("1003f048", "100af38c", DAT_100af38c) as u32)
                    as i32;
                u20 =
                    u20.wrapping_add(mech_trace::gr!("1003f048", "100af394", DAT_100af394) as u32);
            }
        }
    }
}
