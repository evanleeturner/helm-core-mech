// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af324: i32;
    static mut DAT_100af318: i32;
    static mut DAT_100af31c: i32;
    static mut DAT_100af320: i32;
    static mut DAT_100af330: u32;
    static mut DAT_100af334: *mut u8;
    static mut DAT_100af33c: *mut u32;
    static mut DAT_100af340: *mut u32;
    static mut DAT_100af344: *mut u32;
    static mut DAT_100af348: *mut u32;
    static mut DAT_100af34c: *mut u32;
    static mut DAT_100af350: *mut u32;
    static mut DAT_100af354: *mut u32;
    static mut DAT_100af358: i32;
    static mut DAT_100af35c: i32;
    static mut _DAT_100af360: i32;
    static mut DAT_100af364: i32;
    static mut DAT_100af368: i32;
    static mut DAT_100af36c: i32;
    static mut DAT_100af388: i32;
    static mut DAT_100af38c: i32;
    static mut DAT_100af3a8: u32;
    static mut DAT_100af3ac: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

unsafe fn rd(p: *const u32, i: usize) -> u32 {
    // SAFETY: caller supplies a pointer to the game's vertex data.
    unsafe { p.add(i).read_unaligned() }
}

unsafe fn fdiv(num: i64, den: i32, dst: *mut i32) {
    // SAFETY: the exception call and the destination global are the game's own.
    unsafe {
        if den == 0 || (den == -1 && num == i64::MIN) {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        } else {
            let q = num / (den as i64);
            if q != (q as i32) as i64 {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
            } else {
                dst.write(q as i32);
            }
        }
    }
}

fn slope_num(a: u32, b: u32) -> i64 {
    let t = (a.wrapping_sub(b).wrapping_mul(0x10000) as i32) >> 0x10;
    (((t as u32 as u64) << 32) as i64)
}

fn mulshift(a: i32, b: u32) -> u32 {
    let l = (a as i64).wrapping_mul((b.wrapping_mul(0xFFFF0000) as i32) as i64);
    ((l as u32) >> 16) | (((((l as u64) >> 32) as i32) as u32) << 16)
}

