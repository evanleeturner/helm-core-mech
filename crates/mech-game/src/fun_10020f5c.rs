// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100b0870: i32;
    static mut DAT_100b0874: u32;
    static mut DAT_100aa1ac: i32;
    static mut DAT_100b38fc: i32;
    static mut DAT_100b0a00: i32;
    static mut DAT_100a40a8: i32;
    static mut DAT_100b3904: i32;
    static mut DAT_100ab2f8: i32;
    static mut DAT_100ab320: i32;
    static mut DAT_100ba974: i32;
    static mut DAT_100ab304: i32;
    static mut _DAT_100ab32c: i32;
    static mut DAT_100adf34: *mut i8;
    static mut DAT_100ab328: i32;
    static mut DAT_100ab324: i32;
    static mut DAT_100ab094: i32;
    static mut PTR_DAT_100aaf68: *mut i8;
    static mut DAT_100eef40: [i32; 1];
    static mut s_MASC_malfunction__100ab338: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10061d8b(p: *mut i32);
    fn FUN_100635a2();
    fn FUN_10005624(a: i32, p: *mut i32, b: i32, c: i32, d: i32);
    fn FUN_10026f0b(p: *mut i32);
    fn FUN_100082c4(a: i32, b: i32);
    fn FUN_10083b74(a: i32, b: i32, c: i32);
    fn FUN_1008276d(a: i32, b: i32, c: i32);
    fn FUN_10083c70();
    fn FUN_10083c0c(a: i32);
    fn FUN_10083bae(a: i32);
    fn FUN_10083bdd(a: i32);
    fn FUN_100838c2();
    fn FUN_100832d4(a: i32);
    fn FUN_100639ad(p: *mut i32);
    fn FUN_10004190(p: *mut i32);
    fn FUN_10003d60(a: i32, b: i32) -> i32;
    fn FUN_10070800(p: *mut i32);
    fn FUN_1006b47b(p: *mut i32);
    fn FUN_10006b9a(a: i32) -> i32;
    fn FUN_10015b30(a: i32, b: i32) -> i32;
    fn FUN_10051f43(a: i32) -> i32;
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_1005be24(s: *mut i8, a: i32, b: i32, c: i32);
    fn FUN_10003742(a: i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_1002977f(a: i32, b: i32, c: i32) -> i32;
    fn FUN_10011e60(a: i32, b: i32, c: i32) -> i32;
    fn FUN_1007078e(a: i32);
    fn FUN_10061cf0(a: i32);
    fn FUN_1000465a(p: *mut i32);
    fn FUN_100069d3(p: *mut i32, a: i32);
    fn FUN_1004d5e3(a: i32) -> i32;
    fn FUN_100037f0(a: i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_1006b2cc(p: *mut i32);
    fn FUN_1004fed5();
}

unsafe fn g(p: *mut i32, n: usize) -> i32 {
    // SAFETY: p points at the game's entity wrapper; n is a word index inside it.
    unsafe { p.add(n).read_unaligned() }
}

unsafe fn gs(p: *mut i32, n: usize, v: i32) {
    // SAFETY: p points at the game's entity wrapper; n is a word index inside it.
    unsafe { p.add(n).write_unaligned(v) }
}

unsafe fn ep(p: *mut i32) -> *mut u8 {
    // SAFETY: the first word of the wrapper is the entity address.
    unsafe { (p.read_unaligned() as u32) as usize as *mut u8 }
}

unsafe fn e_i(p: *mut i32, off: usize) -> i32 {
    // SAFETY: reads a word inside the entity.
    unsafe { ep(p).byte_add(off).cast::<i32>().read_unaligned() }
}

unsafe fn e_us(p: *mut i32, off: usize) -> u16 {
    // SAFETY: reads a half-word inside the entity.
    unsafe { ep(p).byte_add(off).cast::<u16>().read_unaligned() }
}

unsafe fn e_sus(p: *mut i32, off: usize, v: u16) {
    // SAFETY: writes a half-word inside the entity.
    unsafe { ep(p).byte_add(off).cast::<u16>().write_unaligned(v) }
}

unsafe fn e_uc(p: *mut i32, off: usize) -> u8 {
    // SAFETY: reads a byte inside the entity.
    unsafe { ep(p).byte_add(off).read() }
}

unsafe fn e_u(p: *mut i32, off: usize) -> u32 {
    // SAFETY: reads a word inside the entity.
    unsafe { ep(p).byte_add(off).cast::<u32>().read_unaligned() }
}

unsafe fn e_su(p: *mut i32, off: usize, v: u32) {
    // SAFETY: writes a word inside the entity.
    unsafe { ep(p).byte_add(off).cast::<u32>().write_unaligned(v) }
}

unsafe fn sp(p: *mut i32) -> *mut u8 {
    // SAFETY: entity offset 0x4c holds the address of the sub-struct.
    unsafe { (e_i(p, 0x4c) as u32) as usize as *mut u8 }
}

unsafe fn s_c(p: *mut i32, off: usize) -> i8 {
    // SAFETY: reads a byte inside the sub-struct.
    unsafe { sp(p).byte_add(off).cast::<i8>().read() }
}

unsafe fn s_sc(p: *mut i32, off: usize, v: i8) {
    // SAFETY: writes a byte inside the sub-struct.
    unsafe { sp(p).byte_add(off).cast::<i8>().write(v) }
}

unsafe fn s_i(p: *mut i32, off: usize) -> i32 {
    // SAFETY: reads a word inside the sub-struct.
    unsafe { sp(p).byte_add(off).cast::<i32>().read_unaligned() }
}

unsafe fn s_si(p: *mut i32, off: usize, v: i32) {
    // SAFETY: writes a word inside the sub-struct.
    unsafe { sp(p).byte_add(off).cast::<i32>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10020f5c(param_1: *mut i32) {
    // SAFETY: param_1 is the game's entity wrapper; globals, callees and the
    // pointers derived from the entity are the game's own memory, accessed
    // where and in the order the original does.
    unsafe {
        let mut i_var1: i32;
        let mut local_6c: [i8; 40] = [0; 40];
        let mut local_44: i32;
        let mut local_40: i32;
        let mut local_3c: i32;
        let mut local_38: i32;
        let local_34: u32;
        let mut local_30: i32 = 0;
        let mut local_2c: u32;
        let mut local_28: i32;
        let local_24: *mut i32 = param_1;
        let mut local_20: u32;
        let mut local_1c: i32 = 0;
        let mut local_18: i32 = 0;
        let mut local_14: i32 = 0;
        let mut local_10: i32 = 0;
        let local_c: u32;
        let mut local_8: i32 = 0;

        if (param_1.add(0x110).cast::<u16>().read_unaligned() & 0x200) == 0 {
            local_c = (e_i(local_24, 4) == DAT_100adf38) as u32;
            local_34 = ((e_us(local_24, 0x14) & 1) == 0) as u32;
            if g(param_1, 0xf5) == 2
                && {
                    FUN_10061d8b(param_1);
                    s_c(local_24, 0x2b) != 0
                }
                && local_c != 0
                && s_c(local_24, 0x27) != 0
            {
                FUN_100635a2();
            }
            if DAT_100b0870 > -1 && (DAT_100b0874 as i32) > -1 && e_i(local_24, 4) == DAT_100adf38 {
                FUN_10005624(-1, local_24, DAT_100b0874 as i32, DAT_100b0870, 0);
                DAT_100b0870 = -1;
            }
            FUN_10026f0b(local_24.read_unaligned() as u32 as usize as *mut i32);
            if (DAT_100aa1ac == 0 || local_c != 0) && g(local_24, 0xf5) == 2 {
                if s_c(local_24, 0x44) != 0 {
                    gs(local_24, 0xef, DAT_100b38fc.wrapping_add(0x16a));
                    gs(local_24, 0xf5, 7);
                    if local_c != 0 {
                        FUN_100082c4(4, -1);
                    }
                    s_sc(local_24, 0x44, 0);
                }
                if s_c(local_24, 0x31) != 0 {
                    FUN_10083b74(local_24.read_unaligned(), 1, 0);
                    s_sc(local_24, 0x31, 0);
                }
                if s_c(local_24, 0x32) != 0 {
                    FUN_10083b74(local_24.read_unaligned(), -1, 0);
                    s_sc(local_24, 0x32, 0);
                }
                if s_c(local_24, 0x33) != 0 {
                    FUN_10083b74(local_24.read_unaligned(), 0, 0);
                    s_sc(local_24, 0x33, 0);
                }
                if s_c(local_24, 0x34) != 0 {
                    FUN_1008276d(local_24.read_unaligned(), 1, 8);
                    s_sc(local_24, 0x34, 0);
                }
                if s_c(local_24, 0x35) != 0 {
                    FUN_1008276d(local_24.read_unaligned(), -1, 8);
                    s_sc(local_24, 0x35, 0);
                }
                if s_c(local_24, 0x36) != 0 {
                    FUN_1008276d(local_24.read_unaligned(), 0, 8);
                    s_sc(local_24, 0x35, 0);
                }
                if s_c(local_24, 0x39) != 0 {
                    FUN_10083c70();
                    s_sc(local_24, 0x39, 0);
                }
                if s_c(local_24, 0x38) != 0 {
                    FUN_10083c0c(1);
                    s_sc(local_24, 0x38, 0);
                }
                if s_c(local_24, 0x3a) != 0 {
                    s_sc(local_24, 0x3a, 0);
                }
                if s_c(local_24, 0x3c) != 0 {
                    FUN_1008276d(local_24.read_unaligned(), 1, 0x10008);
                    s_sc(local_24, 0x3c, 0);
                }
                if s_c(local_24, 0x3d) != 0 {
                    FUN_10083bae(1);
                    s_sc(local_24, 0x3d, 0);
                }
                if s_c(local_24, 0x3e) != 0 {
                    FUN_10083bae(-1);
                    s_sc(local_24, 0x3e, 0);
                }
                if s_c(local_24, 0x3f) != 0 {
                    FUN_10083bae(0);
                    s_sc(local_24, 0x3e, 0);
                }
                if s_c(local_24, 0x40) != 0 {
                    FUN_10083bdd(1);
                    s_sc(local_24, 0x40, 0);
                }
                if s_c(local_24, 0x41) != 0 {
                    FUN_10083bdd(-1);
                    s_sc(local_24, 0x41, 0);
                }
                if s_c(local_24, 0x42) != 0 {
                    FUN_10083bdd(0);
                    s_sc(local_24, 0x41, 0);
                }
                if s_c(local_24, 0x37) != 0 {
                    FUN_100838c2();
                    s_sc(local_24, 0x37, 0);
                }
                if (e_uc(local_24, 0xdd) & 0x10) == 0 {
                    FUN_100832d4(local_24.read_unaligned());
                }
                FUN_100639ad(local_24);
                if (s_i(local_24, 0x10) != 0 || DAT_100b0a00 != 0) && g(local_24, 0xfc) != 0 {
                    s_sc(local_24, 0x43, 1);
                }
                if s_c(local_24, 0x43) != 0 {
                    if g(local_24, 0xfc) == 0 {
                        gs(local_24, 0xfc, 1);
                        if e_i(local_24, 4) == DAT_100adf38 {
                            FUN_100082c4(0x1d, 1);
                        }
                    } else {
                        gs(local_24, 0xfc, 0);
                        if e_i(local_24, 4) == DAT_100adf38 {
                            FUN_100082c4(0x1d, 2);
                        }
                    }
                }
                s_sc(local_24, 0x43, 0);
                if e_i(local_24, 0x10) == 0 && g(local_24, 0xfc) == 0 {
                    let v = s_i(local_24, 0x10);
                    s_si(local_24, 0xc, v);
                }
                if e_i(local_24, 0x10) == 0 {
                    FUN_10004190(local_24);
                }
                if s_c(local_24, 0x46) == 0 {
                    if s_c(local_24, 0x47) == 0 {
                        i_var1 = FUN_10003d60(s_i(local_24, 8), g(local_24, 0xf9));
                        gs(local_24, 0x12, i_var1.wrapping_add(0x400));
                    } else {
                        s_sc(local_24, 0x30, 1);
                        gs(local_24, 0x12, 0x800);
                        gs(local_24, 0x13, 0x800);
                        s_sc(local_24, 0x47, 0);
                    }
                } else {
                    s_sc(local_24, 0x30, 0);
                    gs(local_24, 0x12, 0x800);
                    gs(local_24, 0x13, 0x800);
                    s_sc(local_24, 0x46, 0);
                }
                if e_i(local_24, 0x10) != 2
                    && g(local_24, 0x12) == 0x400
                    && (s_i(local_24, 0xc) != 0 || DAT_100b0a00 != 0)
                {
                    gs(local_24, 0x12, 0x480);
                }
            }
            FUN_10070800(local_24.read_unaligned() as u32 as usize as *mut i32);
            if g(local_24, 0xfd) == -2 && s_c(local_24, 0x45) != 0 {
                gs(local_24, 0xfd, 0x712);
                gs(local_24, 0xfe, 3);
                gs(local_24, 0x108, DAT_100a40a8.wrapping_mul(3));
            }
            if -1 < g(local_24, 0xfd) {
                if s_c(local_24, 0x1d) == 0 || g(local_24, 0xf5) != 2 {
                    if g(local_24, 0xfd) < 0x712 {
                        let d = DAT_100b3904;
                        let t = (((d as u32).wrapping_add(((d >> 0x1f) as u32) & 3)) as i32) >> 2;
                        gs(local_24, 0xfd, g(local_24, 0xfd).wrapping_add(t));
                    } else {
                        gs(local_24, 0xfd, 0x712);
                    }
                } else {
                    local_14 = 1;
                    if DAT_100ab2f8 == 0 || local_c == 0 {
                        gs(local_24, 0xfd, g(local_24, 0xfd).wrapping_sub(DAT_100b3904));
                    }
                    if g(local_24, 0xfd) < 1 {
                        gs(local_24, 0xfd, 0);
                    }
                }
            }
            if local_14 == 0 {
                if (e_uc(local_24, 0x80) & 4) == 0 && local_34 != 0 {
                    local_1c = g(local_24, 0xb) / 10000;
                    if local_1c == 0 && (local_c == 0 || g(local_24, 0xf9) < 1) {
                        gs(local_24, 0xe, 0);
                    } else {
                        if 0x50 < local_1c {
                            local_1c = 0x50;
                        }
                        let first = local_24.read_unaligned() as u32 as usize as *const i32;
                        if first.read_unaligned() == 8 || first.read_unaligned() == 2 {
                            i_var1 = s_i(local_24, 0xc);
                            local_18 = ((((i_var1 as u32)
                                .wrapping_add(((i_var1 >> 0x1f) as u32) & 0x3ff))
                                as i32)
                                >> 10)
                                .wrapping_mul(0xb4);
                        } else {
                            i_var1 = s_i(local_24, 0xc);
                            local_18 = ((((i_var1 as u32)
                                .wrapping_add(((i_var1 >> 0x1f) as u32) & 0x3ff))
                                as i32)
                                >> 10)
                                .wrapping_mul(0x5a);
                        }
                        local_10 = FUN_10006b9a(((local_1c as u32) << 0x10) as i32);
                        local_30 = FUN_10015b30(local_18, local_10);
                        gs(local_24, 0xe, local_30);
                    }
                } else if g(local_24, 0xfc) == 0 {
                    gs(local_24, 0xe, 0);
                }
            } else {
                local_18 = 0;
                if 0 < g(local_24, 0xfd) {
                    FUN_1006b47b(local_24);
                    gs(
                        local_24,
                        0xf1,
                        g(local_24, 0xf1).wrapping_add(
                            g(local_24, 0xfe)
                                .wrapping_mul(DAT_100b3904)
                                .wrapping_mul(0x180),
                        ),
                    );
                    e_su(local_24, 0x80, e_u(local_24, 0x80) & 0xfffffffb);
                    e_su(local_24, 0x80, e_u(local_24, 0x80) & 0xfffffffe);
                    i_var1 = s_i(local_24, 0xc);
                    local_18 = ((((i_var1 as u32).wrapping_add(((i_var1 >> 0x1f) as u32) & 0x3ff))
                        as i32)
                        >> 10)
                        .wrapping_mul(0x5a);
                }
                gs(local_24, 0xe, local_18.wrapping_mul(2));
            }
            if local_c != 0 && DAT_100b0a00 != 0 {
                local_40 = e_i(local_24, 0x60);
                local_3c = local_40 >> 0x10;
                if local_3c < 0xb5 {
                    if local_3c < -0xb4 {
                        local_40 = local_40.wrapping_add(0x1680000);
                        local_3c = local_40 >> 0x10;
                    }
                } else {
                    local_40 = 0u32.wrapping_sub(0x1680000u32.wrapping_sub(local_40 as u32)) as i32;
                    local_3c = local_40 >> 0x10;
                }
                local_38 = DAT_100ab320 >> 0x10;
                if local_38 < 0xb5 {
                    if local_38 < -0xb4 {
                        DAT_100ab320 = DAT_100ab320.wrapping_add(0x1680000);
                        local_38 = DAT_100ab320 >> 0x10;
                    }
                } else {
                    DAT_100ab320 =
                        0u32.wrapping_sub(0x1680000u32.wrapping_sub(DAT_100ab320 as u32)) as i32;
                    local_38 = DAT_100ab320 >> 0x10;
                }
                if local_3c < 0x5b || -0x5b < local_38 {
                    if 0x5a < local_38 && local_3c < -0x5a {
                        local_40 = local_40.wrapping_add(0x1680000);
                    }
                } else {
                    local_40 = 0u32.wrapping_sub(0x1680000u32.wrapping_sub(local_40 as u32)) as i32;
                }
                local_44 = local_40.wrapping_sub(DAT_100ab320);
                if g(local_24, 3) < -0x3ffff || 0x3ffff < g(local_24, 3) {
                    if g(local_24, 3) < 1 {
                        gs(local_24, 0xe, -0x370000);
                        if local_44 != 0 {
                            gs(local_24, 2, g(local_24, 3).wrapping_sub(local_44));
                            gs(local_24, 3, g(local_24, 2));
                        }
                    } else {
                        gs(local_24, 0xe, 0x370000);
                        if local_44 != 0 {
                            gs(local_24, 2, g(local_24, 3).wrapping_sub(local_44));
                            gs(local_24, 3, g(local_24, 2));
                        }
                    }
                } else {
                    gs(local_24, 0xe, 0);
                    gs(local_24, 0xf, g(local_24, 0xe));
                    gs(local_24, 2, 0);
                    gs(local_24, 3, g(local_24, 2));
                    DAT_100ba974 = 1;
                    s_si(local_24, 4, 0);
                    DAT_100b0a00 = 0;
                }
            }
            if local_c != 0 {
                DAT_100ab320 = e_i(local_24, 0x60);
            }
            if local_34 != 0 {
                local_8 = g(local_24, 0x13).wrapping_sub(0x400);
                if local_8 < 0x20 {
                    local_8 = 0;
                }
                gs(local_24, 10, g(local_24, 0xee).wrapping_mul(local_8));
                if g(local_24, 10) != 0 && e_i(local_24, 0x84) == 2 {
                    gs(local_24, 10, g(local_24, 10) >> 1);
                    gs(local_24, 10, 0i32.wrapping_sub(g(local_24, 10)));
                }
                if local_14 == 0 && (e_uc(local_24, 0x80) & 2) != 0 {
                    if DAT_100ab304 != 0
                        && local_c != 0
                        && 0xb5 < DAT_100b38fc.wrapping_sub(_DAT_100ab32c)
                    {
                        _DAT_100ab32c = DAT_100b38fc;
                        i_var1 = FUN_10051f43(0x3c);
                        if i_var1 == 0xc
                            && DAT_100adf34.byte_add(1).read() == 0
                            && g(local_24, 0xf5) == 2
                        {
                            sprintf(
                                local_6c.as_mut_ptr(),
                                (&raw const s_MASC_malfunction__100ab338).cast::<i8>(),
                            );
                            FUN_1005be24(local_6c.as_mut_ptr(), 1, 0x16a, 0x32);
                            FUN_10003742(0xda, 100, 0x40, 5, 0x50);
                            gs(
                                local_24,
                                0xf3,
                                g(local_24, 0xf3).wrapping_add(g(local_24, 0xf3) >> 2),
                            );
                            DAT_100ab304 = 0;
                        }
                    }
                    gs(local_24, 0xc, 0x5a);
                    if (DAT_100ab304 != 0 || DAT_100ab328 != 0) && e_i(local_24, 4) == DAT_100adf38
                    {
                        gs(
                            local_24,
                            10,
                            g(local_24, 10).wrapping_add(g(local_24, 10) >> 1),
                        );
                    }
                }
                if local_c != 0 && DAT_100ab328 != 0 && (e_uc(local_24, 0xdd) & 0x10) == 0 {
                    i_var1 = FUN_1002977f(local_24.read_unaligned(), e_i(local_24, 0xd8), 0);
                    sp(local_24)
                        .cast::<i32>()
                        .write_unaligned((i_var1 as u32).wrapping_mul(0xfffff100) as i32);
                }
                if DAT_100b0a00 == 0 && (local_c != 0 || DAT_100aa1ac == 0) {
                    gs(local_24, 2, s_i(local_24, 4));
                }
                if g(local_24, 0x105) < g(local_24, 2) {
                    gs(local_24, 2, g(local_24, 0x105));
                    s_si(local_24, 4, g(local_24, 0x105));
                    s_sc(local_24, 0x1b, 1);
                } else if 0i32.wrapping_sub(g(local_24, 2)) != g(local_24, 0x105)
                    && g(local_24, 2) <= 0i32.wrapping_sub(g(local_24, 0x105))
                {
                    gs(local_24, 2, 0i32.wrapping_sub(g(local_24, 0x105)));
                    s_si(local_24, 4, 0i32.wrapping_sub(g(local_24, 0x105)));
                    s_sc(local_24, 0x1b, 1);
                }
                if local_c != 0 || DAT_100aa1ac == 0 {
                    gs(local_24, 6, sp(local_24).cast::<i32>().read_unaligned());
                }
                i_var1 = FUN_10011e60(
                    g(local_24, 0x13).wrapping_sub(0x400),
                    g(local_24, 0xf4),
                    0x2800,
                );
                gs(local_24, 0xf1, g(local_24, 0xf1).wrapping_add(i_var1));
            }
            if g(local_24, 0xf5) != 2 {
                FUN_1007078e(local_24.read_unaligned());
                gs(local_24, 0x12, 0x400);
                gs(local_24, 10, 0);
                gs(local_24, 2, 0);
                gs(local_24, 0xe, 0);
                gs(local_24, 6, 0);
                s_si(local_24, 8, 0);
                s_sc(local_24, 0x24, 1);
                s_sc(local_24, 0x1d, 0);
                s_sc(local_24, 0x1e, 0);
                s_sc(local_24, 0x1f, 0);
                s_sc(local_24, 0x20, 0);
                s_sc(local_24, 0x21, 0);
                s_sc(local_24, 0x25, 0);
                s_sc(local_24, 0x26, 0);
                s_sc(local_24, 0x2e, 0);
                s_sc(local_24, 0x2f, 0);
                s_sc(local_24, 0x30, 0);
                s_sc(local_24, 0x31, 0);
                s_sc(local_24, 0x43, 0);
                s_sc(local_24, 0x46, 0);
                s_sc(local_24, 0x47, 0);
                if e_i(local_24, 4) == DAT_100adf38 {
                    FUN_10061cf0(local_24 as usize as i32);
                }
            }
            FUN_1000465a(local_24);
            match g(local_24, 0xf5) {
                0 => {
                    i_var1 = FUN_10051f43(0x16a);
                    gs(
                        local_24,
                        0xef,
                        DAT_100b38fc.wrapping_add(i_var1).wrapping_add(0x43e),
                    );
                    gs(local_24, 0xf5, 1);
                    e_sus(local_24, 0x14, e_us(local_24, 0x14) & 0xdfff);
                    local_20 = (e_i(local_24, 0x50) as u32)
                        .wrapping_sub(PTR_DAT_100aaf68.cast::<i32>().read_unaligned() as u32);
                    local_28 = (e_i(local_24, 0x54) as u32).wrapping_sub(
                        PTR_DAT_100aaf68.byte_add(4).cast::<i32>().read_unaligned() as u32,
                    ) as i32;
                    local_2c = (e_i(local_24, 0x58) as u32).wrapping_sub(
                        PTR_DAT_100aaf68.byte_add(8).cast::<i32>().read_unaligned() as u32,
                    );
                    i_var1 = FUN_1004d5e3(e_i(local_24, 4));
                    if i_var1 == 1 {
                        let ent = (&raw mut DAT_100eef40)
                            .cast::<i32>()
                            .offset(DAT_100adf38 as isize)
                            .read();
                        let w = (ent as u32 as usize as *const u8)
                            .byte_add(0x14)
                            .cast::<u16>()
                            .read_unaligned();
                        if (w & 0x2000) != 0 {
                            FUN_100037f0(
                                local_20 as i32,
                                local_28,
                                local_2c as i32,
                                0x11b,
                                DAT_100ab094,
                            );
                        }
                    } else {
                        FUN_100037f0(
                            local_20 as i32,
                            local_28,
                            local_2c as i32,
                            0x10c,
                            (DAT_100ab094 == 0) as i32,
                        );
                        if local_c != 0 {
                            FUN_100082c4(0x26, -1);
                        }
                    }
                }
                1 => {
                    if g(local_24, 0xef) < DAT_100b38fc {
                        gs(local_24, 0xf5, 2);
                        e_sus(local_24, 0x14, e_us(local_24, 0x14) | 0x2000);
                        if DAT_100ab324 == 0 {
                            DAT_100ab324 = 1;
                        }
                    }
                }
                2 => {
                    if (e_us(local_24, 0x14) & 0x10) != 0 {
                        e_sus(local_24, 0x14, e_us(local_24, 0x14) & 0xffef);
                        e_sus(local_24, 0x14, e_us(local_24, 0x14) | 0x2000);
                        i_var1 = FUN_1004d5e3(e_i(local_24, 4));
                        if i_var1 == 1 {
                            let ent = (&raw mut DAT_100eef40)
                                .cast::<i32>()
                                .offset(DAT_100adf38 as isize)
                                .read();
                            let w = (ent as u32 as usize as *const u8)
                                .byte_add(0x14)
                                .cast::<u16>()
                                .read_unaligned();
                            if (w & 0x2000) != 0 {
                                FUN_100082c4(0x1a, -1);
                            }
                        }
                    }
                }
                3 => {
                    e_sus(local_24, 0x14, e_us(local_24, 0x14) | 0x10);
                }
                4 => {
                    e_sus(local_24, 0x14, e_us(local_24, 0x14) & 0xdfff);
                    if g(local_24, 0xef) == 0 {
                        FUN_1006b2cc(local_24);
                        gs(local_24, 0xef, DAT_100b38fc.wrapping_add(0x712));
                        FUN_1004fed5();
                    }
                    if DAT_100b38fc < g(local_24, 0xef) {
                        FUN_1006b2cc(local_24);
                    } else {
                        let q = local_24.add(0x110).cast::<u16>();
                        q.write_unaligned(q.read_unaligned() | 0x200);
                    }
                }
                7 => {
                    if g(local_24, 0xef) < DAT_100b38fc {
                        FUN_100069d3(local_24, 0);
                    }
                }
                _ => {}
            }
            if e_i(local_24, 4) == DAT_100adf38
                && (g(local_24, 3).wrapping_add(0x20000) < g(local_24, 2)
                    || g(local_24, 2) < g(local_24, 3).wrapping_sub(0x20000))
                && DAT_100ab094 != 0
            {
                let a = (e_i(local_24, 0x50) as u32)
                    .wrapping_sub(PTR_DAT_100aaf68.cast::<i32>().read_unaligned() as u32)
                    as i32;
                let b =
                    (e_i(local_24, 0x54) as u32).wrapping_sub(
                        PTR_DAT_100aaf68.byte_add(4).cast::<i32>().read_unaligned() as u32,
                    ) as i32;
                let c =
                    (e_i(local_24, 0x58) as u32).wrapping_sub(
                        PTR_DAT_100aaf68.byte_add(8).cast::<i32>().read_unaligned() as u32,
                    ) as i32;
                FUN_100037f0(a, b, c, 0x155, DAT_100ab094);
            }
            if local_c == 0 {
                e_sus(local_24, 0x14, e_us(local_24, 0x14) & 0xfffe);
            }
        }
    }
}
