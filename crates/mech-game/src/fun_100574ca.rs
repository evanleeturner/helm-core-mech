// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b2b10: [u8; 256];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100579a2(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
}

#[derive(Clone, Copy)]
enum S {
    Next,
    AEntry,
    ATail,
    ASkip,
    ALitMask,
    ALit,
    ALitRem,
    ALitDone,
    AFill,
    AFillCnt,
    BEntry,
    BTail,
    BSkip,
    BLitN,
    BLit,
    BLitRem,
    BLitDone,
    BFillN,
    BFill,
    SFill,
    SLitN,
    SLit,
    SSkip,
    CEntry,
    CTail,
    CSkip,
    CLit,
    CFill,
}

#[inline(always)]
unsafe fn rd(p: u32) -> u32 {
    // SAFETY: the address is inside the sprite data or bitmap, as in the original.
    unsafe { mech_trace::rd!("100574ca", (p as usize as *const u8), read) as u32 }
}

#[inline(always)]
unsafe fn wr(p: u32, v: u8) {
    // SAFETY: the address is inside the destination bitmap, as in the original.
    unsafe { mech_trace::wr!("100574ca", (p as usize as *mut u8), write, v) }
}

#[inline(always)]
unsafe fn tbl(i: u32) -> u8 {
    // SAFETY: i is a byte value; the table is the game's 256-entry remap table.
    unsafe { mech_trace::gr!("100574ca", "100b2b10", DAT_100b2b10[(i & 0xff) as usize]) }
}

