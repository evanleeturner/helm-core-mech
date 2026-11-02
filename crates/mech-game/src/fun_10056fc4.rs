// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100573c8(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
}

unsafe fn rb(p: u32) -> u8 {
    // SAFETY: the address is a game buffer address, read as the original does.
    unsafe { (p as usize as *const u8).read() }
}

unsafe fn wb(p: u32, v: u8) {
    // SAFETY: the address is a game buffer address, written as the original does.
    unsafe { (p as usize as *mut u8).write(v) }
}

unsafe fn rw(p: u32) -> u32 {
    // SAFETY: the address is a game buffer address, read as the original does.
    unsafe { (p as usize as *const u32).read_unaligned() }
}

unsafe fn ww(p: u32, v: u32) {
    // SAFETY: the address is a game buffer address, written as the original does.
    unsafe { (p as usize as *mut u32).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10056fc4(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) -> u32 {
    const SKIP: u32 = 0;
    const ROW: u32 = 1;
    const END: u32 = 2;
    const A0: u32 = 3;
    const JA: u32 = 4;
    const LITA: u32 = 5;
    const LIT2: u32 = 6;
    const RUNA: u32 = 7;
    const RUNA2: u32 = 8;
    const B0: u32 = 9;
    const JB: u32 = 10;
    const LITB: u32 = 11;
    const LITB2: u32 = 12;
    const RUNB: u32 = 13;
    const RUNB2: u32 = 14;
    const C0: u32 = 15;
    const JC: u32 = 16;
    const LITC: u32 = 17;
    const RUNC: u32 = 18;
    const T1: u32 = 19;
    const T2: u32 = 20;
    const T3: u32 = 21;

    // SAFETY: all pointers are the game's own structures and buffers; reads
    // and writes happen in the same order and places as the original.
    unsafe {
        let pi1 = (*param_1 as u32) as usize as *mut i32;
        let mut i10 = pi1.add(1).read();
        let i6 = i10.wrapping_add(1);
        if i6 == 0 || i10 < -1 {
            return 0xffffffff;
        }
        let mut i11 = pi1.add(2).read();
        if i11 < 0 {
            return 0xffffffff;
        }
        let mut i2 = param_1.add(1).read();
        let i7 = if i2 < 1 { 0 } else { i2 };
        let mut i3 = param_1.add(2).read();
        let i8 = if i3 < 1 { 0 } else { i3 };
        let mut i9 = param_1.add(3).read();
        if i10 <= param_1.add(3).read() {
            i9 = i10;
        }
        i10 = param_1.add(4).read();
        if i11 <= param_1.add(4).read() {
            i10 = i11;
        }
        if i9 < i7 || i10 < i8 {
            return 0xfffffffe;
        }
        i2 = param_4.wrapping_add(i2);
        i3 = param_5.wrapping_add(i3);
        let a = (param_3 as u32)
            .wrapping_mul(8)
            .wrapping_add(8)
            .wrapping_add(param_2 as u32);
        let i19 = (rw(a) as i32).wrapping_add(param_2);
        let b = i19 as u32;
        i11 = (rw(b.wrapping_add(8)) as i32).wrapping_add(i2);
        let mut local_24 = (rw(b.wrapping_add(0xc)) as i32).wrapping_add(i3);
        let i12 = (rw(b.wrapping_add(0x10)) as i32).wrapping_add(i2);
        let i13 = (rw(b.wrapping_add(0x14)) as i32).wrapping_add(i3);
        let mut p20: u32 = b.wrapping_add(0x18);
        if i12 < i11 || i13 < local_24 {
            return 0xfffffffc;
        }
        let b16: u8 =
            (((((i11.wrapping_sub(i7) < 0) as i32) * 2 + (i9.wrapping_sub(i11) < 0) as i32) * 2
                + (local_24.wrapping_sub(i8) < 0) as i32)
                * 2
                + (i10.wrapping_sub(local_24) < 0) as i32) as u8;
        let b18: u8 =
            (((((i12.wrapping_sub(i7) < 0) as i32) * 2 + (i9.wrapping_sub(i12) < 0) as i32) * 2
                + (i13.wrapping_sub(i8) < 0) as i32)
                * 2
                + (i10.wrapping_sub(i13) < 0) as i32) as u8;
        if (b18 & b16) != 0 {
            return 0xfffffffd;
        }
        if b16 == 0 && b18 == 0 {
            FUN_100573c8(
                param_1,
                i19,
                i2.wrapping_sub(param_1.add(1).read()),
                i3.wrapping_sub(param_1.add(2).read()),
            );
            return 0;
        }
        let mut local_28: u32 = (local_24 as u32)
            .wrapping_mul(i6 as u32)
            .wrapping_add(pi1.read() as u32)
            .wrapping_add(i11 as u32);
        let mut local_2c: u32 = 0;
        let mut local_30: u32 = 0;

        let mut p21: u32;
        let mut p22: u32 = 0;
        let mut bt: u8 = 0;
        let mut cnt: u8 = 0;
        let mut u14: u32 = 0;
        let mut u15: u32;
        let mut u17: u32;
        let mut st: u32 = SKIP;

        macro_rules! readtok {
            ($p:expr) => {{
                let v = rb($p);
                bt = v & 1;
                cnt = v >> 1;
            }};
        }
        macro_rules! disp {
            ($j:expr, $l:expr, $r:expr) => {{
                u14 = cnt as u32;
                st = if bt == 0 && cnt != 0 {
                    $r
                } else if cnt != 0 {
                    $l
                } else {
                    $j
                };
            }};
        }
        macro_rules! skipstep {
            () => {{
                p21 = p20.wrapping_add(1);
                p22 = p22.wrapping_add(rb(p20) as u32);
                p20 = p20.wrapping_add(2);
                readtok!(p21);
            }};
        }
        macro_rules! clip {
            () => {{
                let t =
                    (p22 as i32).wrapping_add(u14.wrapping_sub(1).wrapping_sub(local_30) as i32);
                u17 = !((t >> 31) as u32) & (t as u32);
            }};
        }

        loop {
            match st {
                SKIP => {
                    if local_24 < i8 {
                        loop {
                            let v = rb(p20);
                            p20 = p20.wrapping_add(1);
                            let bq = v & 1;
                            let c = v >> 1;
                            if bq != 0 || c == 0 {
                                if c != 0 {
                                    p20 = p20.wrapping_add((c as u32).wrapping_sub(1));
                                } else if bq == 0 {
                                    break;
                                }
                            }
                            p20 = p20.wrapping_add(1);
                        }
                        local_28 = local_28.wrapping_add(i6 as u32);
                        local_24 = local_24.wrapping_add(1);
                        st = SKIP;
                        continue;
                    }
                    local_2c = local_28.wrapping_add(i7.wrapping_sub(i11) as u32);
                    local_30 = local_28.wrapping_add(i9.wrapping_sub(i11) as u32);
                    st = ROW;
                }
                ROW => {
                    if i13 < local_24 {
                        return 0;
                    }
                    if i10 < local_24 {
                        return 0;
                    }
                    p22 = local_28;
                    if (b16 & 8) == 0 {
                        if (b18 & 4) == 0 {
                            st = A0;
                        } else {
                            st = B0;
                        }
                    } else {
                        st = C0;
                    }
                }
                END => {
                    local_28 = local_28.wrapping_add(i6 as u32);
                    local_2c = local_2c.wrapping_add(i6 as u32);
                    local_30 = local_30.wrapping_add(i6 as u32);
                    local_24 = local_24.wrapping_add(1);
                    st = ROW;
                }
                A0 => {
                    readtok!(p20);
                    p20 = p20.wrapping_add(1);
                    disp!(JA, LITA, RUNA);
                }
                JA => {
                    if bt == 0 {
                        st = END;
                    } else {
                        skipstep!();
                        disp!(JA, LITA, RUNA);
                    }
                }
                LITA => {
                    u14 &= 0xff;
                    st = LIT2;
                }
                LIT2 => {
                    u15 = u14 & 3;
                    while u15 != 0 {
                        wb(p22, rb(p20));
                        p20 = p20.wrapping_add(1);
                        p22 = p22.wrapping_add(1);
                        u15 -= 1;
                    }
                    p21 = p20;
                    u14 >>= 2;
                    while u14 != 0 {
                        ww(p22, rw(p21));
                        p21 = p21.wrapping_add(4);
                        p22 = p22.wrapping_add(4);
                        u14 -= 1;
                    }
                    p20 = p21.wrapping_add(1);
                    readtok!(p21);
                    disp!(JA, LITA, RUNA);
                }
                RUNA => {
                    u14 &= 0xff;
                    st = RUNA2;
                }
                RUNA2 => {
                    let v = rb(p20);
                    p21 = p20.wrapping_add(1);
                    u15 = u14 & 3;
                    while u15 != 0 {
                        wb(p22, v);
                        p22 = p22.wrapping_add(1);
                        u15 -= 1;
                    }
                    u14 >>= 2;
                    while u14 != 0 {
                        ww(p22, (v as u32).wrapping_mul(0x01010101));
                        p22 = p22.wrapping_add(4);
                        u14 -= 1;
                    }
                    p20 = p20.wrapping_add(2);
                    readtok!(p21);
                    u14 = cnt as u32;
                    if bt == 0 && cnt != 0 {
                        st = RUNA;
                    } else if bt == 0 {
                        st = END;
                    } else if cnt != 0 {
                        st = LITA;
                    } else {
                        st = JA;
                    }
                }
                B0 => {
                    readtok!(p20);
                    p20 = p20.wrapping_add(1);
                    disp!(JB, LITB, RUNB);
                }
                JB => {
                    if bt == 0 {
                        st = END;
                    } else {
                        skipstep!();
                        disp!(JB, LITB, RUNB);
                    }
                }
                LITB => {
                    st = LITB2;
                }
                LITB2 => {
                    if (local_30 as i32) < (p22 as i32) {
                        st = T2;
                        continue;
                    }
                    clip!();
                    let n = u14.wrapping_sub(u17);
                    u15 = n & 3;
                    while u15 != 0 {
                        wb(p22, rb(p20));
                        p20 = p20.wrapping_add(1);
                        p22 = p22.wrapping_add(1);
                        u15 -= 1;
                    }
                    u14 = n >> 2;
                    while u14 != 0 {
                        ww(p22, rw(p20));
                        p20 = p20.wrapping_add(4);
                        p22 = p22.wrapping_add(4);
                        u14 -= 1;
                    }
                    p22 = p22.wrapping_add(u17);
                    let v = rb(p20.wrapping_add(u17));
                    p20 = p20.wrapping_add(u17).wrapping_add(1);
                    bt = v & 1;
                    cnt = v >> 1;
                    disp!(JB, LITB, RUNB);
                }
                RUNB => {
                    st = RUNB2;
                }
                RUNB2 => {
                    if (local_30 as i32) < (p22 as i32) {
                        st = T1;
                        continue;
                    }
                    clip!();
                    let n = u14.wrapping_sub(u17);
                    let v = rb(p20);
                    p21 = p20.wrapping_add(1);
                    u15 = n & 3;
                    while u15 != 0 {
                        wb(p22, v);
                        p22 = p22.wrapping_add(1);
                        u15 -= 1;
                    }
                    u14 = n >> 2;
                    while u14 != 0 {
                        ww(p22, (v as u32).wrapping_mul(0x01010101));
                        p22 = p22.wrapping_add(4);
                        u14 -= 1;
                    }
                    p22 = p22.wrapping_add(u17);
                    p20 = p20.wrapping_add(2);
                    readtok!(p21);
                    u14 = cnt as u32;
                    if bt == 0 && cnt != 0 {
                        st = RUNB;
                    } else if bt == 0 {
                        st = END;
                    } else if cnt != 0 {
                        st = LITB;
                    } else {
                        st = JB;
                    }
                }
                C0 => {
                    readtok!(p20);
                    p20 = p20.wrapping_add(1);
                    disp!(JC, LITC, RUNC);
                }
                JC => {
                    if bt == 0 {
                        st = END;
                    } else {
                        skipstep!();
                        disp!(JC, LITC, RUNC);
                    }
                }
                LITC => {
                    let d = local_2c.wrapping_sub(p22) as i32;
                    if d < (u14 as i32) {
                        if -1 < d {
                            p22 = p22.wrapping_add(d as u32);
                            u14 = u14.wrapping_sub(d as u32);
                            p20 = p20.wrapping_add(d as u32);
                        }
                        if (b18 & 4) == 0 {
                            st = LIT2;
                        } else {
                            st = LITB2;
                        }
                        continue;
                    }
                    p22 = p22.wrapping_add(u14);
                    let v = rb(p20.wrapping_add(u14));
                    p20 = p20.wrapping_add(u14).wrapping_add(1);
                    bt = v & 1;
                    cnt = v >> 1;
                    disp!(JC, LITC, RUNC);
                }
                RUNC => {
                    let d = local_2c.wrapping_sub(p22) as i32;
                    if d < (u14 as i32) {
                        if -1 < d {
                            p22 = p22.wrapping_add(d as u32);
                            u14 = u14.wrapping_sub(d as u32);
                        }
                        if (b18 & 4) == 0 {
                            st = RUNA2;
                        } else {
                            st = RUNB2;
                        }
                        continue;
                    }
                    p22 = p22.wrapping_add(u14);
                    p21 = p20.wrapping_add(1);
                    p20 = p20.wrapping_add(2);
                    readtok!(p21);
                    u14 = cnt as u32;
                    if bt == 0 && cnt != 0 {
                        st = RUNC;
                    } else if bt == 0 {
                        st = END;
                    } else if cnt != 0 {
                        st = LITC;
                    } else {
                        st = JC;
                    }
                }
                T1 => {
                    loop {
                        p22 = p20.wrapping_add(1);
                        p20 = p20.wrapping_add(2);
                        readtok!(p22);
                        if !(bt == 0 && cnt != 0) {
                            break;
                        }
                    }
                    if bt != 0 {
                        if cnt == 0 {
                            st = T3;
                        } else {
                            u14 = cnt as u32;
                            st = T2;
                        }
                    } else {
                        st = END;
                    }
                }
                T2 => {
                    let v = rb(p20.wrapping_add(u14));
                    p20 = p20.wrapping_add(u14).wrapping_add(1);
                    bt = v & 1;
                    cnt = v >> 1;
                    if bt == 0 && cnt != 0 {
                        st = T1;
                    } else if cnt != 0 {
                        u14 = cnt as u32;
                        st = T2;
                    } else if bt == 0 {
                        st = END;
                    } else {
                        st = T3;
                    }
                }
                _ => {
                    p22 = p20.wrapping_add(1);
                    p20 = p20.wrapping_add(2);
                    readtok!(p22);
                    if bt == 0 && cnt != 0 {
                        st = T1;
                    } else if cnt != 0 {
                        u14 = cnt as u32;
                        st = T2;
                    } else if bt == 0 {
                        st = END;
                    } else {
                        st = T3;
                    }
                }
            }
        }
    }
}
