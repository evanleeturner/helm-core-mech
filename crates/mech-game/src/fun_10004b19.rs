// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100adf34: i32;
    static mut DAT_100b3788: i32;
    static mut DAT_100eef40: [i32; 1];
    static mut DAT_100aa1ac: i32;
    static mut DAT_100a40cc: i32;
    static mut DAT_100a4114: i32;
    static mut DAT_100b38fc: i32;
    static mut DAT_1016586d: u8;
    static mut DAT_101658a2: i16;
    static mut DAT_100a411c: [i8; 1];

    static mut _DAT_10165886: u16;
    static mut _DAT_10165884: u16;
    static mut _DAT_10165857: u16;
    static mut _DAT_10165859: u16;
    static mut _DAT_1016585b: u16;
    static mut _DAT_10165898: u16;
    static mut _DAT_10165894: u16;
    static mut _DAT_10165896: u16;
    static mut _DAT_10165872: u16;
    static mut _DAT_1016586e: u16;
    static mut _DAT_10165870: u16;
    static mut _DAT_1016589e: u16;
    static mut _DAT_1016589a: u16;
    static mut _DAT_1016589c: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100141b5();
    fn FUN_1004e27d();
    fn FUN_100082c4(a: i32, b: i32);
    fn FUN_1004d5e3(a: i32) -> i32;
    fn FUN_10026e24(a: *mut u32);
    fn FUN_1004d47f(a: i32) -> i32;
    fn FUN_10027fed(a: i32) -> i32;
    fn FUN_100285f0(a: i32, b: u32, c: u16);
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_1005dd5a(a: *mut i8);
    fn FUN_10008156(a: i32, b: i32);
    fn FUN_10050389(a: i32, b: i32);
    fn FUN_10030fd2(a: i32);
}

#[inline(always)]
unsafe fn m(p: *mut i32) -> *mut u8 {
    // SAFETY: caller guarantees p points at the game's record pointer.
    unsafe { p.read_unaligned() as u32 as *mut u8 }
}

#[inline(always)]
unsafe fn rd32(p: *mut u8, off: usize) -> i32 {
    // SAFETY: p+off is a game record field.
    unsafe { p.byte_add(off).cast::<i32>().read_unaligned() }
}

#[inline(always)]
unsafe fn rd16(p: *mut u8, off: usize) -> u16 {
    // SAFETY: p+off is a game record field.
    unsafe { p.byte_add(off).cast::<u16>().read_unaligned() }
}

