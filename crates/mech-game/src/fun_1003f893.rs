// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af324: i32;
    static mut DAT_100af318: i32;
    static mut DAT_100af31c: i32;
    static mut DAT_100af320: i32;
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
    static mut DAT_100af364: i32;
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
    static mut DAT_100af3c8: i32;
    static mut DAT_100af3cc: i32;
    static mut DAT_100af3d0: *mut c_void;
    static mut DAT_100af3d4: i32;
    static mut DAT_100af334: *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

fn carry(a: u32, b: u32) -> u8 {
    (a.wrapping_add(b) < a) as u8
}

fn carry_at(a: u32, s: u32, k: u32) -> u8 {
    carry(a.wrapping_add(s.wrapping_mul(k)), s)
}

fn cat11(hi: u8, lo: u8) -> u16 {
    ((hi as u16) << 8) | (lo as u16)
}

fn mulshift(l: i64) -> u32 {
    ((l as u32) >> 16) | ((((l as u64) >> 32) as u32).wrapping_shl(16))
}

fn mk(hi: u32, lo: u32) -> i64 {
    (((hi as u64) << 32) | (lo as u64)) as i64
}

fn hdiv(x: u32) -> i64 {
    mk(((x.wrapping_mul(0x10000)) as i32 >> 16) as u32, 0)
}

fn ldiv(d: u32) -> i64 {
    mk(((d as i32) >> 16) as u32, d.wrapping_mul(0x10000))
}

unsafe fn rd(p: *const u32, i: usize) -> u32 {
    // SAFETY: the caller passes a pointer into the game's vertex array.
    unsafe { p.add(i).read_unaligned() }
}

unsafe fn rdi(p: *const i32, i: usize) -> i32 {
    // SAFETY: the caller passes a pointer into the game's own structures.
    unsafe { p.add(i).read_unaligned() }
}

unsafe fn fdiv(n: i64, d: i32) -> Option<i32> {
    if d == 0 || (d == -1 && n == i64::MIN) {
        // SAFETY: raises the same divide exception the original raises.
        unsafe { RaiseException(0xC000_0094, 0, 0, core::ptr::null()) };
        None
    } else {
        let q = n / (d as i64);
        if q != ((q as i32) as i64) {
            // SAFETY: raises the same divide exception the original raises.
            unsafe { RaiseException(0xC000_0094, 0, 0, core::ptr::null()) };
            None
        } else {
            Some(q as i32)
        }
    }
}