unsafe fn xlat(p: *mut u8, tbl: *const u8) {
    // SAFETY: p points into the destination row, tbl at a 256-byte table.
    unsafe {
        let v = p.cast::<u16>().read_unaligned();
        let hi = tbl.add((v >> 8) as u8 as usize).read() as u32;
        let lo = tbl.add(v as u8 as usize).read() as u32;
        p.cast::<u16>().write_unaligned(((hi << 8) | lo) as u16);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10040141(
    param_1: *mut i32,
    param_2: i32,
    param_3: *mut u32,
    param_4: i32,
) {
    // SAFETY: all globals, pointers and the callee belong to the game; the
    // original's reads and writes are reproduced in order.
    unsafe {
        let tbl = param_4 as *const u8;
        let pvar1 = param_1.read() as *mut i32;
        let p1_0 = pvar1.read();
        let p1_1 = pvar1.add(1).read();
        let p1_2 = pvar1.add(2).read();
        DAT_100af324 = (p1_1 as u32).wrapping_add(1) as i32;
        let mut ivar4 = p1_1;
        if param_1.add(3).read() <= p1_1 {
            ivar4 = param_1.add(3).read();
        }
        let mut uvar10: u32 = 0;
        if (param_1.add(1).read() as u32) < 0x80000000 {
            uvar10 = param_1.add(1).read() as u32;
        }
        let mut ivar5 = (ivar4 as u32).wrapping_sub(uvar10) as i32;
        if !((uvar10 as i32) <= ivar4) {
            return;
        }
        ivar4 = p1_2;
        if param_1.add(4).read() <= p1_2 {
            ivar4 = param_1.add(4).read();
        }
        let mut uvar12: u32 = 0;
        if (param_1.add(2).read() as u32) < 0x80000000 {
            uvar12 = param_1.add(2).read() as u32;
        }
        let mut ivar6 = (ivar4 as u32).wrapping_sub(uvar12) as i32;
        DAT_100af318 = ivar5;
        if !((uvar12 as i32) <= ivar4) {
            return;
        }
        DAT_100af320 = uvar12
            .wrapping_mul(DAT_100af324 as u32)
            .wrapping_add(uvar10)
            .wrapping_add(p1_0 as u32) as i32;
        DAT_100af340 = param_3.wrapping_offset((param_2 as u32).wrapping_mul(6) as i32 as isize);
        DAT_100af33c = param_3;
        uvar10 = 0x7fff;
        uvar12 = 0xffff8000;
        let mut uvar7: u32 = 0xf;
        let mut puvar8 = param_3;
        loop {
            let uvar2 = rd(puvar8, 1);
            if (uvar2 as i32) <= (uvar10 as i32) {
                uvar10 = uvar2;
                DAT_100af344 = puvar8;
            }
            if (uvar12 as i32) <= (uvar2 as i32) {
                uvar12 = uvar2;
            }
            let a = rd(puvar8, 0);
            let b = (ivar5 as u32).wrapping_sub(a) >> 0x1f;
            let c = rd(puvar8, 1) >> 0x1f;
            let d = (ivar6 as u32).wrapping_sub(rd(puvar8, 1)) >> 0x1f;
            let t = (((((a >> 0x1f) << 1) | b) << 1) | c) << 1 | d;
            uvar7 &= t;
            puvar8 = puvar8.wrapping_add(6);
            if puvar8 == DAT_100af340 {
                break;
            }
        }
        DAT_100af31c = ivar6;
        if uvar7 != 0 {
            return;
        }
        DAT_100af350 = DAT_100af344;
        DAT_100af354 = DAT_100af344;
        puvar8 = DAT_100af350;
        DAT_100af364 = uvar10 as i32;
        if uvar12 == uvar10 {
            return;
        }
        loop {
            DAT_100af350 = puvar8;
            DAT_100af348 = DAT_100af350;
            puvar8 = DAT_100af350.wrapping_sub(6);
            if (puvar8 as i32) < (param_3 as i32) {
                puvar8 = DAT_100af340.wrapping_sub(6);
            }
            let c = ((rd(DAT_100af350, 1) as i32) < 0 && (rd(puvar8, 1) as i32) < 1) || {
                DAT_100af358 = rd(puvar8, 1).wrapping_sub(rd(DAT_100af350, 1)) as i32;
                DAT_100af358 == 0
            };
            if !c {
                break;
            }
        }
        let l3 = slope_num(rd(puvar8, 0), rd(DAT_100af350, 0));
        let i15 = (DAT_100af358 as u32).wrapping_mul(0x10000) as i32;
        fdiv(l3, i15, &raw mut DAT_100af388);
        DAT_100af368 = rd(DAT_100af350, 0)
            .wrapping_mul(0x10000)
            .wrapping_add(0x8000) as i32;
        let mut puvar11 = DAT_100af354;
        loop {
            DAT_100af354 = puvar11;
            DAT_100af34c = DAT_100af354;
            puvar11 = DAT_100af354.wrapping_add(6);
            if (DAT_100af340 as i32) <= (puvar11 as i32) {
                puvar11 = param_3;
            }
            let c = ((rd(DAT_100af354, 1) as i32) < 0 && (rd(puvar11, 1) as i32) < 1) || {
                DAT_100af35c = rd(puvar11, 1).wrapping_sub(rd(DAT_100af354, 1)) as i32;
                DAT_100af35c == 0
            };
            if !c {
                break;
            }
        }
        let l3 = slope_num(rd(puvar11, 0), rd(DAT_100af354, 0));
        let i15 = (DAT_100af35c as u32).wrapping_mul(0x10000) as i32;
        fdiv(l3, i15, &raw mut DAT_100af38c);
        DAT_100af36c = rd(DAT_100af354, 0)
            .wrapping_mul(0x10000)
            .wrapping_add(0x8000) as i32;
        _DAT_100af360 = (ivar6 as u32).wrapping_sub(uvar10) as i32;
        if uvar12.wrapping_sub(ivar6 as u32) == 0 || (uvar12 as i32) < ivar6 {
            _DAT_100af360 =
                (_DAT_100af360 as u32).wrapping_add(uvar12.wrapping_sub(ivar6 as u32)) as i32;
        }
        if 0x7fffffff < uvar10 {
            _DAT_100af360 = (_DAT_100af360 as u32).wrapping_add(uvar10) as i32;
            DAT_100af364 = 0;
            DAT_100af358 = (DAT_100af358 as u32).wrapping_add(rd(DAT_100af350, 1)) as i32;
            DAT_100af368 = (DAT_100af368 as u32)
                .wrapping_add(mulshift(DAT_100af388, rd(DAT_100af350, 1)))
                as i32;
            DAT_100af35c = (DAT_100af35c as u32).wrapping_add(rd(DAT_100af354, 1)) as i32;
            DAT_100af36c = (DAT_100af36c as u32)
                .wrapping_add(mulshift(DAT_100af38c, rd(DAT_100af354, 1)))
                as i32;
        }
        DAT_100af330 = (DAT_100af364 as u32)
            .wrapping_mul(DAT_100af324 as u32)
            .wrapping_add(DAT_100af320 as u32);
        ivar4 = DAT_100af368;
        ivar5 = DAT_100af36c;
        DAT_100af350 = puvar8;
        DAT_100af354 = puvar11;
        loop {
            ivar6 = ivar4;
            let mut ivar9 = ivar5;
            if ivar5 <= ivar4 {
                ivar6 = ivar5;
                ivar9 = ivar4;
            }
            uvar10 = (ivar6 >> 0x10) as u32;
            'body: {
                if !((uvar10 as i32) <= DAT_100af318) {
                    break 'body;
                }
                uvar12 = (ivar9 >> 0x10) as u32;
                if !(-1 < (uvar12 as i32)) {
                    break 'body;
                }
                DAT_100af3a8 = uvar10;
                DAT_100af3ac = uvar12;
                if uvar12 != uvar10 {
                    if 0x7fffffff < uvar10 {
                        DAT_100af3a8 = 0;
                    }
                    if uvar12.wrapping_sub(DAT_100af318 as u32) != 0
                        && DAT_100af318 <= (uvar12 as i32)
                    {
                        DAT_100af3ac =
                            uvar12.wrapping_sub(uvar12.wrapping_sub(DAT_100af318 as u32));
                    }
                }
                let mut pbvar13 = DAT_100af330.wrapping_add(DAT_100af3a8) as usize as *mut u8;
                uvar12 = DAT_100af3ac.wrapping_sub(DAT_100af3a8);
                DAT_100af334 = pbvar13.wrapping_add(uvar12 as usize);
                uvar10 = uvar12.wrapping_add(1);
                if (pbvar13 as usize as u32) & 1 != 0 {
                    pbvar13.write(tbl.add(pbvar13.read() as usize).read());
                    if uvar12 == 0 {
                        break 'body;
                    }
                    pbvar13 = pbvar13.wrapping_add(1);
                    uvar10 = uvar12;
                }
                if uvar10 != 1 {
                    ivar6 = (uvar10 >> 1).wrapping_sub(1) as i32;
                    loop {
                        if ivar6 < 5 {
                            let mut k: u32 = 0;
                            while k < 5 {
                                if k > 0 && ((ivar6 as u32).wrapping_sub(k) as i32) < 0 {
                                    break;
                                }
                                xlat(pbvar13.wrapping_add((k * 2) as usize), tbl);
                                k += 1;
                            }
                            break;
                        }
                        xlat(pbvar13, tbl);
                        xlat(pbvar13.wrapping_add(2), tbl);
                        xlat(pbvar13.wrapping_add(4), tbl);
                        xlat(pbvar13.wrapping_add(6), tbl);
                        xlat(pbvar13.wrapping_add(8), tbl);
                        xlat(pbvar13.wrapping_add(10), tbl);
                        pbvar13 = pbvar13.wrapping_add(0xc);
                        ivar6 = (ivar6 as u32).wrapping_sub(6) as i32;
                        if !(-1 < ivar6) {
                            break;
                        }
                    }
                    if uvar10 & 1 == 0 {
                        break 'body;
                    }
                }
                DAT_100af334.write(tbl.add(DAT_100af334.read() as usize).read());
            }
            DAT_100af330 = DAT_100af330.wrapping_add(DAT_100af324 as u32);
            _DAT_100af360 = (_DAT_100af360 as u32).wrapping_sub(1) as i32;
            if !(-1 < _DAT_100af360) {
                return;
            }
            if _DAT_100af360 == 0 {
                ivar4 = (ivar4 as u32).wrapping_add(DAT_100af388 as u32) as i32;
                ivar5 = (ivar5 as u32).wrapping_add(DAT_100af38c as u32) as i32;
            } else {
                DAT_100af358 = (DAT_100af358 as u32).wrapping_sub(1) as i32;
                if DAT_100af358 == 0 {
                    DAT_100af348 = DAT_100af350;
                    puvar8 = DAT_100af350.wrapping_sub(6);
                    if (puvar8 as i32) < (DAT_100af33c as i32) {
                        puvar8 = DAT_100af340.wrapping_sub(6);
                    }
                    let d = rd(puvar8, 1).wrapping_sub(rd(DAT_100af350, 1));
                    DAT_100af358 = d.wrapping_add((d == 0) as u32) as i32;
                    let l3 = slope_num(rd(puvar8, 0), rd(DAT_100af350, 0));
                    let i15 = (DAT_100af358 as u32).wrapping_mul(0x10000) as i32;
                    fdiv(l3, i15, &raw mut DAT_100af388);
                    ivar4 = rd(DAT_100af350, 0)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000) as i32;
                    DAT_100af350 = puvar8;
                } else {
                    ivar4 = (ivar4 as u32).wrapping_add(DAT_100af388 as u32) as i32;
                }
                DAT_100af35c = (DAT_100af35c as u32).wrapping_sub(1) as i32;
                if DAT_100af35c == 0 {
                    DAT_100af34c = DAT_100af354;
                    puvar8 = DAT_100af354.wrapping_add(6);
                    if (DAT_100af340 as i32) <= (puvar8 as i32) {
                        puvar8 = DAT_100af33c;
                    }
                    let d = rd(puvar8, 1).wrapping_sub(rd(DAT_100af354, 1));
                    DAT_100af35c = d.wrapping_add((d == 0) as u32) as i32;
                    let l3 = slope_num(rd(puvar8, 0), rd(DAT_100af354, 0));
                    let i15 = (DAT_100af35c as u32).wrapping_mul(0x10000) as i32;
                    fdiv(l3, i15, &raw mut DAT_100af38c);
                    ivar5 = rd(DAT_100af354, 0)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000) as i32;
                    DAT_100af354 = puvar8;
                } else {
                    ivar5 = (ivar5 as u32).wrapping_add(DAT_100af38c as u32) as i32;
                }
            }
        }
    }
}