#[inline(always)]
unsafe fn inc16(p: *mut u16) {
    // SAFETY: p is the address of a game global.
    unsafe { p.write_unaligned(p.read_unaligned().wrapping_add(1)) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10004b19(param_1: i32, param_2: *mut i32) {
    // SAFETY: all pointers and globals belong to the game; accessed as the original does.
    unsafe {
        let cond1 = rd32(m(param_2), 4) != DAT_100adf38
            || ((DAT_100adf34 as u32 as *const u8).byte_add(1).read() == 0)
            || param_2.add(0xf5).read_unaligned() == 7;
        if !(cond1 && (rd16(m(param_2), 0x14) & 2) == 0 && (rd16(m(param_2), 0x14) & 4) == 0) {
            return;
        }
        if DAT_100b3788 != 0 {
            let b = DAT_100b3788 as u32 as *mut u8;
            if (rd16(b, 2) & 0x100) != 0 && (rd32(m(param_2), 4) as u32) == (rd16(b, 0x14) as u32) {
                DAT_100b3788 = 0;
            }
        }
        m(param_2)
            .byte_add(0x13c)
            .cast::<i32>()
            .write_unaligned(param_1);
        let idx = DAT_100adf38 as isize;
        let ent = (&raw mut DAT_100eef40)
            .cast::<i32>()
            .offset(idx)
            .read_unaligned();
        if rd32(ent as u32 as *mut u8, 8) == rd32(m(param_2), 8) {
            if param_2.add(0xf5).read_unaligned() == 5 {
                inc16(&raw mut _DAT_10165886);
            } else {
                inc16(&raw mut _DAT_10165884);
            }
            if rd32(m(param_2), 4) == DAT_100adf38
                && -1 < param_1
                && param_1 < 8
                && DAT_100aa1ac != 0
            {
                let ps = (&raw mut DAT_101658a2)
                    .cast::<u8>()
                    .byte_add(
                        (DAT_100adf38 as u32)
                            .wrapping_mul(2)
                            .wrapping_add((param_1 as u32).wrapping_mul(0x10))
                            as usize,
                    )
                    .cast::<i16>();
                ps.write_unaligned(ps.read_unaligned().wrapping_add(1));
                FUN_100141b5();
            }
        }
        FUN_1004e27d();
        if param_2.add(0xf5).read_unaligned() == 5 {
            if DAT_100a40cc == 0 {
                if rd32(m(param_2), 4) == DAT_100adf38 {
                    DAT_1016586d = 2;
                    FUN_100082c4(0xc, -1);
                }
            } else {
                if rd32(m(param_2), 4) == DAT_100adf38 {
                    DAT_1016586d = 4;
                    FUN_100082c4(0x20, -1);
                }
                param_2.add(0xf5).write_unaligned(4);
            }
        } else if rd32(m(param_2), 4) == DAT_100adf38 {
            DAT_1016586d = 4;
        }
        if DAT_100adf38 == param_1 {
            if rd32(m(param_2), 0) == 1 {
                let r = FUN_1004d5e3(rd32(m(param_2), 4));
                if r == 0 {
                    DAT_100a4114 = DAT_100a4114.wrapping_sub(1);
                    inc16(&raw mut _DAT_1016585b);
                } else if r == 1 {
                    DAT_100a4114 = DAT_100a4114.wrapping_add(1);
                    inc16(&raw mut _DAT_10165857);
                } else if r == 2 {
                    DAT_100a4114 = DAT_100a4114.wrapping_add(1);
                    inc16(&raw mut _DAT_10165859);
                }
            } else {
                let r = FUN_1004d5e3(rd32(m(param_2), 4));
                if r == 0 {
                    inc16(&raw mut _DAT_10165898);
                } else if r == 1 {
                    inc16(&raw mut _DAT_10165894);
                } else if r == 2 {
                    inc16(&raw mut _DAT_10165896);
                }
            }
        }
        if (rd16(m(param_2), 0x14) & 0x1400) != 0 {
            if rd32(m(param_2), 0) == 1 {
                let r = FUN_1004d5e3(rd32(m(param_2), 4));
                if r == 0 {
                    inc16(&raw mut _DAT_10165872);
                } else if r == 1 {
                    inc16(&raw mut _DAT_1016586e);
                } else if r == 2 {
                    inc16(&raw mut _DAT_10165870);
                }
            } else {
                let r = FUN_1004d5e3(rd32(m(param_2), 4));
                if r == 0 {
                    inc16(&raw mut _DAT_1016589e);
                } else if r == 1 {
                    inc16(&raw mut _DAT_1016589a);
                } else if r == 2 {
                    inc16(&raw mut _DAT_1016589c);
                }
            }
        }
        let r = FUN_1004d5e3(rd32(m(param_2), 4));
        if r == 1 {
            match rd32(m(param_2), 0) {
                1 => FUN_100082c4(0x15, -1),
                2 => FUN_100082c4(0x17, -1),
                3 => FUN_100082c4(0x19, -1),
                4 => FUN_100082c4(0x16, -1),
                5 => FUN_100082c4(0x17, -1),
                8 => FUN_100082c4(0x17, -1),
                _ => {}
            }
        }
        let f = rd16(m(param_2), 0x14) | 6;
        m(param_2).byte_add(0x14).cast::<u16>().write_unaligned(f);
        param_2.add(0xef).write_unaligned(0);
        if DAT_100aa1ac == 0 {
            if param_2.add(0xf5).read_unaligned() != 5 || rd32(m(param_2), 4) != DAT_100adf38 {
                param_2.add(0xf5).write_unaligned(4);
            }
            FUN_10026e24(m(param_2).cast::<u32>());
            let r = FUN_1004d47f(rd32(m(param_2), 8));
            if r == rd32(m(param_2), 4) {
                let local_8 = FUN_10027fed(rd32(m(param_2), 8));
                FUN_100285f0(
                    rd32(m(param_2), 8),
                    rd32(m(param_2), 4) as u32,
                    local_8 as u16,
                );
                let mut local_58 = [0i8; 80];
                sprintf(
                    local_58.as_mut_ptr(),
                    (&raw const DAT_100a411c).cast::<i8>(),
                    DAT_100b38fc,
                    rd32(m(param_2), 8),
                    local_8,
                );
                FUN_1005dd5a(local_58.as_mut_ptr());
            }
        } else {
            param_2.add(0xf5).write_unaligned(4);
        }
        if DAT_100aa1ac == 0 && rd32(m(param_2), 4) != DAT_100adf38 {
            let r = FUN_1004d47f(rd32(m(param_2), 8));
            if r == DAT_100adf38 {
                FUN_10008156(6, rd32(m(param_2), 0xc));
            }
        }
        FUN_10050389(rd32(m(param_2), 4), 1);
        FUN_10030fd2(rd32(m(param_2), 0x140));
        m(param_2).byte_add(0x140).cast::<i32>().write_unaligned(-1);
    }
}