#[allow(non_snake_case, unused_assignments)]
unsafe fn span_run(p: *mut u16, count: u32, s: u32, sb: u8, a: u32, b: u32, u6: u32) -> u32 {
    let mut p = p;
    let mut a = a;
    let mut b = b;
    let mut u6 = u6;
    let mut u5 = u6 as u16;
    let mut n = (count >> 1).wrapping_sub(1) as i32;
    loop {
        if n < 5 {
            // SAFETY: writes the pixels of the span into the caller's destination buffer.
            unsafe {
                p.write_unaligned(u5);
                let mut lo = (u5 as u8).wrapping_add(sb).wrapping_add(carry_at(a, s, 0));
                let mut hi = ((u5 >> 8) as u8)
                    .wrapping_add(sb)
                    .wrapping_add(carry_at(b, s, 0));
                u5 = cat11(hi, lo);
                u6 = u5 as u32;
                if -1 < n.wrapping_sub(1) {
                    p.add(1).write_unaligned(u5);
                    lo = lo.wrapping_add(sb).wrapping_add(carry_at(a, s, 1));
                    hi = hi.wrapping_add(sb).wrapping_add(carry_at(b, s, 1));
                    u5 = cat11(hi, lo);
                    u6 = u5 as u32;
                    if -1 < n.wrapping_sub(2) {
                        p.add(2).write_unaligned(u5);
                        lo = lo.wrapping_add(sb).wrapping_add(carry_at(a, s, 2));
                        hi = hi.wrapping_add(sb).wrapping_add(carry_at(b, s, 2));
                        u5 = cat11(hi, lo);
                        u6 = u5 as u32;
                        if -1 < n.wrapping_sub(3) {
                            p.add(3).write_unaligned(u5);
                            lo = lo.wrapping_add(sb).wrapping_add(carry_at(a, s, 3));
                            u5 = cat11(hi.wrapping_add(sb).wrapping_add(carry_at(b, s, 3)), lo);
                            u6 = u5 as u32;
                            if -1 < n.wrapping_sub(4) {
                                p.add(4).write_unaligned(u5);
                                u6 = lo.wrapping_add(sb).wrapping_add(carry_at(a, s, 4)) as u32;
                            }
                        }
                    }
                }
            }
            break;
        }
        // SAFETY: writes six pixels of the span into the caller's destination buffer.
        unsafe {
            p.write_unaligned(u6 as u16);
            let mut lo = (u6 as u8).wrapping_add(sb).wrapping_add(carry_at(a, s, 0));
            let mut hi = ((u6 >> 8) as u8)
                .wrapping_add(sb)
                .wrapping_add(carry_at(b, s, 0));
            p.add(1).write_unaligned(cat11(hi, lo));
            lo = lo.wrapping_add(sb).wrapping_add(carry_at(a, s, 1));
            hi = hi.wrapping_add(sb).wrapping_add(carry_at(b, s, 1));
            p.add(2).write_unaligned(cat11(hi, lo));
            lo = lo.wrapping_add(sb).wrapping_add(carry_at(a, s, 2));
            hi = hi.wrapping_add(sb).wrapping_add(carry_at(b, s, 2));
            p.add(3).write_unaligned(cat11(hi, lo));
            lo = lo.wrapping_add(sb).wrapping_add(carry_at(a, s, 3));
            hi = hi.wrapping_add(sb).wrapping_add(carry_at(b, s, 3));
            p.add(4).write_unaligned(cat11(hi, lo));
            lo = lo.wrapping_add(sb).wrapping_add(carry_at(a, s, 4));
            hi = hi.wrapping_add(sb).wrapping_add(carry_at(b, s, 4));
            p.add(5).write_unaligned(cat11(hi, lo));
            u5 = cat11(
                hi.wrapping_add(sb).wrapping_add(carry_at(b, s, 5)),
                lo.wrapping_add(sb).wrapping_add(carry_at(a, s, 5)),
            );
            u6 = u5 as u32;
            a = a.wrapping_add(s.wrapping_mul(6));
            b = b.wrapping_add(s.wrapping_mul(6));
            p = p.add(6);
        }
        n = n.wrapping_sub(6);
        if !(-1 < n) {
            break;
        }
    }
    u6
}

