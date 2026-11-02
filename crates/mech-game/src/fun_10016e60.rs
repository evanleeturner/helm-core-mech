// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aae80: u32;
    static mut DAT_100aaf74: u32;
    static mut DAT_100aaf7c: u32;
    static mut DAT_100aafbc: u32;
    static mut DAT_101eb480: u32;
    static mut DAT_101eb484: u32;
    static mut DAT_101f52e0: u8;
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100242e8(param_1: u32);
    fn FUN_10031b85(param_1: i32);
    fn FUN_1003eb9c(param_1: *mut i32, param_2: i32, param_3: *mut u32);
    fn FUN_1003f893(param_1: *mut i32, param_2: i32, param_3: i32, param_4: *mut u32);
    fn FUN_10019df1(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_1001a040(param_1: u32, param_2: u32) -> u32;
    fn FUN_1001a060(param_1: i32, param_2: i32) -> i32;
    fn FUN_1005f040(
        param_1: i32,
        param_2: i32,
        param_3: *mut u32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u32;
    fn FUN_10055f10(param_1: u32, param_2: i32, param_3: *mut i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10016e60(param_1: i32, param_2: *mut u32, param_3: u32) {
    // SAFETY: the globals and callees are the game's own; param_2 points at
    // the caller's array of 6-word records, accessed where the original does.
    unsafe {
        let mut param_3 = param_3;
        let mut u_var2: u32 = param_3 & 0xf000;
        let mut local_1c: *mut u32 = param_2;
        let mut local_24: i32 = 1;
        let mut local_18: i32;

        if u_var2 < 0x1001 {
            if u_var2 == 0x1000 || u_var2 == 0 {
                u_var2 = param_3 & 0xff;
                if u_var2 < 0x30
                    || u_var2 == 0x60
                    || u_var2 == 0xea
                    || u_var2 == 0xd0
                    || u_var2 == 0xdf
                    || u_var2 == 0x7f
                    || (0x5f < u_var2 && u_var2 < 0x80)
                {
                    param_3 = u_var2;
                    if DAT_100aae80 != 0 && u_var2 == 0xd0 {
                        param_3 = 0xdf;
                    }
                    local_18 = 0;
                    while local_18 < param_1 {
                        local_1c.add(2).write_unaligned(param_3.wrapping_shl(0x10));
                        local_1c = local_1c.add(6);
                        local_18 = local_18.wrapping_add(1);
                    }
                    FUN_1003eb9c((&raw mut DAT_101f52e0).cast::<i32>(), param_1, param_2);
                } else {
                    if DAT_100aae80 == 0 {
                        local_18 = 0;
                        while local_18 < param_1 {
                            let v = local_1c.add(2).read_unaligned();
                            local_1c.add(2).write_unaligned(
                                v.wrapping_add(param_3 & 0xf0).wrapping_mul(0x10000),
                            );
                            local_1c = local_1c.add(6);
                            local_18 = local_18.wrapping_add(1);
                        }
                    } else {
                        local_18 = 0;
                        while local_18 < param_1 {
                            let v = local_1c.add(2).read_unaligned();
                            local_1c.add(2).write_unaligned(
                                0xfu32
                                    .wrapping_sub(v)
                                    .wrapping_add(param_3 & 0xf0)
                                    .wrapping_mul(0x10000),
                            );
                            local_1c = local_1c.add(6);
                            local_18 = local_18.wrapping_add(1);
                        }
                    }
                    FUN_1003f893(
                        (&raw mut DAT_101f52e0).cast::<i32>(),
                        0x7fff,
                        param_1,
                        param_2,
                    );
                }
            }
        } else if u_var2 < 0x3001 {
            if u_var2 == 0x3000 {
                FUN_10055f10(param_3, param_1, param_2.cast::<i32>(), -1);
            } else if u_var2 == 0x2000 && DAT_100aaf7c != 0 {
                if DAT_100aafbc == 0 {
                    local_18 = 0;
                    while local_18 < param_1 {
                        let base = DAT_100aaf68.wrapping_add(0x94) as *const u32;
                        let a =
                            FUN_1001a040(base.read_unaligned(), local_1c.add(5).read_unaligned());
                        local_1c.add(5).write_unaligned(a);
                        let b = FUN_1001a060(
                            local_1c.add(3).read_unaligned() as i32,
                            local_1c.add(5).read_unaligned() as i32,
                        );
                        local_1c.add(3).write_unaligned(b as u32);
                        let c = FUN_1001a060(
                            local_1c.add(4).read_unaligned() as i32,
                            local_1c.add(5).read_unaligned() as i32,
                        );
                        local_1c.add(4).write_unaligned(c as u32);
                        let d = local_1c.add(2).read_unaligned();
                        local_1c.add(2).write_unaligned(d.wrapping_shl(0x10));
                        local_1c = local_1c.add(6);
                        local_18 = local_18.wrapping_add(1);
                    }
                }
                FUN_1005f040(
                    (param_3 & 0xff) as i32,
                    param_1,
                    param_2,
                    ((param_3 & 0xf00) >> 8) as i32,
                    0,
                    1,
                    0,
                );
            }
        } else if u_var2 < 0x5001 {
            if u_var2 == 0x5000 {
                if DAT_100aaf7c != 0 {
                    if DAT_100aafbc == 0 {
                        u_var2 = param_2.add(2).read_unaligned();
                        local_18 = 0;
                        while local_18 < param_1 {
                            let base = DAT_100aaf68.wrapping_add(0x94) as *const u32;
                            let a = FUN_1001a040(
                                base.read_unaligned(),
                                local_1c.add(5).read_unaligned(),
                            );
                            local_1c.add(5).write_unaligned(a);
                            let b = FUN_1001a060(
                                local_1c.add(3).read_unaligned() as i32,
                                local_1c.add(5).read_unaligned() as i32,
                            );
                            local_1c.add(3).write_unaligned(b as u32);
                            let c = FUN_1001a060(
                                local_1c.add(4).read_unaligned() as i32,
                                local_1c.add(5).read_unaligned() as i32,
                            );
                            local_1c.add(4).write_unaligned(c as u32);
                            if local_24 == 0
                                || 3 < (u_var2 as i32)
                                || (u_var2 as i32) < (local_1c.add(2).read_unaligned() as i32)
                            {
                                local_24 = 0;
                            } else {
                                local_24 = 1;
                            }
                            let d = local_1c.add(2).read_unaligned();
                            local_1c.add(2).write_unaligned(d.wrapping_shl(0x10));
                            local_1c = local_1c.add(6);
                            local_18 = local_18.wrapping_add(1);
                        }
                    }
                    FUN_1005f040(
                        (param_3 & 0xff) as i32,
                        param_1,
                        param_2,
                        ((param_3 & 0xf00) >> 8) as i32,
                        0,
                        1,
                        local_24,
                    );
                }
            } else if u_var2 == 0x4000 {
                if DAT_100aae80 == 0 {
                    local_18 = 0;
                    while local_18 < param_1 {
                        if (local_1c.add(3).read_unaligned() as i32) < 0x300000 {
                            let v = local_1c.add(3).read_unaligned();
                            local_1c.add(2).write_unaligned(v);
                        } else {
                            let v = local_1c.add(2).read_unaligned();
                            local_1c.add(2).write_unaligned(
                                v.wrapping_add(param_3 & 0xf0).wrapping_mul(0x10000),
                            );
                        }
                        local_1c = local_1c.add(6);
                        local_18 = local_18.wrapping_add(1);
                    }
                } else {
                    local_18 = 0;
                    while local_18 < param_1 {
                        if (local_1c.add(3).read_unaligned() as i32) < 0x300000 {
                            let v = local_1c.add(3).read_unaligned();
                            local_1c.add(2).write_unaligned(v);
                        } else {
                            let v = local_1c.add(2).read_unaligned();
                            local_1c.add(2).write_unaligned(
                                0xfu32
                                    .wrapping_sub(v)
                                    .wrapping_add(param_3 & 0xf0)
                                    .wrapping_mul(0x10000),
                            );
                        }
                        local_1c = local_1c.add(6);
                        local_18 = local_18.wrapping_add(1);
                    }
                }
                if DAT_100aaf74 == 0 {
                    FUN_1003eb9c((&raw mut DAT_101f52e0).cast::<i32>(), param_1, param_2);
                } else {
                    FUN_1003f893(
                        (&raw mut DAT_101f52e0).cast::<i32>(),
                        0x7fff,
                        param_1,
                        param_2,
                    );
                }
            }
        } else if u_var2 < 0x7001 {
            if u_var2 == 0x7000 {
                if DAT_100aaf7c != 0 {
                    FUN_1005f040(
                        (param_3 & 0xff) as i32,
                        param_1,
                        param_2,
                        ((param_3 & 0xf00) >> 8) as i32,
                        0,
                        0,
                        0,
                    );
                }
            } else if u_var2 == 0x6000 && DAT_100aaf7c != 0 {
                u_var2 = param_2.add(2).read_unaligned();
                local_18 = 0;
                let mut i_var1: u32;
                loop {
                    i_var1 = DAT_100aafbc;
                    if !(local_18 < param_1) {
                        break;
                    }
                    if local_24 == 0
                        || 3 < (u_var2 as i32)
                        || (u_var2 as i32) < (local_1c.add(2).read_unaligned() as i32)
                    {
                        local_24 = 0;
                    } else {
                        local_24 = 1;
                    }
                    let d = local_1c.add(2).read_unaligned();
                    local_1c.add(2).write_unaligned(d.wrapping_shl(0x10));
                    local_1c = local_1c.add(6);
                    local_18 = local_18.wrapping_add(1);
                }
                DAT_100aafbc = 1;
                FUN_1005f040(
                    (param_3 & 0xff) as i32,
                    param_1,
                    param_2,
                    ((param_3 & 0xf00) >> 8) as i32,
                    0,
                    1,
                    local_24,
                );
                DAT_100aafbc = i_var1;
            }
        } else if u_var2 == 0x8000 {
            FUN_100242e8(param_3 & 0xfff);
        } else if u_var2 == 0x9000 {
            FUN_10031b85((param_3 & 0xfff) as i32);
        } else if u_var2 == 0xa000 {
            DAT_101eb480 = param_2.read_unaligned();
            DAT_101eb484 = param_2.add(1).read_unaligned();
            local_18 = 1;
            while local_18 < param_1 {
                let idx = (local_18 as isize).wrapping_mul(6);
                FUN_10019df1(
                    param_2.offset(idx).read_unaligned() as i32,
                    param_2.offset(idx + 1).read_unaligned() as i32,
                    (param_3 & 0xff) as i32,
                );
                local_18 = local_18.wrapping_add(1);
            }
            FUN_10019df1(
                param_2.read_unaligned() as i32,
                param_2.add(1).read_unaligned() as i32,
                (param_3 & 0xff) as i32,
            );
        }
    }
}
