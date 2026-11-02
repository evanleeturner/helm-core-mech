// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10082064: i32;
    static mut DAT_10082058: i32;
    static mut DAT_1008205c: i32;
    static mut DAT_10082060: u32;
    static mut DAT_10082070: u32;
    static mut DAT_10082074: *mut u8;
    static mut DAT_1008207c: *mut u32;
    static mut DAT_10082080: *mut u32;
    static mut DAT_10082084: *mut u32;
    static mut DAT_10082088: *mut u32;
    static mut DAT_1008208c: *mut u32;
    static mut DAT_10082090: *mut u32;
    static mut DAT_10082094: *mut u32;
    static mut DAT_10082098: u32;
    static mut DAT_1008209c: u32;
    static mut _DAT_100820a0: i32;
    static mut DAT_100820a4: u32;
    static mut DAT_100820a8: i32;
    static mut DAT_100820ac: i32;
    static mut DAT_100820c8: i32;
    static mut DAT_100820cc: i32;
    static mut DAT_100820e8: u32;
    static mut DAT_100820ec: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

unsafe fn rd(p: *const u32, i: usize) -> u32 {
    // SAFETY: the caller supplies a valid vertex pointer, as the original does.
    unsafe { p.wrapping_add(i).read_unaligned() }
}

unsafe fn qdiv(e: i32, d: i32) -> i32 {
    // SAFETY: RaiseException is the OS routine; arguments match the original.
    unsafe {
        if d == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
            0
        } else if d == -1 {
            if e != 0 {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
            }
            0
        } else {
            let q = (((e as u32 as u64) << 32) as i64) / (d as i64);
            if q != (q as i32) as i64 {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
                0
            } else {
                q as i32
            }
        }
    }
}

