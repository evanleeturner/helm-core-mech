// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf34: *mut i8;
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: i32;
    static mut DAT_100aa1ac: i32;
    static mut DAT_10165863: u8;
    static mut DAT_1016587a: u8;
    static mut DAT_101ec750: i8;
    static mut DAT_101ec780: i8;
    static mut DAT_101ec78c: i8;
    static mut DAT_100a40a8: i32;
    static mut DAT_100b3790: i32;
    static mut DAT_100b06ec: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063e41(param_1: i32);
    fn FUN_1005f81d(param_1: i16, param_2: i32);
    fn FUN_1005f7ad(param_1: i16, param_2: i32);
    fn FUN_10063f4d(param_1: i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32);
    fn FUN_1000fef4(param_1: *mut u32, param_2: *mut u32, param_3: *mut u32);
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1006416e(
        param_1: i32,
        param_2: i32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
        param_6: i32,
        param_7: i32,
    );
    fn FUN_1002f783(param_1: i32);
    fn FUN_1002f7f7(param_1: i32);
    fn FUN_1002fbc0(param_1: *mut i32);
    fn FUN_1002f322(param_1: i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32);
    fn FUN_10025061(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
    fn FUN_1007d03e() -> i32;
    fn FUN_1007d0d3(param_1: i32, param_2: i32, param_3: *mut i32);
    fn FUN_10030c40(param_1: i32, param_2: i32) -> i32;
}

unsafe fn rd32(a: i32) -> i32 {
    // SAFETY: caller supplies a valid game address.
    unsafe { (a as usize as *const i32).read_unaligned() }
}

unsafe fn wr32(a: i32, v: i32) {
    // SAFETY: caller supplies a valid game address.
    unsafe { (a as usize as *mut i32).write_unaligned(v) }
}

unsafe fn rd16(a: i32) -> i16 {
    // SAFETY: caller supplies a valid game address.
    unsafe { (a as usize as *const i16).read_unaligned() }
}

