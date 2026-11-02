// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_1011516c: i32;
    static mut _DAT_101150f4: i32;
    static mut _DAT_10115164: i32;
    static mut _DAT_10115158: i32;
    static mut _DAT_10160f44: i32;
    static mut _DAT_10160f48: i32;
    static mut _DAT_10160f4c: i32;
    static mut DAT_10160f0c: i32;
    static mut DAT_10160f10: i32;
    static mut DAT_10160f14: i32;
    static mut DAT_10160eb0: i32;
    static mut DAT_10160ebc: i32;
    static mut DAT_101150f8: i32;
    static mut DAT_10115168: i8;
    static mut DAT_10115160: u32;
    static mut DAT_10115174: u32;
    static mut DAT_10115100: [i32; 1];
    static mut DAT_101150f0: *mut u32;
    static mut DAT_10168288: *mut u32;
    static mut DAT_100aafcc: unsafe extern "C" fn(*mut i32);
    static mut DAT_100aafd0: unsafe extern "C" fn(*mut u16, i32, u16, u32) -> u16;
    static mut DAT_100b0f18: i32;
    static mut DAT_10167658: i32;
    static mut DAT_100b0f1c: i32;
    static mut DAT_10115170: i32;
    static mut DAT_1011515c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10080130(param_1: i32) -> *mut i32;
    fn FUN_10080241(param_1: i32, param_2: i32) -> *mut i32;
    fn FUN_10035f89() -> *mut u16;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

unsafe fn rd(base: i32, off: i32) -> i32 {
    // SAFETY: caller guarantees base+off addresses readable game memory.
    unsafe {
        mech_trace::rd!(
            "10080919",
            (base.wrapping_add(off) as *const i32),
            read_unaligned
        )
    }
}

unsafe fn wr(base: i32, off: i32, v: i32) {
    // SAFETY: caller guarantees base+off addresses writable game memory.
    unsafe {
        mech_trace::wr!(
            "10080919",
            (base.wrapping_add(off) as *mut i32),
            write_unaligned,
            v
        )
    }
}

unsafe fn flags(v: i32) -> u8 {
    // SAFETY: v points at a vertex record in the game's memory.
    unsafe { mech_trace::dr!("10080919", ((v.wrapping_add(0x28)) as *const u8)) }
}