#[inline(always)]
unsafe fn fill(mut dst: u32, cnt: u32, c: u8) -> u32 {
    let mut u = cnt & 3;
    // SAFETY: writes land in the destination bitmap, as in the original.
    unsafe {
        while u != 0 {
            wr(dst, c);
            dst = dst.wrapping_add(1);
            u = u.wrapping_sub(1);
        }
        let cu = c as u32;
        let v = cu | (cu << 8) | (cu << 16) | (cu << 24);
        let mut q = cnt >> 2;
        while q != 0 {
            mech_trace::wr!("100574ca", (dst as usize as *mut u32), write_unaligned, v);
            dst = dst.wrapping_add(4);
            q = q.wrapping_sub(1);
        }
    }
    dst
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100574ca(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) -> i32 {
    mech_trace::enter!("100574ca", param_1, param_2, param_3, param_4, param_5);
    // SAFETY: all pointers are the game's own bitmap, clip rectangle and sprite
    // data; reads and writes happen where the original performs them.
    unsafe {
        let bmp =
            mech_trace::rd!("100574ca", param_1, read_unaligned) as u32 as usize as *const i32;
        let w = mech_trace::rd!("100574ca", bmp.add(1), read_unaligned);
        let stride = (w as u32).wrapping_add(1) as i32;
        if stride == 0 || w < -1 {
            return mech_trace::rt!("100574ca", -1);
        }
        let h = mech_trace::rd!("100574ca", bmp.add(2), read_unaligned);
        if h < 0 {
            return mech_trace::rt!("100574ca", -1);
        }
        let p1 = mech_trace::rd!("100574ca", param_1.add(1), read_unaligned);
        let p2 = mech_trace::rd!("100574ca", param_1.add(2), read_unaligned);
        let mut xmin = p1;
        if xmin < 1 {
            xmin = 0;
        }
        let mut ymin = p2;
        if ymin < 1 {
            ymin = 0;
        }
        let mut xmax = mech_trace::rd!("100574ca", param_1.add(3), read_unaligned);
        if w <= mech_trace::rd!("100574ca", param_1.add(3), read_unaligned) {
            xmax = w;
        }
        let mut ymax = mech_trace::rd!("100574ca", param_1.add(4), read_unaligned);
        if h <= mech_trace::rd!("100574ca", param_1.add(4), read_unaligned) {
            ymax = h;
        }
        if xmax < xmin || ymax < ymin {
            return mech_trace::rt!("100574ca", -2);
        }

        let ox = (param_4 as u32).wrapping_add(mech_trace::rd!(
            "100574ca",
            param_1.add(1),
            read_unaligned
        ) as u32) as i32;
        let oy = (param_5 as u32).wrapping_add(mech_trace::rd!(
            "100574ca",
            param_1.add(2),
            read_unaligned
        ) as u32) as i32;
        let ra = (param_3 as u32)
            .wrapping_mul(8)
            .wrapping_add(8)
            .wrapping_add(param_2 as u32);
        let rec = (mech_trace::rd!("100574ca", (ra as usize as *const i32), read_unaligned) as u32)
            .wrapping_add(param_2 as u32);
        let rp = rec as usize as *const u8;
        let left = (mech_trace::rd!("100574ca", rp.add(8).cast::<i32>(), read_unaligned) as u32)
            .wrapping_add(ox as u32) as i32;
        let top = (mech_trace::rd!("100574ca", rp.add(0xc).cast::<i32>(), read_unaligned) as u32)
            .wrapping_add(oy as u32) as i32;
        let right = (mech_trace::rd!("100574ca", rp.add(0x10).cast::<i32>(), read_unaligned) as u32)
            .wrapping_add(ox as u32) as i32;
        let bottom = (mech_trace::rd!("100574ca", rp.add(0x14).cast::<i32>(), read_unaligned)
            as u32)
            .wrapping_add(oy as u32) as i32;
        let mut src: u32 = rec.wrapping_add(0x18);
        if right < left || bottom < top {
            return mech_trace::rt!("100574ca", -4);
        }

        let bit = |v: i32| -> u8 { (v < 0) as u8 };
        let code_l: u8 = ((((bit((left as u32).wrapping_sub(xmin as u32) as i32) * 2
            + bit((xmax as u32).wrapping_sub(left as u32) as i32))
            * 2
            + bit((top as u32).wrapping_sub(ymin as u32) as i32))
            * 2)
            + bit((ymax as u32).wrapping_sub(top as u32) as i32)) as u8;
        let code_r: u8 = ((((bit((right as u32).wrapping_sub(xmin as u32) as i32) * 2
            + bit((xmax as u32).wrapping_sub(right as u32) as i32))
            * 2
            + bit((bottom as u32).wrapping_sub(ymin as u32) as i32))
            * 2)
            + bit((ymax as u32).wrapping_sub(bottom as u32) as i32)) as u8;
        if (code_r & code_l) != 0 {
            return mech_trace::rt!("100574ca", -3);
        }
        if code_l == 0 && code_r == 0 {
            mech_trace::cr!(
                "100574ca",
                "100579a2",
                FUN_100579a2(
                    param_1,
                    rec as i32,
                    (ox as u32).wrapping_sub(mech_trace::rd!(
                        "100574ca",
                        param_1.add(1),
                        read_unaligned
                    ) as u32) as i32,
                    (oy as u32).wrapping_sub(mech_trace::rd!(
                        "100574ca",
                        param_1.add(2),
                        read_unaligned
                    ) as u32) as i32,
                )
            );
            return mech_trace::rt!("100574ca", 0);
        }

        let stride_u = stride as u32;
        let mut row: u32 = (top as u32)
            .wrapping_mul(stride_u)
            .wrapping_add(mech_trace::rd!("100574ca", bmp, read_unaligned) as u32)
            .wrapping_add(left as u32);
        let mut y = top;

        while y < ymin {
            loop {
                let b = rd(src);
                src = src.wrapping_add(1);
                let flag = b & 1;
                let n = b >> 1;
                if flag != 0 || n == 0 {
                    if n != 0 {
                        src = src.wrapping_add(n.wrapping_sub(1));
                    } else if flag == 0 {
                        break;
                    }
                }
                src = src.wrapping_add(1);
            }
            row = row.wrapping_add(stride_u);
            y = y.wrapping_add(1);
        }

        let mut clip_l: u32 = row.wrapping_add((xmin as u32).wrapping_sub(left as u32));
        let mut clip_r: u32 = row.wrapping_add((xmax as u32).wrapping_sub(left as u32));

        let mut dst: u32;
        let mut b: u32;
        let mut flag: u32 = 0;
        let mut n: u32 = 0;
        let mut cnt: u32 = 0;
        let mut k: i32 = 0;
        let mut over: u32 = 0;
        let mut t: u32;
        let mut d: i32;
        let mut tb: u32;

        loop {
            if bottom < y {
                return mech_trace::rt!("100574ca", 0);
            }
            if ymax < y {
                return mech_trace::rt!("100574ca", 0);
            }
            dst = row;
            let mut st = if (code_l & 8) != 0 {
                S::CEntry
            } else if (code_r & 4) != 0 {
                S::BEntry
            } else {
                S::AEntry
            };

            loop {
                match st {
                    S::Next => break,

                    // ---------- mode A ----------
                    S::AEntry => {
                        b = rd(src);
                        src = src.wrapping_add(1);
                        flag = b & 1;
                        n = b >> 1;
                        cnt = n;
                        st = if flag == 0 && n != 0 {
                            S::AFill
                        } else if n != 0 {
                            S::ALitMask
                        } else {
                            S::ATail
                        };
                    }
                    S::ATail => {
                        st = if flag != 0 { S::ASkip } else { S::Next };
                    }
                    S::ASkip => {
                        t = src.wrapping_add(1);
                        dst = dst.wrapping_add(rd(src));
                        src = src.wrapping_add(2);
                        tb = rd(t);
                        flag = tb & 1;
                        n = tb >> 1;
                        cnt = n;
                        st = if flag == 0 && n != 0 {
                            S::AFill
                        } else if n != 0 {
                            S::ALitMask
                        } else if flag != 0 {
                            S::ASkip
                        } else {
                            S::Next
                        };
                    }
                    S::ALitMask => {
                        cnt &= 0xff;
                        st = S::ALit;
                    }
                    S::ALit => loop {
                        if cnt == 0 {
                            st = S::ALitDone;
                            break;
                        }
                        if (cnt as i32) < 4 {
                            st = S::ALitRem;
                            break;
                        }
                        wr(dst, tbl(rd(src)));
                        wr(dst.wrapping_add(1), tbl(rd(src.wrapping_add(1))));
                        wr(dst.wrapping_add(2), tbl(rd(src.wrapping_add(2))));
                        wr(dst.wrapping_add(3), tbl(rd(src.wrapping_add(3))));
                        src = src.wrapping_add(4);
                        dst = dst.wrapping_add(4);
                        cnt = cnt.wrapping_sub(4);
                    },
                    S::ALitRem => {
                        loop {
                            wr(dst, tbl(rd(src)));
                            src = src.wrapping_add(1);
                            dst = dst.wrapping_add(1);
                            cnt = cnt.wrapping_sub(1);
                            if cnt == 0 {
                                break;
                            }
                        }
                        st = S::ALitDone;
                    }
                    S::ALitDone => {
                        b = rd(src);
                        src = src.wrapping_add(1);
                        flag = b & 1;
                        n = b >> 1;
                        cnt = n;
                        st = if flag != 0 || n == 0 {
                            if n == 0 { S::ATail } else { S::ALitMask }
                        } else {
                            S::AFill
                        };
                    }
                    S::AFill => {
                        cnt &= 0xff;
                        st = S::AFillCnt;
                    }
                    S::AFillCnt => {
                        t = src.wrapping_add(1);
                        let c = tbl(rd(src));
                        dst = fill(dst, cnt, c);
                        src = src.wrapping_add(2);
                        tb = rd(t);
                        flag = tb & 1;
                        n = tb >> 1;
                        cnt = n;
                        st = if flag == 0 && n != 0 {
                            S::AFill
                        } else if flag == 0 {
                            S::Next
                        } else if n == 0 {
                            S::ASkip
                        } else {
                            S::ALitMask
                        };
                    }

                    // ---------- mode B ----------
                    S::BEntry => {
                        b = rd(src);
                        src = src.wrapping_add(1);
                        flag = b & 1;
                        n = b >> 1;
                        st = if flag == 0 && n != 0 {
                            S::BFillN
                        } else if n != 0 {
                            S::BLitN
                        } else {
                            S::BTail
                        };
                    }
                    S::BTail => {
                        st = if flag != 0 { S::BSkip } else { S::Next };
                    }
                    S::BSkip => {
                        t = src.wrapping_add(1);
                        dst = dst.wrapping_add(rd(src));
                        src = src.wrapping_add(2);
                        tb = rd(t);
                        flag = tb & 1;
                        n = tb >> 1;
                        st = if flag == 0 && n != 0 {
                            S::BFillN
                        } else if n != 0 {
                            S::BLitN
                        } else if flag != 0 {
                            S::BSkip
                        } else {
                            S::Next
                        };
                    }
                    S::BLitN => {
                        cnt = n;
                        st = S::BLit;
                    }
                    S::BLit => {
                        if (clip_r as i32) < (dst as i32) {
                            st = S::SLit;
                        } else {
                            d = dst.wrapping_add(cnt.wrapping_sub(1).wrapping_sub(clip_r)) as i32;
                            over = ((!(d >> 31)) & d) as u32;
                            k = cnt.wrapping_sub(over) as i32;
                            loop {
                                if k == 0 {
                                    st = S::BLitDone;
                                    break;
                                }
                                if k < 4 {
                                    st = S::BLitRem;
                                    break;
                                }
                                wr(dst, tbl(rd(src)));
                                wr(dst.wrapping_add(1), tbl(rd(src.wrapping_add(1))));
                                wr(dst.wrapping_add(2), tbl(rd(src.wrapping_add(2))));
                                wr(dst.wrapping_add(3), tbl(rd(src.wrapping_add(3))));
                                src = src.wrapping_add(4);
                                dst = dst.wrapping_add(4);
                                k = k.wrapping_sub(4);
                            }
                        }
                    }
                    S::BLitRem => {
                        loop {
                            wr(dst, tbl(rd(src)));
                            src = src.wrapping_add(1);
                            dst = dst.wrapping_add(1);
                            k = k.wrapping_sub(1);
                            if k == 0 {
                                break;
                            }
                        }
                        st = S::BLitDone;
                    }
                    S::BLitDone => {
                        dst = dst.wrapping_add(over);
                        b = rd(src.wrapping_add(over));
                        src = src.wrapping_add(over).wrapping_add(1);
                        flag = b & 1;
                        n = b >> 1;
                        st = if flag != 0 || n == 0 {
                            if n == 0 { S::BTail } else { S::BLitN }
                        } else {
                            S::BFillN
                        };
                    }
                    S::BFillN => {
                        cnt = n;
                        st = S::BFill;
                    }
                    S::BFill => {
                        if (clip_r as i32) < (dst as i32) {
                            st = S::SFill;
                        } else {
                            d = dst.wrapping_add(cnt.wrapping_sub(1).wrapping_sub(clip_r)) as i32;
                            over = ((!(d >> 31)) & d) as u32;
                            cnt = cnt.wrapping_sub(over);
                            t = src.wrapping_add(1);
                            let c = tbl(rd(src));
                            dst = fill(dst, cnt, c);
                            dst = dst.wrapping_add(over);
                            src = src.wrapping_add(2);
                            tb = rd(t);
                            flag = tb & 1;
                            n = tb >> 1;
                            st = if flag == 0 && n != 0 {
                                S::BFillN
                            } else if flag == 0 {
                                S::Next
                            } else if n == 0 {
                                S::BSkip
                            } else {
                                S::BLitN
                            };
                        }
                    }

                    // ---------- past the right edge ----------
                    S::SFill => {
                        t = src.wrapping_add(1);
                        src = src.wrapping_add(2);
                        tb = rd(t);
                        flag = tb & 1;
                        n = tb >> 1;
                        st = if flag == 0 && n != 0 {
                            S::SFill
                        } else if flag == 0 {
                            S::Next
                        } else if n == 0 {
                            S::SSkip
                        } else {
                            S::SLitN
                        };
                    }
                    S::SLitN => {
                        cnt = n;
                        st = S::SLit;
                    }
                    S::SLit => {
                        b = rd(src.wrapping_add(cnt));
                        src = src.wrapping_add(cnt).wrapping_add(1);
                        flag = b & 1;
                        n = b >> 1;
                        st = if flag == 0 && n != 0 {
                            S::SFill
                        } else if n == 0 {
                            if flag == 0 { S::Next } else { S::SSkip }
                        } else {
                            S::SLitN
                        };
                    }
                    S::SSkip => {
                        t = src.wrapping_add(1);
                        src = src.wrapping_add(2);
                        tb = rd(t);
                        flag = tb & 1;
                        n = tb >> 1;
                        st = if flag == 0 && n != 0 {
                            S::SFill
                        } else if n == 0 {
                            if flag == 0 { S::Next } else { S::SSkip }
                        } else {
                            S::SLitN
                        };
                    }

                    // ---------- mode C ----------
                    S::CEntry => {
                        b = rd(src);
                        src = src.wrapping_add(1);
                        flag = b & 1;
                        n = b >> 1;
                        st = if flag == 0 && n != 0 {
                            S::CFill
                        } else if n != 0 {
                            S::CLit
                        } else {
                            S::CTail
                        };
                    }
                    S::CTail => {
                        st = if flag != 0 { S::CSkip } else { S::Next };
                    }
                    S::CSkip => {
                        t = src.wrapping_add(1);
                        dst = dst.wrapping_add(rd(src));
                        src = src.wrapping_add(2);
                        tb = rd(t);
                        flag = tb & 1;
                        n = tb >> 1;
                        st = if flag == 0 && n != 0 {
                            S::CFill
                        } else if n == 0 {
                            if flag != 0 { S::CSkip } else { S::Next }
                        } else {
                            S::CLit
                        };
                    }
                    S::CLit => {
                        cnt = n;
                        d = clip_l.wrapping_sub(dst) as i32;
                        if d < (cnt as i32) {
                            if -1 < d {
                                dst = dst.wrapping_add(d as u32);
                                cnt = cnt.wrapping_sub(d as u32);
                                src = src.wrapping_add(d as u32);
                            }
                            st = if (code_r & 4) == 0 { S::ALit } else { S::BLit };
                        } else {
                            dst = dst.wrapping_add(cnt);
                            b = rd(src.wrapping_add(cnt));
                            src = src.wrapping_add(cnt).wrapping_add(1);
                            flag = b & 1;
                            n = b >> 1;
                            st = if flag == 0 && n != 0 {
                                S::CFill
                            } else if n == 0 {
                                S::CTail
                            } else {
                                S::CLit
                            };
                        }
                    }
                    S::CFill => {
                        cnt = n;
                        d = clip_l.wrapping_sub(dst) as i32;
                        if d < (cnt as i32) {
                            if -1 < d {
                                dst = dst.wrapping_add(d as u32);
                                cnt = cnt.wrapping_sub(d as u32);
                            }
                            st = if (code_r & 4) == 0 {
                                S::AFillCnt
                            } else {
                                S::BFill
                            };
                        } else {
                            dst = dst.wrapping_add(cnt);
                            t = src.wrapping_add(1);
                            src = src.wrapping_add(2);
                            tb = rd(t);
                            flag = tb & 1;
                            n = tb >> 1;
                            st = if flag == 0 && n != 0 {
                                S::CFill
                            } else if flag == 0 {
                                S::Next
                            } else if n != 0 {
                                S::CLit
                            } else {
                                S::CSkip
                            };
                        }
                    }
                }
            }

            row = row.wrapping_add(stride_u);
            clip_l = clip_l.wrapping_add(stride_u);
            clip_r = clip_r.wrapping_add(stride_u);
            y = y.wrapping_add(1);
        }
    }
}
