// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af324: i32;
    static mut DAT_100af318: i32;
    static mut DAT_100af31c: i32;
    static mut DAT_100af320: i32;
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
    static mut DAT_100af338: u32;
    static mut DAT_100af3c0: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[inline(always)]
fn rd(p: *const u32, i: usize) -> u32 {
    // SAFETY: the caller supplies a pointer into the game's vertex table.
    unsafe { p.wrapping_add(i).read_unaligned() }
}

#[inline(always)]
fn ri(p: *const i32, i: usize) -> i32 {
    // SAFETY: the caller supplies a pointer into the game's structures.
    unsafe { p.wrapping_add(i).read_unaligned() }
}

fn div(de: u32, dd: i32) -> i32 {
    let dq: i64;
    if dd == 0 {
        // SAFETY: raises the machine's divide exception as the original does.
        unsafe { RaiseException(0xC0000094, 0, 0, core::ptr::null()) };
        dq = 0;
    } else {
        let dn = ((de as u64) << 32) as i64;
        dq = dn.wrapping_div(dd as i64);
        if dq != (dq as i32) as i64 {
            // SAFETY: raises the machine's divide exception as the original does.
            unsafe { RaiseException(0xC0000094, 0, 0, core::ptr::null()) };
        }
    }
    dq as i32
}

fn fill(mut p: *mut u8, count: u32, v: u32) {
    let mut n = count & 3;
    while n != 0 {
        // SAFETY: writes into the caller's destination surface as the original does.
        unsafe { p.write(v as u8) };
        p = p.wrapping_add(1);
        n = n.wrapping_sub(1);
    }
    let mut n = count >> 2;
    while n != 0 {
        // SAFETY: writes into the caller's destination surface as the original does.
        unsafe { p.cast::<u32>().write_unaligned(v) };
        p = p.wrapping_add(4);
        n = n.wrapping_sub(1);
    }
}