const S_JOINED: u8 = 0;
const S_A_BODY: u8 = 1;
const S_A_END: u8 = 2;
const S_A_TAIL: u8 = 3;
const S_L07: u8 = 4;
const S_FFA6: u8 = 5;
const S_CLIP_BODY: u8 = 6;
const S_CLIP_END: u8 = 7;
const S_FF92: u8 = 8;
const S_L0A5: u8 = 9;

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_mut, unused_variables)]
pub unsafe extern "C" fn FUN_1003f893(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: *mut u32,
) {
    // SAFETY: all globals are the game's own memory; the pointers passed in
    // reference the caller's structures, vertex array and destination buffer,
    // read and written exactly where the original does.
    unsafe {
        let piVar1 = (param_1.read_unaligned() as u32) as usize as *mut i32;
        DAT_100af324 = rdi(piVar1, 1).wrapping_add(1);
        let mut iVar10 = rdi(piVar1, 1);
        if rdi(param_1, 3) <= rdi(piVar1, 1) {
            iVar10 = rdi(param_1, 3);
        }
        let mut uVar21: u32 = 0;
        if (rdi(param_1, 1) as u32) < 0x8000_0000 {
            uVar21 = rdi(param_1, 1) as u32;
        }
        let mut iVar7 = (iVar10 as u32).wrapping_sub(uVar21) as i32;
        if (uVar21 as i32) > iVar10 {
            return;
        }
        iVar10 = rdi(piVar1, 2);
        if rdi(param_1, 4) <= rdi(piVar1, 2) {
            iVar10 = rdi(param_1, 4);
        }
        let mut uVar23: u32 = 0;
        if (rdi(param_1, 2) as u32) < 0x8000_0000 {
            uVar23 = rdi(param_1, 2) as u32;
        }
        let mut iVar8 = (iVar10 as u32).wrapping_sub(uVar23) as i32;
        DAT_100af318 = iVar7;
        if (uVar23 as i32) > iVar10 {
            return;
        }
        DAT_100af320 = uVar23
            .wrapping_mul(DAT_100af324 as u32)
            .wrapping_add(uVar21)
            .wrapping_add(rdi(piVar1, 0) as u32) as i32;
        DAT_100af340 = param_4.wrapping_offset(param_3.wrapping_mul(6) as isize);
        DAT_100af33c = param_4;
        DAT_100af338 = 0;
        uVar21 = 0x7fff;
        uVar23 = 0xffff_8000;
        let mut uVar9: u32 = 0xf;
        let mut puVar13 = param_4;
        loop {
            let uVar12 =
                (rd(puVar13, 0) >> 31) << 1 | ((iVar7 as u32).wrapping_sub(rd(puVar13, 0)) >> 31);
            DAT_100af338 |= uVar12;
            let uVar19 = rd(puVar13, 1);
            if (uVar19 as i32) <= (uVar21 as i32) {
                uVar21 = uVar19;
                DAT_100af344 = puVar13;
            }
            if (uVar23 as i32) <= (uVar19 as i32) {
                uVar23 = uVar19;
            }
            uVar9 &= ((uVar12 << 1 | rd(puVar13, 1) >> 31) << 1)
                | ((iVar8 as u32).wrapping_sub(rd(puVar13, 1)) >> 31);
            puVar13 = puVar13.wrapping_add(6);
            if puVar13 == DAT_100af340 {
                break;
            }
        }
        DAT_100af31c = iVar8;
        DAT_100af3d0 = (&raw const param_1).cast::<u8>().wrapping_sub(8) as *mut c_void;
        if uVar9 != 0 {
            return;
        }
        DAT_100af350 = DAT_100af344;
        DAT_100af354 = DAT_100af344;
        puVar13 = DAT_100af350;
        DAT_100af364 = uVar21 as i32;
        DAT_100af3d0 = (&raw const param_1).cast::<u8>().wrapping_sub(8) as *mut c_void;
        if uVar23 == uVar21 {
            return;
        }
        loop {
            DAT_100af350 = puVar13;
            puVar13 = DAT_100af350.wrapping_sub(6);
            if (puVar13 as usize as i32) < (param_4 as usize as i32) {
                puVar13 = DAT_100af340.wrapping_sub(6);
            }
            if (rd(DAT_100af350, 1) as i32) < 0 && (rd(puVar13, 1) as i32) < 1 {
                continue;
            }
            DAT_100af358 = rd(puVar13, 1).wrapping_sub(rd(DAT_100af350, 1)) as i32;
            if DAT_100af358 == 0 {
                continue;
            }
            break;
        }
        let d = (DAT_100af358 as u32).wrapping_mul(0x10000) as i32;
        if let Some(q) = fdiv(hdiv(rd(puVar13, 0).wrapping_sub(rd(DAT_100af350, 0))), d) {
            DAT_100af388 = q;
        }
        let d = (DAT_100af358 as u32).wrapping_mul(0x10000) as i32;
        if let Some(q) = fdiv(ldiv(rd(puVar13, 2).wrapping_sub(rd(DAT_100af350, 2))), d) {
            DAT_100af390 = q;
        }
        DAT_100af368 = rd(DAT_100af350, 0)
            .wrapping_mul(0x10000)
            .wrapping_add(0x8000) as i32;
        DAT_100af370 = rd(DAT_100af350, 2).wrapping_add(0x8000);
        let mut puVar22 = DAT_100af354;
        loop {
            DAT_100af354 = puVar22;
            puVar22 = DAT_100af354.wrapping_add(6);
            if (DAT_100af340 as usize as i32) <= (DAT_100af354.wrapping_add(6) as usize as i32) {
                puVar22 = param_4;
            }
            if (rd(DAT_100af354, 1) as i32) < 0 && (rd(puVar22, 1) as i32) < 1 {
                continue;
            }
            DAT_100af35c = rd(puVar22, 1).wrapping_sub(rd(DAT_100af354, 1)) as i32;
            if DAT_100af35c == 0 {
                continue;
            }
            break;
        }
        let d = (DAT_100af35c as u32).wrapping_mul(0x10000) as i32;
        if let Some(q) = fdiv(hdiv(rd(puVar22, 0).wrapping_sub(rd(DAT_100af354, 0))), d) {
            DAT_100af38c = q;
        }
        let d = (DAT_100af35c as u32).wrapping_mul(0x10000) as i32;
        if let Some(q) = fdiv(ldiv(rd(puVar22, 2).wrapping_sub(rd(DAT_100af354, 2))), d) {
            DAT_100af394 = q;
        }
        DAT_100af36c = rd(DAT_100af354, 0)
            .wrapping_mul(0x10000)
            .wrapping_add(0x8000) as i32;
        DAT_100af374 = rd(DAT_100af354, 2).wrapping_add(0x8000);
        DAT_100af360 = (iVar8 as u32).wrapping_sub(uVar21) as i32;
        if uVar23.wrapping_sub(iVar8 as u32) == 0 || (uVar23 as i32) < iVar8 {
            DAT_100af360 =
                (DAT_100af360 as u32).wrapping_add(uVar23.wrapping_sub(iVar8 as u32)) as i32;
        }
        DAT_100af3cc = 0;
        DAT_100af3c8 = param_2;
        if uVar21.wrapping_neg() != 0 && (uVar21 as i32) < 1 {
            DAT_100af360 = (DAT_100af360 as u32).wrapping_add(uVar21) as i32;
            if (uVar21.wrapping_neg() & 1) != 0 {
                DAT_100af3c8 = 0;
                DAT_100af3cc = param_2;
            }
            DAT_100af364 = 0;
            DAT_100af358 = (DAT_100af358 as u32).wrapping_add(rd(DAT_100af350, 1)) as i32;
            let m = rd(DAT_100af350, 1).wrapping_mul(0xFFFF_0000) as i32;
            let l = (DAT_100af388 as i64) * (m as i64);
            DAT_100af368 = (DAT_100af368 as u32).wrapping_add(mulshift(l)) as i32;
            let l = (DAT_100af390 as i64) * (m as i64);
            DAT_100af370 = DAT_100af370.wrapping_add(mulshift(l));
            DAT_100af35c = (DAT_100af35c as u32).wrapping_add(rd(DAT_100af354, 1)) as i32;
            let m = rd(DAT_100af354, 1).wrapping_mul(0xFFFF_0000) as i32;
            let l = (DAT_100af38c as i64) * (m as i64);
            DAT_100af36c = (DAT_100af36c as u32).wrapping_add(mulshift(l)) as i32;
            let l = (DAT_100af394 as i64) * (m as i64);
            DAT_100af374 = DAT_100af374.wrapping_add(mulshift(l));
        }
        iVar10 = (DAT_100af320 as u32)
            .wrapping_add((DAT_100af364 as u32).wrapping_mul(DAT_100af324 as u32))
            as i32;
        DAT_100af348 = DAT_100af350;
        DAT_100af34c = DAT_100af354;
        DAT_100af354 = puVar22;
        DAT_100af350 = puVar13;
        iVar7 = DAT_100af368;
        iVar8 = DAT_100af36c;
        uVar21 = DAT_100af370;
        uVar23 = DAT_100af374;
        DAT_100af3d0 = (&raw const param_1).cast::<u8>().wrapping_sub(8) as *mut c_void;

        let mut state = S_JOINED;
        loop {
            match state {
                S_JOINED => {
                    state = if DAT_100af338 != 0 {
                        S_CLIP_BODY
                    } else {
                        S_A_BODY
                    };
                }
                S_A_BODY => {
                    let mut iVar18 = iVar7;
                    let mut uVar19 = uVar21;
                    let mut uVar9 = uVar23;
                    let mut iVar15 = iVar8;
                    if iVar8 <= iVar7 {
                        iVar18 = iVar8;
                        uVar19 = uVar23;
                        uVar9 = uVar21;
                        iVar15 = iVar7;
                    }
                    let mut uVar12 = (iVar18 >> 16) as u32;
                    let mut puVar24 = (iVar10 as u32).wrapping_add(uVar12) as usize as *mut u16;
                    let uVar14 = ((iVar15 >> 16) as u32).wrapping_sub(uVar12);
                    if uVar14 != 0 {
                        let h = (uVar14 as i32) >> 1;
                        let divd =
                            ((h as u32).wrapping_add((h == 0) as u32)).wrapping_mul(0x10000) as i32;
                        if let Some(q) = fdiv(ldiv(uVar9.wrapping_sub(uVar19)), divd) {
                            uVar12 = q as u32;
                        }
                        uVar9 = uVar12 >> 16;
                    }
                    uVar12 = uVar12.wrapping_mul(0x10000);
                    DAT_100af334 = uVar14.wrapping_add(puVar24 as usize as u32) as usize as *mut i8;
                    iVar18 = uVar19.wrapping_add(DAT_100af3cc as u32) as i32;
                    let s8 = uVar19.wrapping_add(DAT_100af3c8 as u32);
                    let cVar3 = (s8 >> 16) as u8;
                    let cVar4 = uVar9 as u8;
                    let mut uVar6: u32 =
                        ((((iVar18 as u32) >> 16) as u16 as u32) << 8) | (cVar3 as u32);
                    uVar9 = s8.wrapping_mul(0x10000);
                    uVar19 = (iVar18 as u32).wrapping_mul(0x10000);
                    let mut uVar17 = uVar14.wrapping_add(1);
                    let mut go = false;
                    if (puVar24 as usize & 1) == 0 {
                        go = true;
                    } else {
                        (puVar24 as *mut u8).write(cVar3);
                        if uVar14 != 0 {
                            puVar24 = (puVar24 as *mut u8).add(1) as *mut u16;
                            let nb = cVar3.wrapping_add(cVar4).wrapping_add(carry(uVar9, uVar12));
                            uVar19 = uVar9.wrapping_add(uVar12);
                            uVar6 = cat11(nb, ((iVar18 as u32) >> 16) as u8) as u32;
                            uVar9 = (iVar18 as u32).wrapping_mul(0x10000);
                            uVar17 = uVar14;
                            go = true;
                        }
                    }
                    if go {
                        if uVar17 != 1 {
                            uVar6 = span_run(puVar24, uVar17, uVar12, cVar4, uVar9, uVar19, uVar6);
                            if (uVar17 & 1) != 0 {
                                DAT_100af334.write(uVar6 as i8);
                            }
                        } else {
                            DAT_100af334.write(uVar6 as i8);
                        }
                    }
                    state = S_A_END;
                }
                S_A_END => {
                    let t = DAT_100af3cc;
                    DAT_100af3cc = DAT_100af3c8;
                    DAT_100af3c8 = t;
                    iVar10 = (iVar10 as u32).wrapping_add(DAT_100af324 as u32) as i32;
                    DAT_100af360 = DAT_100af360.wrapping_sub(1);
                    if DAT_100af360 < 0 {
                        return;
                    }
                    if DAT_100af360 == 0 {
                        iVar7 = iVar7.wrapping_add(DAT_100af388);
                        uVar21 = uVar21.wrapping_add(DAT_100af390 as u32);
                        iVar8 = iVar8.wrapping_add(DAT_100af38c);
                        uVar23 = uVar23.wrapping_add(DAT_100af394 as u32);
                        state = S_A_BODY;
                    } else {
                        DAT_100af358 = DAT_100af358.wrapping_sub(1);
                        if DAT_100af358 == 0 {
                            state = S_L07;
                        } else {
                            iVar7 = iVar7.wrapping_add(DAT_100af388);
                            uVar21 = uVar21.wrapping_add(DAT_100af390 as u32);
                            state = S_A_TAIL;
                        }
                    }
                }
                S_A_TAIL => {
                    DAT_100af35c = DAT_100af35c.wrapping_sub(1);
                    if DAT_100af35c == 0 {
                        state = S_L0A5;
                    } else {
                        iVar8 = iVar8.wrapping_add(DAT_100af38c);
                        uVar23 = uVar23.wrapping_add(DAT_100af394 as u32);
                        state = S_A_BODY;
                    }
                }
                S_L07 => {
                    DAT_100af348 = DAT_100af350;
                    let mut p = DAT_100af350.wrapping_sub(6);
                    if (p as usize as i32) < (DAT_100af33c as usize as i32) {
                        p = DAT_100af340.wrapping_sub(6);
                    }
                    let dy = rd(p, 1).wrapping_sub(rd(DAT_100af350, 1));
                    DAT_100af358 = dy.wrapping_add((dy == 0) as u32) as i32;
                    let d = (DAT_100af358 as u32).wrapping_mul(0x10000) as i32;
                    if let Some(q) = fdiv(ldiv(rd(p, 2).wrapping_sub(rd(DAT_100af350, 2))), d) {
                        DAT_100af390 = q;
                    }
                    let d = (DAT_100af358 as u32).wrapping_mul(0x10000) as i32;
                    if let Some(q) = fdiv(hdiv(rd(p, 0).wrapping_sub(rd(DAT_100af350, 0))), d) {
                        DAT_100af388 = q;
                    }
                    uVar21 = rd(DAT_100af350, 2).wrapping_add(0x8000);
                    iVar7 = rd(DAT_100af350, 0)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000) as i32;
                    DAT_100af350 = p;
                    state = if DAT_100af338 != 0 { S_FFA6 } else { S_A_TAIL };
                }
                S_FFA6 => {
                    DAT_100af35c = DAT_100af35c.wrapping_sub(1);
                    if DAT_100af35c != 0 {
                        iVar8 = iVar8.wrapping_add(DAT_100af38c);
                        uVar23 = uVar23.wrapping_add(DAT_100af394 as u32);
                        state = S_CLIP_BODY;
                    } else {
                        state = S_L0A5;
                    }
                }
                S_CLIP_BODY => {
                    let mut iVar18 = iVar7;
                    let mut uVar9 = uVar21;
                    let mut uVar19 = uVar23;
                    let mut iVar15 = iVar8;
                    if iVar8 <= iVar7 {
                        iVar18 = iVar8;
                        uVar9 = uVar23;
                        uVar19 = uVar21;
                        iVar15 = iVar7;
                    }
                    iVar18 >>= 16;
                    if iVar18 <= DAT_100af318 {
                        iVar15 >>= 16;
                        if -1 < iVar15 {
                            let mut puVar24 =
                                iVar10.wrapping_add(iVar18) as u32 as usize as *mut u16;
                            let mut uVar12 = iVar15.wrapping_sub(iVar18).wrapping_add(1) as u32;
                            if uVar12 != 1 {
                                let iVar16 = iVar15.wrapping_sub(iVar18) >> 1;
                                let divd = ((iVar16 as u32).wrapping_add((iVar16 == 0) as u32))
                                    .wrapping_mul(0x10000)
                                    as i32;
                                if let Some(q) = fdiv(ldiv(uVar19.wrapping_sub(uVar9)), divd) {
                                    DAT_100af3b0 = q;
                                }
                            }
                            DAT_100af3d4 = 0u32.wrapping_sub(iVar18 as u32) as i32;
                            if DAT_100af3d4 == 0 || 0 < iVar18 {
                                DAT_100af3d4 = 0;
                            } else {
                                puVar24 = (puVar24 as usize as u32)
                                    .wrapping_add(DAT_100af3d4 as u32)
                                    as usize as *mut u16;
                                uVar12 = uVar12.wrapping_add(iVar18 as u32);
                                let t =
                                    ((DAT_100af3d4 as u32 & 0xffff_fffe).wrapping_shl(15)) as i32;
                                let l = (t as i64) * (DAT_100af3b0 as i64);
                                uVar9 = uVar9.wrapping_add(mulshift(l));
                            }
                            if iVar15.wrapping_sub(DAT_100af318) != 0 && DAT_100af318 <= iVar15 {
                                uVar12 =
                                    uVar12.wrapping_sub(iVar15.wrapping_sub(DAT_100af318) as u32);
                            }
                            DAT_100af334 = (puVar24 as usize as u32)
                                .wrapping_add(uVar12.wrapping_sub(1))
                                as usize as *mut i8;
                            let cVar3 = ((DAT_100af3b0 as u32) >> 16) as u8;
                            uVar19 = (DAT_100af3b0 as u32).wrapping_mul(0x10000);
                            let iVar16 = uVar9.wrapping_add(DAT_100af3cc as u32) as i32;
                            let s8 = uVar9.wrapping_add(DAT_100af3c8 as u32);
                            let cVar4 = (s8 >> 16) as u8;
                            let mut uVar6: u32 =
                                ((((iVar16 as u32) >> 16) as u16 as u32) << 8) | (cVar4 as u32);
                            uVar9 = s8.wrapping_mul(0x10000);
                            DAT_100af3a8 = iVar18;
                            DAT_100af3ac = iVar15;
                            let mut uVar14: u32 = 0;
                            let mut go_e97 = false;
                            let mut skip = false;
                            if (DAT_100af3d4 as u32 & 1) == 0 {
                                uVar14 = (iVar16 as u32).wrapping_mul(0x10000);
                                if (puVar24 as usize & 1) != 0 {
                                    (puVar24 as *mut u8).write(cVar4);
                                    uVar12 = uVar12.wrapping_sub(1);
                                    if uVar12 == 0 {
                                        skip = true;
                                    } else {
                                        puVar24 = (puVar24 as *mut u8).add(1) as *mut u16;
                                        go_e97 = true;
                                    }
                                }
                            } else {
                                go_e97 = true;
                            }
                            if !skip {
                                if go_e97 {
                                    uVar14 = uVar9.wrapping_add(uVar19);
                                    uVar6 = cat11(
                                        cVar4
                                            .wrapping_add(cVar3)
                                            .wrapping_add(carry(uVar9, uVar19)),
                                        ((iVar16 as u32) >> 16) as u8,
                                    ) as u32;
                                    uVar9 = (iVar16 as u32).wrapping_mul(0x10000);
                                }
                                if uVar12 != 1 {
                                    uVar6 = span_run(
                                        puVar24, uVar12, uVar19, cVar3, uVar9, uVar14, uVar6,
                                    );
                                    if (uVar12 & 1) != 0 {
                                        DAT_100af334.write(uVar6 as i8);
                                    }
                                } else {
                                    DAT_100af334.write(uVar6 as i8);
                                }
                            }
                        }
                    }
                    state = S_CLIP_END;
                }
                S_CLIP_END => {
                    let t = DAT_100af3cc;
                    DAT_100af3cc = DAT_100af3c8;
                    DAT_100af3c8 = t;
                    iVar10 = (iVar10 as u32).wrapping_add(DAT_100af324 as u32) as i32;
                    DAT_100af360 = DAT_100af360.wrapping_sub(1);
                    if DAT_100af360 < 0 {
                        return;
                    }
                    if DAT_100af360 != 0 {
                        state = S_FF92;
                    } else {
                        iVar7 = iVar7.wrapping_add(DAT_100af388);
                        uVar21 = uVar21.wrapping_add(DAT_100af390 as u32);
                        iVar8 = iVar8.wrapping_add(DAT_100af38c);
                        uVar23 = uVar23.wrapping_add(DAT_100af394 as u32);
                        state = S_CLIP_BODY;
                    }
                }
                S_FF92 => {
                    DAT_100af358 = DAT_100af358.wrapping_sub(1);
                    if DAT_100af358 == 0 {
                        state = S_L07;
                    } else {
                        iVar7 = iVar7.wrapping_add(DAT_100af388);
                        uVar21 = uVar21.wrapping_add(DAT_100af390 as u32);
                        state = S_FFA6;
                    }
                }
                _ => {
                    let mut p = DAT_100af354.wrapping_add(6);
                    if (DAT_100af340 as usize as i32)
                        <= (DAT_100af354.wrapping_add(6) as usize as i32)
                    {
                        p = DAT_100af33c;
                    }
                    let dy = rd(p, 1).wrapping_sub(rd(DAT_100af354, 1));
                    DAT_100af35c = dy.wrapping_add((dy == 0) as u32) as i32;
                    let d = (DAT_100af35c as u32).wrapping_mul(0x10000) as i32;
                    if let Some(q) = fdiv(ldiv(rd(p, 2).wrapping_sub(rd(DAT_100af354, 2))), d) {
                        DAT_100af394 = q;
                    }
                    let d = (DAT_100af35c as u32).wrapping_mul(0x10000) as i32;
                    if let Some(q) = fdiv(hdiv(rd(p, 0).wrapping_sub(rd(DAT_100af354, 0))), d) {
                        DAT_100af38c = q;
                    }
                    uVar23 = rd(DAT_100af354, 2).wrapping_add(0x8000);
                    iVar8 = rd(DAT_100af354, 0)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000) as i32;
                    DAT_100af34c = DAT_100af354;
                    DAT_100af354 = p;
                    state = S_JOINED;
                }
            }
        }
    }
}