unsafe fn set_flags(v: i32, b: u8) {
    // SAFETY: v points at a vertex record in the game's memory.
    unsafe { mech_trace::dw!("10080919", ((v.wrapping_add(0x28)) as *mut u8), b) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10080919(param_1: *mut u16, param_2: i32) {
    mech_trace::enter!("10080919", param_1, param_2);
    // SAFETY: all globals and callees are the game's own; param_1 and
    // param_2 point at the caller's polygon record and vertex array.
    unsafe {
        let p1i = param_1 as usize as i32;
        let idx_off = rd(p1i, 4);
        let mut local_18: u8 = 3;
        mech_trace::gw!(
            "10080919",
            "1011516c",
            _DAT_1011516c,
            mech_trace::gr!("10080919", "1011516c", _DAT_1011516c).wrapping_add(1)
        );

        let do_it: bool;
        if mech_trace::rd!("10080919", param_1.add(1), read_unaligned) < 3 {
            do_it = true;
        } else {
            let b = mech_trace::dr!("10080919", ((p1i.wrapping_add(idx_off)) as *const u8));
            let v = param_2.wrapping_add(((b as u16).wrapping_mul(0x44)) as i32);
            let p1 =
                (rd(v, 0xc).wrapping_sub(mech_trace::gr!("10080919", "10160f48", _DAT_10160f48))
                    as i64)
                    .wrapping_mul(rd(p1i, 0x14) as i64);
            let p2 =
                (rd(v, 0x10).wrapping_sub(mech_trace::gr!("10080919", "10160f44", _DAT_10160f44))
                    as i64)
                    .wrapping_mul(rd(p1i, 0x18) as i64);
            let s3 = (p1 as u64).wrapping_add(p2 as u64);
            let p3 =
                (rd(v, 0x14).wrapping_sub(mech_trace::gr!("10080919", "10160f4c", _DAT_10160f4c))
                    as i64)
                    .wrapping_mul(rd(p1i, 0x1c) as i64);
            let lo3 = s3 as u32;
            let lo2 = (p3 as u64) as u32;
            let carry: i64 = if lo3.wrapping_add(lo2) < lo3 { 1 } else { 0 };
            let hi_sum = ((s3 >> 32) as u32 as i32 as i64)
                .wrapping_add(((p3 as u64) >> 32) as u32 as i32 as i64)
                .wrapping_add(carry);
            do_it = hi_sum < 0;
        }
        if !do_it {
            return;
        }

        mech_trace::gw!(
            "10080919",
            "101150f4",
            _DAT_101150f4,
            mech_trace::gr!("10080919", "101150f4", _DAT_101150f4).wrapping_add(1)
        );
        let mut local_1c: i16 = mech_trace::rd!("10080919", param_1.add(1), read_unaligned) as i16;
        let mut local_24: *const u8 = p1i.wrapping_add(idx_off) as *const u8;
        loop {
            let v = param_2.wrapping_add(
                ((mech_trace::dr!("10080919", local_24) as u16).wrapping_mul(0x44)) as i32,
            );
            if flags(v) & 4 == 0 {
                mech_trace::gw!(
                    "10080919",
                    "10115164",
                    _DAT_10115164,
                    mech_trace::gr!("10080919", "10115164", _DAT_10115164).wrapping_add(1)
                );
                let t1 = (mech_trace::gr!("10080919", "10160f10", DAT_10160f10) as i64)
                    .wrapping_mul(rd(v, 0x10).wrapping_sub(mech_trace::gr!(
                        "10080919",
                        "10160f44",
                        _DAT_10160f44
                    )) as i64);
                let t2 = (mech_trace::gr!("10080919", "10160f0c", DAT_10160f0c) as i64)
                    .wrapping_mul(rd(v, 0xc).wrapping_sub(mech_trace::gr!(
                        "10080919",
                        "10160f48",
                        _DAT_10160f48
                    )) as i64);
                let t3 = (mech_trace::gr!("10080919", "10160f14", DAT_10160f14) as i64)
                    .wrapping_mul(rd(v, 0x14).wrapping_sub(mech_trace::gr!(
                        "10080919",
                        "10160f4c",
                        _DAT_10160f4c
                    )) as i64);
                let sum = (t1 as u64).wrapping_add(t2 as u64).wrapping_add(t3 as u64);
                let lo = sum as u32;
                let hi = (sum >> 32) as u32;
                let rnd: u32 = if (lo >> 0x1a) & 1 != 0 { 1 } else { 0 };
                let i_var15 = ((lo >> 0x1b) | hi.wrapping_mul(0x20)).wrapping_add(rnd) as i32;
                wr(v, 0x20, i_var15);
                set_flags(v, flags(v) | 4);
                let mut b4: u8 =
                    (i_var15 < mech_trace::gr!("10080919", "10160eb0", DAT_10160eb0)) as u8;
                if mech_trace::gr!("10080919", "10160ebc", DAT_10160ebc) < i_var15 {
                    b4 |= 2;
                }
                set_flags(v, flags(v) & 0xfc);
                set_flags(v, flags(v) | b4);
            }
            local_18 &= flags(v);
            local_24 = local_24.add(1);
            local_1c = local_1c.wrapping_sub(1);
            if local_1c == 0 {
                break;
            }
        }

        if local_18 == 1 || local_18 == 2 {
            return;
        }

        mech_trace::gw!("10080919", "101150f8", DAT_101150f8, 0);
        mech_trace::gw!("10080919", "10115168", DAT_10115168, 0x0f);
        mech_trace::gw!("10080919", "10115160", DAT_10115160, 0);
        let mut local_3c: *const u8 = p1i.wrapping_add(rd(p1i, 4)) as *const u8;
        local_1c = mech_trace::rd!("10080919", param_1.add(1), read_unaligned) as i16;
        let i_var16 = param_2.wrapping_add(
            ((mech_trace::dr!("10080919", local_3c) as u16).wrapping_mul(0x44)) as i32,
        );
        let b_var4 = flags(i_var16) & 1;
        let mut local_40 = b_var4;
        let mut local_34 = i_var16;
        if b_var4 == 0 {
            let p = mech_trace::cr!("10080919", "10080130", FUN_10080130(i_var16));
            DAT_100aafcc(p);
        }
        loop {
            local_1c = local_1c.wrapping_sub(1);
            if local_1c == 0 {
                break;
            }
            local_3c = local_3c.add(1);
            let i_var15 = param_2.wrapping_add(
                ((mech_trace::dr!("10080919", local_3c) as u16).wrapping_mul(0x44)) as i32,
            );
            let b_var5 = flags(i_var15) & 1;
            if b_var5 != local_40 {
                let p = mech_trace::cr!("10080919", "10080241", FUN_10080241(local_34, i_var15));
                DAT_100aafcc(p);
            }
            local_34 = param_2.wrapping_add(
                ((mech_trace::dr!("10080919", local_3c) as u16).wrapping_mul(0x44)) as i32,
            );
            local_40 = b_var5;
            if b_var5 == 0 {
                let p = mech_trace::cr!("10080919", "10080130", FUN_10080130(local_34));
                DAT_100aafcc(p);
            }
        }
        if local_40 != b_var4 {
            let p = mech_trace::cr!("10080919", "10080241", FUN_10080241(local_34, i_var16));
            DAT_100aafcc(p);
        }

        if !(((2 < mech_trace::gr!("10080919", "10115160", DAT_10115160) as i32)
            || (mech_trace::rd!("10080919", param_1.add(1), read_unaligned) < 3))
            && mech_trace::gr!("10080919", "10115168", DAT_10115168) == 0)
        {
            return;
        }

        let puvar8 = mech_trace::cr!("10080919", "10035f89", FUN_10035f89());
        mech_trace::wr!(
            "10080919",
            puvar8.add(2).cast::<u32>(),
            write_unaligned,
            p1i as u32
        );
        mech_trace::gw!(
            "10080919",
            "101150f0",
            DAT_101150f0,
            mech_trace::gr!("10080919", "10168288", DAT_10168288)
        );
        mech_trace::wr!(
            "10080919",
            puvar8,
            write_unaligned,
            mech_trace::gr!("10080919", "10115160", DAT_10115160) as u16
        );
        let local_14: u32;
        let tbl = &raw mut DAT_10115100 as *mut i32;
        if mech_trace::gr!("10080919", "10115174", DAT_10115174) & 2 == 0 {
            if mech_trace::gr!("10080919", "10115174", DAT_10115174) & 4 == 0 {
                let mut piv = tbl;
                let mut l14: u32 = 0x80000001;
                let mut u14 = mech_trace::gr!("10080919", "10115160", DAT_10115160);
                loop {
                    let e = rd(mech_trace::rd!("10080919", piv, read_unaligned), 8);
                    if (l14 as i32) < e {
                        l14 = e as u32;
                    }
                    piv = piv.add(1);
                    u14 = u14.wrapping_sub(1);
                    if u14 == 0 {
                        break;
                    }
                }
                local_14 = l14;
            } else {
                let mut piv = tbl;
                let mut l14: u32 = 0x7fffffff;
                let mut u14 = mech_trace::gr!("10080919", "10115160", DAT_10115160);
                loop {
                    let e = rd(mech_trace::rd!("10080919", piv, read_unaligned), 8);
                    if e < l14 as i32 {
                        l14 = e as u32;
                    }
                    piv = piv.add(1);
                    u14 = u14.wrapping_sub(1);
                    if u14 == 0 {
                        break;
                    }
                }
                local_14 = l14;
            }
        } else {
            let mut piv = tbl;
            let mut u9: u32 = 0;
            let mut u14 = mech_trace::gr!("10080919", "10115160", DAT_10115160);
            loop {
                u9 =
                    u9.wrapping_add(rd(mech_trace::rd!("10080919", piv, read_unaligned), 8) as u32);
                piv = piv.add(1);
                u14 = u14.wrapping_sub(1);
                if u14 == 0 {
                    break;
                }
            }
            let divisor = (mech_trace::gr!("10080919", "10115160", DAT_10115160) & 0xffff) as i32
                as i64 as u64;
            if divisor == 0 || (u9 as u64) / divisor > 0xffff_ffff {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
            }
            local_14 = if divisor != 0 {
                ((u9 as u64) / divisor) as u32
            } else {
                0
            };
        }
        let mut local_14 = local_14;
        if mech_trace::gr!("10080919", "10115174", DAT_10115174) & 1 != 0 {
            local_14 |= 0x40000000;
        }
        mech_trace::wr!(
            "10080919",
            puvar8.add(4).cast::<u32>(),
            write_unaligned,
            local_14
        );
        let mut pu17 = tbl as *const u32;
        let mut u14 = mech_trace::gr!("10080919", "10115160", DAT_10115160);
        while u14 != 0 {
            mech_trace::wr!(
                "10080919",
                mech_trace::gr!("10080919", "101150f0", DAT_101150f0),
                write_unaligned,
                mech_trace::rd!("10080919", pu17, read_unaligned)
            );
            pu17 = pu17.add(1);
            mech_trace::gw!(
                "10080919",
                "101150f0",
                DAT_101150f0,
                mech_trace::gr!("10080919", "101150f0", DAT_101150f0).add(1)
            );
            u14 = u14.wrapping_sub(1);
        }
        mech_trace::gw!(
            "10080919",
            "10168288",
            DAT_10168288,
            mech_trace::gr!("10080919", "101150f0", DAT_101150f0)
        );
        let u6 = DAT_100aafd0(
            param_1,
            param_2,
            mech_trace::rd!("10080919", param_1, read_unaligned),
            local_14,
        );
        mech_trace::wr!("10080919", puvar8.add(1), write_unaligned, u6);

        if mech_trace::gr!("10080919", "100b0f18", DAT_100b0f18)
            < mech_trace::gr!("10080919", "10167658", DAT_10167658)
        {
            let u_var1 = mech_trace::rd!("10080919", param_1.add(1), read_unaligned);
            mech_trace::gw!(
                "10080919",
                "10115158",
                _DAT_10115158,
                mech_trace::gr!("10080919", "10115158", _DAT_10115158).wrapping_add(1)
            );
            if 1 < mech_trace::gr!("10080919", "10115160", DAT_10115160) as i32 {
                mech_trace::gw!(
                    "10080919",
                    "100b0f1c",
                    DAT_100b0f1c,
                    mech_trace::gr!("10080919", "100b0f1c", DAT_100b0f1c).wrapping_add(1)
                );
            }
            let rec = mech_trace::gr!("10080919", "100b0f18", DAT_100b0f18)
                .wrapping_mul(0x20)
                .wrapping_add(mech_trace::gr!("10080919", "10115170", DAT_10115170));
            wr(rec, 0, puvar8 as usize as i32);
            wr(rec, 4, local_14 as i32);
            let off0 = rd(p1i, 4);
            let v0 = (((mech_trace::dr!("10080919", ((off0.wrapping_add(p1i)) as *const u8)))
                as u32)
                .wrapping_mul(0x44) as i32)
                .wrapping_add(param_2);
            let mut local_64 = rd(v0, 0x14);
            let mut local_60 = rd(v0, 0x10);
            let mut local_5c = rd(v0, 0xc);
            let mut local_54 = rd(v0, 0x14);
            let mut local_50 = rd(v0, 0x10);
            let mut local_4c = rd(v0, 0xc);
            let mut local_58: i32 = 1;
            while local_58 < u_var1 as i32 {
                let o = rd(p1i, 4);
                let b = mech_trace::dr!(
                    "10080919",
                    ((o.wrapping_add(local_58).wrapping_add(p1i)) as *const u8)
                );
                let v = ((b as u32).wrapping_mul(0x44) as i32).wrapping_add(param_2);
                let x = rd(v, 0xc);
                let i_var16 = if x <= local_5c { local_5c } else { x };
                let i_var18 = if local_4c <= x { local_4c } else { x };
                let y = rd(v, 0x10);
                let i_var10 = if y <= local_60 { local_60 } else { y };
                let i_var11 = if local_50 <= y { local_50 } else { y };
                let z = rd(v, 0x14);
                let i_var12 = if z <= local_64 { local_64 } else { z };
                let i_var13 = if local_54 <= z { local_54 } else { z };
                local_64 = i_var12;
                local_60 = i_var10;
                local_5c = i_var16;
                local_54 = i_var13;
                local_50 = i_var11;
                local_4c = i_var18;
                local_58 = local_58.wrapping_add(1);
            }
            let rec = mech_trace::gr!("10080919", "100b0f18", DAT_100b0f18)
                .wrapping_mul(0x20)
                .wrapping_add(mech_trace::gr!("10080919", "10115170", DAT_10115170));
            wr(rec, 8, local_5c);
            wr(rec, 0x14, local_4c);
            wr(rec, 0xc, local_60);
            wr(rec, 0x18, local_50);
            wr(rec, 0x10, local_64);
            wr(rec, 0x1c, local_54);
            mech_trace::gw!(
                "10080919",
                "100b0f18",
                DAT_100b0f18,
                mech_trace::gr!("10080919", "100b0f18", DAT_100b0f18).wrapping_add(1)
            );
        } else {
            mech_trace::gw!("10080919", "1011515c", DAT_1011515c, 0);
        }
    }
}