#[derive(Clone, Copy, PartialEq)]
enum St {
    A,
    B,
    B2,
    C,
    D,
    E,
    R,
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003eb9c(param_1: *mut i32, param_2: i32, param_3: *mut u32) {
    // SAFETY: all globals and pointers belong to the game; accessed as the original does.
    unsafe {
        let pi1 = param_1.read_unaligned() as u32 as usize as *const i32;
        DAT_100af324 = ri(pi1, 1).wrapping_add(1);
        let mut i7 = ri(pi1, 1);
        if ri(param_1, 3) <= ri(pi1, 1) {
            i7 = ri(param_1, 3);
        }
        let mut u15: u32 = 0;
        if (ri(param_1, 1) as u32) < 0x80000000 {
            u15 = ri(param_1, 1) as u32;
        }
        let mut i9 = (i7 as u32).wrapping_sub(u15) as i32;
        if !((u15 as i32) <= i7) {
            return;
        }
        i7 = ri(pi1, 2);
        if ri(param_1, 4) <= ri(pi1, 2) {
            i7 = ri(param_1, 4);
        }
        let mut u17: u32 = 0;
        if (ri(param_1, 2) as u32) < 0x80000000 {
            u17 = ri(param_1, 2) as u32;
        }
        let mut i10 = (i7 as u32).wrapping_sub(u17) as i32;
        DAT_100af318 = i9;
        if !((u17 as i32) <= i7) {
            return;
        }
        DAT_100af320 = u17
            .wrapping_mul(DAT_100af324 as u32)
            .wrapping_add(u15)
            .wrapping_add(ri(pi1, 0) as u32) as i32;
        DAT_100af340 = param_3.wrapping_offset(param_2.wrapping_mul(6) as isize);
        DAT_100af33c = param_3;
        let u5 = (rd(param_3, 2).wrapping_add(0x8000) >> 16) as u8;
        let u6 = (((u5 as u32) << 8) | (u5 as u32)) as u16;
        DAT_100af3c0 = ((u6 as u32) << 16) | (u6 as u32);
        DAT_100af338 = 0;
        u15 = 0x7fff;
        u17 = 0xffff8000;
        let mut u11: u32 = 0xf;
        let mut p14 = param_3;
        loop {
            let u12 = ((rd(p14, 0) >> 31) << 1) | ((i9 as u32).wrapping_sub(rd(p14, 0)) >> 31);
            DAT_100af338 |= u12;
            let u2 = rd(p14, 1);
            if (u2 as i32) <= (u15 as i32) {
                u15 = u2;
                DAT_100af344 = p14;
            }
            if (u17 as i32) <= (u2 as i32) {
                u17 = u2;
            }
            u11 &= (((u12 << 1) | (rd(p14, 1) >> 31)) << 1)
                | ((i10 as u32).wrapping_sub(rd(p14, 1)) >> 31);
            p14 = p14.wrapping_add(6);
            if p14 == DAT_100af340 {
                break;
            }
        }
        DAT_100af31c = i10;
        if u11 != 0 {
            return;
        }
        DAT_100af350 = DAT_100af344;
        DAT_100af354 = DAT_100af344;
        p14 = DAT_100af350;
        DAT_100af364 = u15 as i32;
        if u17 == u15 {
            return;
        }
        loop {
            DAT_100af350 = p14;
            DAT_100af348 = DAT_100af350;
            p14 = DAT_100af350.wrapping_sub(6);
            if (p14 as usize as i32) < (param_3 as usize as i32) {
                p14 = DAT_100af340.wrapping_sub(6);
            }
            if (rd(DAT_100af350, 1) as i32) < 0 && (rd(p14, 1) as i32) < 1 {
                continue;
            }
            DAT_100af358 = rd(p14, 1).wrapping_sub(rd(DAT_100af350, 1)) as i32;
            if DAT_100af358 == 0 {
                continue;
            }
            break;
        }
        let mut de = ((rd(p14, 0).wrapping_sub(rd(DAT_100af350, 0))).wrapping_mul(0x10000) as i32
            >> 16) as u32;
        let mut dd = (DAT_100af358 as u32).wrapping_mul(0x10000) as i32;
        DAT_100af388 = div(de, dd);
        DAT_100af368 = rd(DAT_100af350, 0)
            .wrapping_mul(0x10000)
            .wrapping_add(0x8000) as i32;
        let mut p16 = DAT_100af354;
        loop {
            DAT_100af354 = p16;
            DAT_100af34c = DAT_100af354;
            p16 = DAT_100af354.wrapping_add(6);
            if (DAT_100af340 as usize as i32) <= (p16 as usize as i32) {
                p16 = param_3;
            }
            if (rd(DAT_100af354, 1) as i32) < 0 && (rd(p16, 1) as i32) < 1 {
                continue;
            }
            DAT_100af35c = rd(p16, 1).wrapping_sub(rd(DAT_100af354, 1)) as i32;
            if DAT_100af35c == 0 {
                continue;
            }
            break;
        }
        de = ((rd(p16, 0).wrapping_sub(rd(DAT_100af354, 0))).wrapping_mul(0x10000) as i32 >> 16)
            as u32;
        dd = (DAT_100af35c as u32).wrapping_mul(0x10000) as i32;
        DAT_100af38c = div(de, dd);
        DAT_100af36c = rd(DAT_100af354, 0)
            .wrapping_mul(0x10000)
            .wrapping_add(0x8000) as i32;
        _DAT_100af360 = (i10 as u32).wrapping_sub(u15) as i32;
        if u17.wrapping_sub(i10 as u32) == 0 || (u17 as i32) < i10 {
            _DAT_100af360 =
                (_DAT_100af360 as u32).wrapping_add(u17.wrapping_sub(i10 as u32)) as i32;
        }
        if 0x7fffffffu32 < u15 {
            _DAT_100af360 = (_DAT_100af360 as u32).wrapping_add(u15) as i32;
            DAT_100af364 = 0;
            DAT_100af358 = (DAT_100af358 as u32).wrapping_add(rd(DAT_100af350, 1)) as i32;
            let l3 = (DAT_100af388 as i64)
                .wrapping_mul(rd(DAT_100af350, 1).wrapping_mul(0xffff0000) as i32 as i64);
            DAT_100af368 = (DAT_100af368 as u32)
                .wrapping_add(((l3 as u32) >> 16) | (((((l3 as u64) >> 32) as i32) as u32) << 16))
                as i32;
            DAT_100af35c = (DAT_100af35c as u32).wrapping_add(rd(DAT_100af354, 1)) as i32;
            let l3 = (DAT_100af38c as i64)
                .wrapping_mul(rd(DAT_100af354, 1).wrapping_mul(0xffff0000) as i32 as i64);
            DAT_100af36c = (DAT_100af36c as u32)
                .wrapping_add(((l3 as u32) >> 16) | (((((l3 as u64) >> 32) as i32) as u32) << 16))
                as i32;
        }
        i10 = (DAT_100af320 as u32)
            .wrapping_add((DAT_100af364 as u32).wrapping_mul(DAT_100af324 as u32))
            as i32;
        i7 = DAT_100af368;
        i9 = DAT_100af36c;
        DAT_100af350 = p14;
        DAT_100af354 = p16;
        let mut st = if DAT_100af338 != 0 { St::B } else { St::A };
        loop {
            match st {
                St::A => {
                    let u4 = DAT_100af3c0;
                    let (mut i8, mut i13) = (i7, i9);
                    if i9 <= i7 {
                        i8 = i9;
                        i13 = i7;
                    }
                    let cnt = (((i13 >> 16).wrapping_sub(i8 >> 16)).wrapping_add(1)) as u32;
                    let p18 = (i10.wrapping_add(i8 >> 16)) as u32 as usize as *mut u8;
                    fill(p18, cnt, u4);
                    i10 = (i10 as u32).wrapping_add(DAT_100af324 as u32) as i32;
                    _DAT_100af360 = (_DAT_100af360 as u32).wrapping_sub(1) as i32;
                    if _DAT_100af360 < 0 {
                        return;
                    }
                    if _DAT_100af360 == 0 {
                        i7 = (i7 as u32).wrapping_add(DAT_100af388 as u32) as i32;
                        i9 = (i9 as u32).wrapping_add(DAT_100af38c as u32) as i32;
                    } else {
                        DAT_100af358 = DAT_100af358.wrapping_sub(1);
                        if DAT_100af358 == 0 {
                            st = St::C;
                        } else {
                            i7 = (i7 as u32).wrapping_add(DAT_100af388 as u32) as i32;
                            st = St::D;
                        }
                    }
                }
                St::D => {
                    DAT_100af35c = DAT_100af35c.wrapping_sub(1);
                    if DAT_100af35c == 0 {
                        st = St::R;
                    } else {
                        i9 = (i9 as u32).wrapping_add(DAT_100af38c as u32) as i32;
                        st = St::A;
                    }
                }
                St::R => {
                    DAT_100af34c = DAT_100af354;
                    p14 = DAT_100af354.wrapping_add(6);
                    if (DAT_100af340 as usize as i32)
                        <= (DAT_100af354.wrapping_add(6) as usize as i32)
                    {
                        p14 = DAT_100af33c;
                    }
                    let d = rd(p14, 1).wrapping_sub(rd(DAT_100af354, 1));
                    DAT_100af35c = d.wrapping_add((d == 0) as u32) as i32;
                    de = ((rd(p14, 0).wrapping_sub(rd(DAT_100af354, 0))).wrapping_mul(0x10000)
                        as i32
                        >> 16) as u32;
                    dd = (DAT_100af35c as u32).wrapping_mul(0x10000) as i32;
                    DAT_100af38c = div(de, dd);
                    i9 = rd(DAT_100af354, 0)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000) as i32;
                    DAT_100af354 = p14;
                    st = if DAT_100af338 == 0 { St::A } else { St::B };
                }
                St::B => {
                    let u4 = DAT_100af3c0;
                    let (mut i8, mut i13) = (i7, i9);
                    if i9 <= i7 {
                        i8 = i9;
                        i13 = i7;
                    }
                    i8 >>= 16;
                    i13 >>= 16;
                    if i8 <= DAT_100af318 && -1 < i13 {
                        let mut cnt = (i13.wrapping_sub(i8).wrapping_add(1)) as u32;
                        let mut p18 = (i10.wrapping_add(i8)) as u32 as usize as *mut u8;
                        if i8 < 0 {
                            p18 = (p18 as usize as u32).wrapping_sub(i8 as u32) as usize as *mut u8;
                            cnt = cnt.wrapping_add(i8 as u32);
                        }
                        if i13.wrapping_sub(DAT_100af318) != 0 && DAT_100af318 <= i13 {
                            cnt = cnt.wrapping_sub(i13.wrapping_sub(DAT_100af318) as u32);
                        }
                        fill(p18, cnt, u4);
                    }
                    i10 = (i10 as u32).wrapping_add(DAT_100af324 as u32) as i32;
                    _DAT_100af360 = (_DAT_100af360 as u32).wrapping_sub(1) as i32;
                    if _DAT_100af360 < 0 {
                        return;
                    }
                    if _DAT_100af360 != 0 {
                        st = St::B2;
                    } else {
                        i7 = (i7 as u32).wrapping_add(DAT_100af388 as u32) as i32;
                        i9 = (i9 as u32).wrapping_add(DAT_100af38c as u32) as i32;
                    }
                }
                St::B2 => {
                    DAT_100af358 = DAT_100af358.wrapping_sub(1);
                    if DAT_100af358 == 0 {
                        st = St::C;
                    } else {
                        i7 = (i7 as u32).wrapping_add(DAT_100af388 as u32) as i32;
                        st = St::E;
                    }
                }
                St::C => {
                    DAT_100af348 = DAT_100af350;
                    p14 = DAT_100af350.wrapping_sub(6);
                    if (p14 as usize as i32) < (DAT_100af33c as usize as i32) {
                        p14 = DAT_100af340.wrapping_sub(6);
                    }
                    let d = rd(p14, 1).wrapping_sub(rd(DAT_100af350, 1));
                    DAT_100af358 = d.wrapping_add((d == 0) as u32) as i32;
                    de = ((rd(p14, 0).wrapping_sub(rd(DAT_100af350, 0))).wrapping_mul(0x10000)
                        as i32
                        >> 16) as u32;
                    dd = (DAT_100af358 as u32).wrapping_mul(0x10000) as i32;
                    DAT_100af388 = div(de, dd);
                    i7 = rd(DAT_100af350, 0)
                        .wrapping_mul(0x10000)
                        .wrapping_add(0x8000) as i32;
                    DAT_100af350 = p14;
                    st = if DAT_100af338 == 0 { St::D } else { St::E };
                }
                St::E => {
                    DAT_100af35c = DAT_100af35c.wrapping_sub(1);
                    if DAT_100af35c == 0 {
                        st = St::R;
                    } else {
                        i9 = (i9 as u32).wrapping_add(DAT_100af38c as u32) as i32;
                        st = St::B;
                    }
                }
            }
        }
    }
}