unsafe fn xl(pb: *mut u8, k: i32, tbl: i32) {
    // SAFETY: pb points into the caller's pixel row; tbl is the caller's lookup table.
    unsafe {
        let p = pb.byte_add((k * 2) as usize).cast::<u16>();
        let w = p.read_unaligned();
        let hi = *((((w >> 8) as u8) as u32).wrapping_add(tbl as u32) as usize as *const u8);
        let lo = *((((w as u8) as u32).wrapping_add(tbl as u32)) as usize as *const u8);
        let nw = (((hi as u32) << 8) | (lo as u32)) as u16;
        p.write_unaligned(nw);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100026c1(
    param_1: *mut i32,
    param_2: i32,
    param_3: *mut u32,
    param_4: i32,
) {
    // SAFETY: the globals are the game's own memory; the pointers are the
    // caller's structures, read and written where the original does.
    unsafe {
        let pi1 = (param_1.read_unaligned() as u32 as usize) as *mut i32;
        let p1 = |i: usize| -> i32 { param_1.wrapping_add(i).read_unaligned() };
        let q1 = |i: usize| -> i32 { pi1.wrapping_add(i).read_unaligned() };
        DAT_10082064 = (q1(1) as u32).wrapping_add(1) as i32;
        let mut i_var4 = q1(1);
        if p1(3) <= q1(1) {
            i_var4 = p1(3);
        }
        let mut u_var10: u32 = 0;
        if (p1(1) as u32) < 0x80000000 {
            u_var10 = p1(1) as u32;
        }
        let i_var5 = (i_var4 as u32).wrapping_sub(u_var10) as i32;
        if !((u_var10 as i32) <= i_var4) {
            return;
        }
        i_var4 = q1(2);
        if p1(4) <= q1(2) {
            i_var4 = p1(4);
        }
        let mut u_var12: u32 = 0;
        if (p1(2) as u32) < 0x80000000 {
            u_var12 = p1(2) as u32;
        }
        let i_var6 = (i_var4 as u32).wrapping_sub(u_var12) as i32;
        DAT_10082058 = i_var5;
        if !((u_var12 as i32) <= i_var4) {
            return;
        }
        DAT_10082060 = u_var12
            .wrapping_mul(DAT_10082064 as u32)
            .wrapping_add(u_var10)
            .wrapping_add(q1(0) as u32);
        DAT_10082080 = param_3.wrapping_offset(param_2.wrapping_mul(6) as isize);
        DAT_1008207c = param_3;
        u_var10 = 0x7fff;
        u_var12 = 0xffff8000;
        let mut u_var7: u32 = 0xf;
        let mut pu8 = param_3;
        loop {
            let u_var2 = rd(pu8, 1);
            if (u_var2 as i32) <= (u_var10 as i32) {
                u_var10 = u_var2;
                DAT_10082084 = pu8;
            }
            if (u_var12 as i32) <= (u_var2 as i32) {
                u_var12 = u_var2;
            }
            let a = rd(pu8, 0);
            let b = rd(pu8, 1);
            u_var7 &= (((((a >> 0x1f) << 1) | ((i_var5 as u32).wrapping_sub(a) >> 0x1f)) << 1
                | (b >> 0x1f))
                << 1)
                | ((i_var6 as u32).wrapping_sub(b) >> 0x1f);
            pu8 = pu8.wrapping_add(6);
            if pu8 == DAT_10082080 {
                break;
            }
        }
        DAT_1008205c = i_var6;
        if u_var7 != 0 {
            return;
        }
        DAT_10082090 = DAT_10082084;
        DAT_10082094 = DAT_10082084;
        pu8 = DAT_10082090;
        DAT_100820a4 = u_var10;
        if u_var12 == u_var10 {
            return;
        }
        loop {
            DAT_10082090 = pu8;
            DAT_10082088 = DAT_10082090;
            pu8 = DAT_10082090.wrapping_sub(6);
            if (pu8 as usize as i32) < (param_3 as usize as i32) {
                pu8 = DAT_10082080.wrapping_sub(6);
            }
            if (rd(DAT_10082090, 1) as i32) < 0 && (rd(pu8, 1) as i32) < 1 {
                continue;
            }
            DAT_10082098 = rd(pu8, 1).wrapping_sub(rd(DAT_10082090, 1));
            if DAT_10082098 == 0 {
                continue;
            }
            break;
        }
        let mut e_ = (rd(pu8, 0)
            .wrapping_sub(rd(DAT_10082090, 0))
            .wrapping_mul(0x10000) as i32)
            >> 0x10;
        let mut d_ = DAT_10082098.wrapping_mul(0x10000) as i32;
        DAT_100820c8 = qdiv(e_, d_);
        DAT_100820a8 = rd(DAT_10082090, 0)
            .wrapping_mul(0x10000)
            .wrapping_add(0x8000) as i32;
        let mut pu11 = DAT_10082094;
        loop {
            DAT_10082094 = pu11;
            DAT_1008208c = DAT_10082094;
            pu11 = DAT_10082094.wrapping_add(6);
            if (DAT_10082080 as usize as i32) <= (DAT_10082094.wrapping_add(6) as usize as i32) {
                pu11 = param_3;
            }
            if (rd(DAT_10082094, 1) as i32) < 0 && (rd(pu11, 1) as i32) < 1 {
                continue;
            }
            DAT_1008209c = rd(pu11, 1).wrapping_sub(rd(DAT_10082094, 1));
            if DAT_1008209c == 0 {
                continue;
            }
            break;
        }
        e_ = (rd(pu11, 0)
            .wrapping_sub(rd(DAT_10082094, 0))
            .wrapping_mul(0x10000) as i32)
            >> 0x10;
        d_ = DAT_1008209c.wrapping_mul(0x10000) as i32;
        DAT_100820cc = qdiv(e_, d_);
        DAT_100820ac = rd(DAT_10082094, 0)
            .wrapping_mul(0x10000)
            .wrapping_add(0x8000) as i32;
        _DAT_100820a0 = (i_var6 as u32).wrapping_sub(u_var10) as i32;
        if u_var12.wrapping_sub(i_var6 as u32) == 0 || (u_var12 as i32) < i_var6 {
            _DAT_100820a0 =
                (_DAT_100820a0 as u32).wrapping_add(u_var12.wrapping_sub(i_var6 as u32)) as i32;
        }
        if 0x7fffffffu32 < u_var10 {
            _DAT_100820a0 = (_DAT_100820a0 as u32).wrapping_add(u_var10) as i32;
            DAT_100820a4 = 0;
            DAT_10082098 = DAT_10082098.wrapping_add(rd(DAT_10082090, 1));
            let l3 = (DAT_100820c8 as i64)
                .wrapping_mul(rd(DAT_10082090, 1).wrapping_mul(0xffff0000) as i32 as i64);
            DAT_100820a8 = (DAT_100820a8 as u32)
                .wrapping_add(((l3 as u32) >> 0x10) | ((((l3 as u64) >> 0x20) as u32) << 0x10))
                as i32;
            DAT_1008209c = DAT_1008209c.wrapping_add(rd(DAT_10082094, 1));
            let l3 = (DAT_100820cc as i64)
                .wrapping_mul(rd(DAT_10082094, 1).wrapping_mul(0xffff0000) as i32 as i64);
            DAT_100820ac = (DAT_100820ac as u32)
                .wrapping_add(((l3 as u32) >> 0x10) | ((((l3 as u64) >> 0x20) as u32) << 0x10))
                as i32;
        }
        DAT_10082070 = DAT_100820a4
            .wrapping_mul(DAT_10082064 as u32)
            .wrapping_add(DAT_10082060);
        i_var4 = DAT_100820a8;
        let mut i_var5 = DAT_100820ac;
        DAT_10082090 = pu8;
        DAT_10082094 = pu11;
        loop {
            let mut i_var6 = i_var4;
            let mut i_var9 = i_var5;
            if i_var5 <= i_var4 {
                i_var6 = i_var5;
                i_var9 = i_var4;
            }
            u_var10 = (i_var6 >> 0x10) as u32;
            if (u_var10 as i32) <= DAT_10082058 {
                u_var12 = (i_var9 >> 0x10) as u32;
                if -1 < (u_var12 as i32) {
                    DAT_100820e8 = u_var10;
                    DAT_100820ec = u_var12;
                    if u_var12 != u_var10 {
                        if 0x7fffffffu32 < u_var10 {
                            DAT_100820e8 = 0;
                        }
                        if u_var12.wrapping_sub(DAT_10082058 as u32) != 0
                            && DAT_10082058 <= (u_var12 as i32)
                        {
                            DAT_100820ec =
                                u_var12.wrapping_sub(u_var12.wrapping_sub(DAT_10082058 as u32));
                        }
                    }
                    let mut pb = DAT_10082070.wrapping_add(DAT_100820e8) as usize as *mut u8;
                    u_var12 = DAT_100820ec.wrapping_sub(DAT_100820e8);
                    DAT_10082074 = pb.wrapping_add(u_var12 as usize);
                    u_var10 = u_var12.wrapping_add(1);
                    'blk: {
                        if (pb as usize as u32) & 1 != 0 {
                            *pb =
                                *((*pb as u32).wrapping_add(param_4 as u32) as usize as *const u8);
                            if u_var12 == 0 {
                                break 'blk;
                            }
                            pb = pb.wrapping_add(1);
                            u_var10 = u_var12;
                        }
                        if u_var10 != 1 {
                            i_var6 = (u_var10 >> 1).wrapping_sub(1) as i32;
                            loop {
                                if i_var6 < 5 {
                                    let mut k: i32 = 0;
                                    while k < 5 {
                                        if k > 0 && !(-1 < i_var6.wrapping_sub(k)) {
                                            break;
                                        }
                                        xl(pb, k, param_4);
                                        k += 1;
                                    }
                                    break;
                                }
                                let mut k: i32 = 0;
                                while k < 6 {
                                    xl(pb, k, param_4);
                                    k += 1;
                                }
                                pb = pb.wrapping_add(0xc);
                                i_var6 = i_var6.wrapping_sub(6);
                                if !(-1 < i_var6) {
                                    break;
                                }
                            }
                            if (u_var10 & 1) == 0 {
                                break 'blk;
                            }
                        }
                        *DAT_10082074 = *((*DAT_10082074 as u32).wrapping_add(param_4 as u32)
                            as usize as *const u8);
                    }
                }
            }
            DAT_10082070 = DAT_10082070.wrapping_add(DAT_10082064 as u32);
            _DAT_100820a0 = _DAT_100820a0.wrapping_sub(1);
            if !(-1 < _DAT_100820a0) {
                return;
            }
            if _DAT_100820a0 == 0 {
                i_var4 = i_var4.wrapping_add(DAT_100820c8);
                i_var5 = i_var5.wrapping_add(DAT_100820cc);
            } else {
                DAT_10082098 = DAT_10082098.wrapping_sub(1);
                if DAT_10082098 == 0 {
                    DAT_10082088 = DAT_10082090;
                    let mut p8 = DAT_10082090.wrapping_sub(6);
                    if (p8 as usize as i32) < (DAT_1008207c as usize as i32) {
                        p8 = DAT_10082080.wrapping_sub(6);
                    }
                    let df = rd(p8, 1).wrapping_sub(rd(DAT_10082090, 1));
                    DAT_10082098 = df.wrapping_add((df == 0) as u32);
                    e_ = (rd(p8, 0)
                        .wrapping_sub(rd(DAT_10082090, 0))
                        .wrapping_mul(0x10000) as i32)
                        >> 0x10;
                    d_ = DAT_10082098.wrapping_mul(0x10000) as i32;
                    DAT_100820c8 = qdiv(e_, d_);
                    i_var4 = rd(DAT_10082090, 0)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000) as i32;
                    DAT_10082090 = p8;
                } else {
                    i_var4 = i_var4.wrapping_add(DAT_100820c8);
                }
                DAT_1008209c = DAT_1008209c.wrapping_sub(1);
                if DAT_1008209c == 0 {
                    DAT_1008208c = DAT_10082094;
                    let mut p8 = DAT_10082094.wrapping_add(6);
                    if (DAT_10082080 as usize as i32)
                        <= (DAT_10082094.wrapping_add(6) as usize as i32)
                    {
                        p8 = DAT_1008207c;
                    }
                    let df = rd(p8, 1).wrapping_sub(rd(DAT_10082094, 1));
                    DAT_1008209c = df.wrapping_add((df == 0) as u32);
                    e_ = (rd(p8, 0)
                        .wrapping_sub(rd(DAT_10082094, 0))
                        .wrapping_mul(0x10000) as i32)
                        >> 0x10;
                    d_ = DAT_1008209c.wrapping_mul(0x10000) as i32;
                    DAT_100820cc = qdiv(e_, d_);
                    i_var5 = rd(DAT_10082094, 0)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000) as i32;
                    DAT_10082094 = p8;
                } else {
                    i_var5 = i_var5.wrapping_add(DAT_100820cc);
                }
            }
        }
    }
}