unsafe fn wr16(a: i32, v: i16) {
    // SAFETY: caller supplies a valid game address.
    unsafe { (a as usize as *mut i16).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10062775(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    // SAFETY: all addresses are the game's own memory, accessed as the original does.
    unsafe {
        let mut local_c: i32 = 1;
        if param_1 == 0 || rd32(param_1.wrapping_add(0x48)) == 0 {
            local_c = 0;
        } else {
            let local_14 = rd32(param_1.wrapping_add(0x20));
            let mut local_1c: i32 = 0;
            let local_8: i32 =
                if rd32(param_3.wrapping_add(0x20)) < rd32(local_14.wrapping_add(0x60)) {
                    (rd32(param_3.wrapping_add(0x20)) as u32)
                        .wrapping_mul(0x3a)
                        .wrapping_add(local_14 as u32)
                        .wrapping_add(100) as i32
                } else {
                    0
                };
            let mut f2: i32 = 0;
            if local_8 != 0 {
                f2 = rd32(local_8.wrapping_add(2));
            }
            if local_8 == 0 || f2 != 0 || *DAT_100adf34 != 0 {
                if rd32(param_1.wrapping_add(4)) == DAT_100adf38 {
                    let p = (&raw mut DAT_10165863).cast::<i32>();
                    p.write_unaligned(p.read_unaligned().wrapping_add(1));
                }
                let tbl = (&raw mut DAT_100eef40).cast::<i32>();
                let a = tbl.offset(rd32(param_1.wrapping_add(4)) as isize).read();
                let b = tbl.offset(DAT_100adf38 as isize).read();
                if rd32(a.wrapping_add(8)) == rd32(b.wrapping_add(8)) {
                    let p = (&raw mut DAT_1016587a).cast::<i32>();
                    p.write_unaligned(p.read_unaligned().wrapping_add(1));
                }
                if local_8 != 0 {
                    f2 = rd32(local_8.wrapping_add(2));
                }
                if local_8 != 0
                    && f2 != -1
                    && (*DAT_100adf34 == 0 || rd32(param_1.wrapping_add(4)) != DAT_100adf38)
                    && (DAT_100aa1ac == 0 || rd32(param_1.wrapping_add(4)) == DAT_100adf38)
                {
                    f2 = f2.wrapping_sub(1);
                    wr32(local_8.wrapping_add(2), f2);
                    let f36 = rd32(local_8.wrapping_add(0x36));
                    if rd16(f36.wrapping_add(2)) == 0 {
                        let mut fa = rd32(local_8.wrapping_add(0xa));
                        fa = fa.wrapping_add(1);
                        wr32(local_8.wrapping_add(0xa), fa);
                        let f6 = rd32(local_8.wrapping_add(6));
                        fa = rd32(local_8.wrapping_add(0xa));
                        if f6 == fa {
                            fa = fa.wrapping_sub(1);
                            wr32(local_8.wrapping_add(0xa), fa);
                        } else {
                            let mut g = rd32(local_8.wrapping_add(0x36));
                            g = g.wrapping_add(0x1e);
                            wr32(local_8.wrapping_add(0x36), g);
                        }
                    } else {
                        wr16(
                            f36.wrapping_add(2),
                            rd16(f36.wrapping_add(2)).wrapping_add(-1),
                        );
                    }
                    f2 = rd32(local_8.wrapping_add(2));
                    if f2 == 0 {
                        let mut local_20 = rd32(local_14.wrapping_add(0x54));
                        let mut local_18: i32 = 0;
                        while local_18 < 0xe {
                            let v = rd32(local_20.wrapping_add(0x3c)) / 100;
                            if (v as u32)
                                == u32::from((local_8 as usize as *const u16).read_unaligned())
                            {
                                wr32(local_20.wrapping_add(8), -1);
                            }
                            local_20 = local_20.wrapping_add(0x40);
                            local_18 += 1;
                        }
                        local_c = 0;
                    }
                }
                if rd32(param_2.wrapping_add(0x24)) != 0 {
                    let mut local_10: *mut i32 = core::ptr::null_mut();
                    let mut local_68: i32 = 0;
                    while local_1c == 0 && local_68 < 0xff {
                        let ivar1 = local_68.wrapping_mul(0x60) as isize;
                        local_10 = (&raw mut DAT_101ec750).byte_offset(ivar1).cast::<i32>();
                        if local_10.read_unaligned() == rd32(param_2.wrapping_add(0x10))
                            && (&raw mut DAT_101ec78c)
                                .byte_offset(ivar1)
                                .cast::<i32>()
                                .read_unaligned()
                                == 0
                            && (&raw mut DAT_101ec780)
                                .byte_offset(ivar1)
                                .cast::<i32>()
                                .read_unaligned()
                                != 0
                        {
                            local_1c = 1;
                        }
                        local_68 += 1;
                    }
                    if local_1c != 0 {
                        if rd32(param_1.wrapping_add(4)) != DAT_100adf38 {
                            FUN_10063e41(param_1);
                        }
                        let l = local_10;
                        l.add(1).write_unaligned(rd32(param_2.wrapping_add(8)));
                        l.add(9).write_unaligned(rd32(param_3.wrapping_add(0x10)));
                        l.add(10).write_unaligned(
                            rd32(param_2.wrapping_add(0x5c))
                                .wrapping_sub(rd32(param_3.wrapping_add(0x10))),
                        );
                        l.add(2).write_unaligned(rd32(param_2.wrapping_add(0x14)));
                        l.add(0x12)
                            .write_unaligned(rd32(param_2.wrapping_add(0x40)));
                        l.add(0x13)
                            .write_unaligned(rd32(param_2.wrapping_add(0x48)));
                        l.add(0xf).write_unaligned(1);
                        l.add(0x10).write_unaligned(0);
                        l.add(0xd).write_unaligned(rd32(param_3.wrapping_add(0x18)));
                        l.add(0xe).write_unaligned(rd32(param_3.wrapping_add(0x1c)));
                        l.add(0x11).write_unaligned(rd32(param_1.wrapping_add(4)));
                        if ((param_2.wrapping_add(8) as usize as *const u8).read() & 0x20) != 0 {
                            let mut local_74: i32 = 0;
                            while local_74 < 8 {
                                FUN_1005f81d(
                                    (rd32(param_2.wrapping_add(0xc)) as i16)
                                        .wrapping_add(local_74 as i16),
                                    0,
                                );
                                FUN_1005f7ad(
                                    (rd32(param_2.wrapping_add(0xc)) as i16)
                                        .wrapping_add(local_74 as i16),
                                    1,
                                );
                                local_74 += 1;
                            }
                        }
                        let mut ivar1 = rd32(param_2.wrapping_add(0x3c));
                        let mut local_24: u32 = 0;
                        let mut local_2c: u32 = 0;
                        let mut local_34: u32 = 0;
                        let mut local_28: i32 = 0;
                        let mut local_30: i32 = 0;
                        let mut local_64: [i32; 10] = [0; 10];
                        FUN_10063f4d(
                            param_1,
                            (&raw mut local_24).cast::<i32>(),
                            (&raw mut local_2c).cast::<i32>(),
                            (&raw mut local_34).cast::<i32>(),
                        );
                        FUN_1000fef4(&raw mut local_24, &raw mut local_2c, &raw mut local_34);
                        l.add(3)
                            .write_unaligned((ivar1 as u32).wrapping_mul(local_24) as i32);
                        l.add(4)
                            .write_unaligned(local_2c.wrapping_mul(ivar1 as u32) as i32);
                        l.add(5)
                            .write_unaligned(local_34.wrapping_mul(ivar1 as u32) as i32);
                        ivar1 = FUN_10003d60(
                            DAT_100a40a8.wrapping_neg(),
                            rd32(param_2.wrapping_add(0x38)).wrapping_shl(16),
                        );
                        l.add(7).write_unaligned(ivar1);
                        l.add(8).write_unaligned(0);
                        l.add(6).write_unaligned(l.add(8).read_unaligned());
                        let k = l.read_unaligned();
                        let local_6c: i32 = if k == 3 || k == 10 || k == 0xc || k == 9 || k == 4 {
                            1
                        } else {
                            0
                        };
                        FUN_1006416e(
                            param_1,
                            l.add(0xc).read_unaligned(),
                            local_24,
                            local_2c,
                            local_34,
                            local_6c,
                            param_2,
                        );
                        FUN_1002f783(l.add(0xc).read_unaligned());
                        FUN_1002f7f7(l.add(0xc).read_unaligned());
                        FUN_1002fbc0(l.add(0xc).read_unaligned() as usize as *mut i32);
                        FUN_1002f322(
                            l.add(0xc).read_unaligned(),
                            &raw mut local_28,
                            &raw mut local_30,
                            local_64.as_mut_ptr(),
                        );
                        if l.read_unaligned() == 5 {
                            FUN_10025061(
                                (l.add(3).read_unaligned() as u32)
                                    .wrapping_mul(2)
                                    .wrapping_add(local_28 as u32)
                                    as i32,
                                (l.add(4).read_unaligned() as u32)
                                    .wrapping_mul(2)
                                    .wrapping_add(local_30 as u32)
                                    as i32,
                                (l.add(5).read_unaligned() as u32)
                                    .wrapping_mul(2)
                                    .wrapping_add(local_64[0] as u32)
                                    as i32,
                                l.add(3).read_unaligned(),
                                l.add(4).read_unaligned(),
                                l.add(5).read_unaligned(),
                            );
                        }
                        if DAT_100b3790 != 0 {
                            ivar1 = FUN_1007d03e();
                            l.add(0x16).write_unaligned(ivar1);
                            local_64[1] = 1;
                            local_64[2] = local_24 as i32;
                            local_64[3] = local_2c as i32;
                            local_64[4] = local_34 as i32;
                            local_64[9] = 10000;
                            FUN_1007d0d3(
                                l.add(0x16).read_unaligned(),
                                7,
                                local_64.as_mut_ptr().add(1),
                            );
                        }
                        ivar1 = FUN_10030c40(2, l as usize as i32);
                        l.add(0x17).write_unaligned(ivar1);
                        if rd32(param_1.wrapping_add(4)) == DAT_100adf38 {
                            let k = l.read_unaligned();
                            if k == 3 || k == 4 || k == 10 || k == 0xc || k == 9 {
                                DAT_100b06ec = local_68.wrapping_add(-1);
                            } else {
                                DAT_100b06ec = -1;
                            }
                        }
                    }
                }
            } else {
                local_c = 0;
            }
        }
        local_c
    }
}
