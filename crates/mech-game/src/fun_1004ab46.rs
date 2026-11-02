// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b70: u32;
    static mut DAT_100a9670: u8;
    static mut DAT_100a9680: u8;
    static mut DAT_100a9684: u8;
    static mut DAT_100aafac: u32;
    static mut DAT_100aefa8: u32;
    static mut DAT_100b06e4: u32;
    static mut DAT_100b06e8: u32;
    static mut DAT_100b06f4: u32;
    static mut DAT_100b06f8: u32;
    static mut DAT_100b3904: u32;
    static mut DAT_10160bc0: u32;
    static mut DAT_10160bc4: u32;
    static mut DAT_10160bc8: u32;
    static mut DAT_10160bcc: u32;
    static mut DAT_10160bd0: u32;
    static mut DAT_10160bd4: u32;
    static mut DAT_101f26f0: u8;
    static mut DAT_101f26f4: u8;
    static mut DAT_101f26f8: u8;
    static mut DAT_101f26fc: u8;
    static mut DAT_101f2700: u8;
    static mut DAT_101f2704: u8;
    static mut DAT_101f2708: u8;
    static mut DAT_101f2714: u8;
    static mut DAT_101f2718: u8;
    static mut PTR_DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004be92(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_1007d0d3(param_1: i32, param_2: u8, param_3: *mut u32);
    fn FUN_1007d258(param_1: i32, param_2: i32);
    fn FUN_10030c40(param_1: i32, param_2: i32) -> i32;
    fn FUN_100142b4(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_1003109e(param_1: i32, param_2: u8, param_3: *mut u32);
    fn FUN_1001d5cb(param_1: i32) -> u32;
    fn FUN_1002fd94(param_1: *mut i32, param_2: *mut i32);
    fn FUN_10064114(param_1: i32, param_2: i32);
    fn FUN_1002f668(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: u32);
    fn FUN_1002f37a(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_1002f783(param_1: i32);
    fn FUN_1002f7f7(param_1: i32);
    fn FUN_1002fbc0(param_1: *mut i32);
    fn FUN_1005f81d(param_1: i16, param_2: u16);
    fn FUN_1005f7ad(param_1: i16, param_2: u16);
    fn FUN_10060864(param_1: i32, param_2: u32, param_3: i32);
    fn FUN_1004aa97(
        param_1: u32,
        param_2: u32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: u32,
        param_8: i32,
    );
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_100037f0(param_1: u32, param_2: u32, param_3: u32, param_4: u32, param_5: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1007d03e() -> i32;
    fn FUN_1001a620() -> u32;
}

unsafe fn rdi(p: *const u8) -> i32 {
    // SAFETY: caller supplies a valid game address.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn wri(p: *mut u8, v: i32) {
    // SAFETY: caller supplies a valid game address.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables, unused_mut)]
pub unsafe extern "C" fn FUN_1004ab46(
    param_1: u32,
    param_2: i32,
    param_3: u32,
    mut param_4: i32,
    mut param_5: i32,
    mut param_6: i32,
    param_7: i32,
    param_8: u32,
    param_9: i32,
    param_10: i32,
    param_11: i32,
    param_12: i32,
) {
    // SAFETY: all globals and pointers are the game's own memory, accessed
    // at the offsets the original uses.
    unsafe {
        let mut local_50: *mut u8 = core::ptr::null_mut();
        let mut local_20: i32 = 0;
        let mut local_1c: u32 = 0xffff_ffff;
        let mut local_48: i32 = 0;
        if (param_3 as i32) < 0 {
            return;
        }
        let f = param_3;
        let mut u1 = param_3 & 0xff;
        if u1 >= 0x24 {
            return;
        }
        let local_2c: u32 = ((f & 0x4000) != 0) as u32;
        match u1 {
            0 | 1 | 2 => {
                if f & 0x2000 == 0 {
                    if f & 0x400 != 0 {
                        u1 = 9;
                    }
                } else {
                    u1 = 0xe + u1;
                }
            }
            3 | 0x22 | 0x23 => {
                let mut p3;
                if f & 0x400 == 0 {
                    if f & 0x2000 == 0 {
                        p3 = u1;
                        if f & 0x200 != 0 {
                            p3 = 0x13;
                        }
                    } else {
                        p3 = 0x11;
                    }
                } else {
                    p3 = 0xc;
                }
                local_20 = 1;
                local_1c = 0x10b;
                local_48 = 1;
                u1 = p3;
            }
            4 => {
                let mut p3;
                if f & 0x400 == 0 {
                    if f & 0x2000 == 0 {
                        p3 = u1;
                        if f & 0x200 != 0 {
                            p3 = 0x14;
                        }
                    } else {
                        p3 = 0x11;
                    }
                } else {
                    p3 = 0xc;
                }
                local_20 = 1;
                local_1c = 0x10b;
                u1 = p3;
            }
            5 => {
                if f & 0x100 == 0 {
                    if f & 0x400 == 0 {
                        if f & 0x2000 != 0 {
                            u1 = 0x12;
                        }
                    } else {
                        u1 = 9;
                    }
                } else {
                    param_4 = param_7;
                    param_6 = param_9;
                    u1 = 8;
                }
            }
            6 => {
                if f & 0x400 != 0 {
                    u1 = 0x15;
                }
                local_48 = 1;
            }
            0xb => {
                let local_8: i32 = if f & 0x1000 == 0 {
                    if f & 0x100 == 0 {
                        if f & 0x200 == 0 { 8 } else { 4 }
                    } else {
                        4
                    }
                } else {
                    0x10
                };
                FUN_1004be92(param_4, param_5, param_6, local_8);
                return;
            }
            _ => {}
        }
        let param_3 = u1;
        let iv = (param_3 as i32).wrapping_mul(0x2c);
        let local_38: *mut u32 = (&raw mut DAT_100a9670 as *mut u8)
            .byte_add(iv as usize)
            .cast::<u32>();
        let pp = PTR_DAT_100aaf68 as usize as *mut u8;
        let local_18: u32 = (rdi(pp) as u32).wrapping_sub(param_4 as u32);
        let local_10: i32 = rdi(pp.byte_add(4)).wrapping_sub(param_5);
        let local_28: u32 = (rdi(pp.byte_add(8)) as u32).wrapping_sub(param_6 as u32);
        let mut local_3c: i32 = -1;
        if rdi((&raw mut DAT_100a9684 as *mut u8).byte_add(iv as usize)) != 0
            || rdi((&raw mut DAT_100a9680 as *mut u8).byte_add(iv as usize)) != 0
        {
            let mut local_a0: i32 = 0;
            let mut local_40: i32 = 0;
            while local_40 < 0xff {
                let iv = local_40.wrapping_mul(0x2c) as usize;
                local_50 = (&raw mut DAT_101f26f0 as *mut u8).byte_add(iv);
                let t = rdi((&raw mut DAT_101f2714 as *mut u8).byte_add(iv));
                if t as u32 == param_3 {
                    if rdi((&raw mut DAT_101f2708 as *mut u8).byte_add(iv)) == 0 {
                        if local_3c < 0
                            && (local_38.add(5).read_unaligned() == 0 || rdi(local_50) != 0)
                        {
                            local_3c = local_40;
                        }
                    } else if (t < 0x17 || 0x21 < t) && (local_38.add(10).read_unaligned() & 2) == 0
                    {
                        let r = FUN_1000fc30(
                            rdi((&raw mut DAT_101f26f8 as *mut u8).byte_add(iv))
                                .wrapping_sub(param_4),
                            rdi((&raw mut DAT_101f26fc as *mut u8).byte_add(iv))
                                .wrapping_sub(param_5),
                            rdi((&raw mut DAT_101f2700 as *mut u8).byte_add(iv))
                                .wrapping_sub(param_6),
                        );
                        if r < 500 {
                            local_a0 = local_a0.wrapping_add(1);
                            if DAT_100aefa8 != 1 {
                                return;
                            }
                            if 2 < local_a0 {
                                return;
                            }
                        }
                    }
                }
                local_40 = local_40.wrapping_add(1);
            }
            if local_3c < 0 {
                return;
            }
            let iv = local_3c.wrapping_mul(0x2c) as usize;
            local_50 = (&raw mut DAT_101f26f0 as *mut u8).byte_add(iv);
            wri((&raw mut DAT_101f2708 as *mut u8).byte_add(iv), 1);
            wri((&raw mut DAT_101f26f8 as *mut u8).byte_add(iv), param_4);
            wri((&raw mut DAT_101f26fc as *mut u8).byte_add(iv), param_5);
            wri((&raw mut DAT_101f2700 as *mut u8).byte_add(iv), param_6);
            wri(
                (&raw mut DAT_101f2704 as *mut u8).byte_add(iv),
                local_38.read_unaligned().wrapping_add(DAT_100b3904) as i32,
            );
            wri((&raw mut DAT_101f26f4 as *mut u8).byte_add(iv), 0);
            wri(
                (&raw mut DAT_101f2718 as *mut u8).byte_add(iv),
                param_1 as i32,
            );
            let l50 = local_50.cast::<i32>();
            if local_48 != 0 {
                let h = FUN_1007d03e();
                if -1 < h {
                    let mut s = [0u32; 11];
                    s[0] = 1;
                    s[1] = param_4 as u32;
                    s[2] = param_5 as u32;
                    s[3] = param_6 as u32;
                    s[8] = 20000;
                    s[10] = l50.add(5).read_unaligned() as u32;
                    FUN_1007d0d3(h, 0xf, s.as_mut_ptr());
                    FUN_1007d258(h, 1);
                }
                let local_4c = FUN_10030c40(6, 0);
                if -1 < local_4c {
                    let mut s = [0u32; 5];
                    s[0] = param_4 as u32;
                    s[1] = FUN_100142b4(param_4, param_5, param_6) as u32;
                    s[2] = param_6 as u32;
                    s[4] = 5000;
                    s[3] = l50.add(5).read_unaligned() as u32;
                    FUN_1003109e(local_4c, 7, s.as_mut_ptr());
                }
            }
            if rdi(local_50) != 0 {
                if local_38.add(10).read_unaligned() & 1 != 0 {
                    let mut use_default = true;
                    if param_2 != 0 {
                        let w = (param_2 as usize as *const u8)
                            .byte_add(2)
                            .cast::<u16>()
                            .read_unaligned();
                        if w & 0x400 == 0 && w & 0x100 != 0 {
                            let l34 = FUN_1001d5cb(param_2) as usize as *mut i32;
                            if !l34.is_null() {
                                use_default = false;
                                param_6 = 0;
                                param_5 = 0;
                                param_4 = 0;
                                l50.add(1).write_unaligned(param_2);
                                FUN_1002fd94(rdi(local_50) as usize as *mut i32, l34);
                            }
                        }
                    }
                    if use_default {
                        l50.add(5)
                            .write_unaligned(DAT_100b3904.wrapping_add(0xb5) as i32);
                    }
                }
                if param_3 < 0x17 || 0x21 < param_3 || DAT_100b06f4 == 0 {
                    FUN_1002f668(rdi(local_50), param_10, param_11, param_12, 0);
                    FUN_1002f37a(rdi(local_50), param_4, param_5, param_6);
                } else {
                    FUN_10064114(DAT_100b06f4 as i32, rdi(local_50));
                    DAT_100b06f4 = 0;
                }
                FUN_1002f783(rdi(local_50));
                FUN_1002f7f7(rdi(local_50));
                FUN_1002fbc0(rdi(local_50) as usize as *mut i32);
                let v7 = l50.add(7).read_unaligned();
                if v7 != -1 {
                    FUN_1005f81d(v7 as i16, 0);
                    FUN_1005f7ad(
                        l50.add(7).read_unaligned() as i16,
                        local_38.add(7).read_unaligned() as u16,
                    );
                }
            }
            if local_38.add(4).read_unaligned() != 0 && DAT_100b06f8 != 0 {
                let mut local_6c = 0;
                let mut local_68 = 1;
                if DAT_100b06e4 != 0 {
                    let local_c: i32 = local_18
                        .wrapping_mul(local_18)
                        .wrapping_add(local_28.wrapping_mul(local_28))
                        as i32;
                    let local_24 = rdi(pp.byte_add(0x1c)).wrapping_sub(rdi(pp));
                    let local_44 = rdi(pp.byte_add(0x24)).wrapping_sub(rdi(pp.byte_add(8)));
                    let local_30 = local_44
                        .wrapping_mul(local_44)
                        .wrapping_add(local_24.wrapping_mul(local_24));
                    if local_c < local_30 {
                        local_6c = 1;
                    } else {
                        local_68 = 0;
                    }
                }
                if local_68 != 0 {
                    if local_6c == 0 {
                        DAT_100b06e4 = 1;
                        DAT_100a7b70 = 0;
                        DAT_10160bc0 = rdi(pp.byte_add(0x1c)) as u32;
                        DAT_10160bc4 = rdi(pp.byte_add(0x20)) as u32;
                        DAT_10160bc8 = rdi(pp.byte_add(0x24)) as u32;
                        DAT_10160bcc =
                            pp.byte_add(0x28).cast::<i16>().read_unaligned() as i32 as u32;
                        DAT_10160bd0 =
                            pp.byte_add(0x2a).cast::<i16>().read_unaligned() as i32 as u32;
                        let nv = pp
                            .byte_add(0x2a)
                            .cast::<i16>()
                            .read_unaligned()
                            .wrapping_add(-10);
                        pp.byte_add(0x2a).cast::<i16>().write_unaligned(nv);
                        let c = pp.byte_add(0x2a).cast::<i16>().read_unaligned();
                        if 0xff < c || c < 0 {
                            pp.byte_add(0x2a).cast::<u16>().write_unaligned(0x40);
                        }
                        DAT_10160bd4 = DAT_100aafac;
                        pp.byte_add(0x28).cast::<u16>().write_unaligned(0);
                        DAT_100aafac = 0;
                    }
                    DAT_100b06e8 = local_3c as u32;
                    l50.add(8).write_unaligned(1);
                    wri(pp.byte_add(0x1c), param_7);
                    pp.byte_add(0x20).cast::<u32>().write_unaligned(param_8);
                    wri(pp.byte_add(0x24), param_9);
                    let v1 = local_38.add(1).read_unaligned() as i32;
                    if -1 < v1 {
                        FUN_10060864(v1, local_38.read_unaligned(), 1);
                    }
                }
            }
        }
        if local_20 != 0 {
            FUN_1004aa97(
                param_1, local_1c, param_4, param_5, param_6, param_7, param_8, param_9,
            );
        }
        if 0 < local_38.add(2).read_unaligned() as i32 {
            let v3 = local_38.add(3).read_unaligned();
            if v3 == 0xffff_ffff || FUN_10051f43(100) < v3 as i32 {
                let r = FUN_1001a620();
                let local_a4: i32 = if r == 0 && local_2c == 0 { 1 } else { 0 };
                FUN_100037f0(
                    local_18,
                    local_10 as u32,
                    local_28,
                    local_38.add(2).read_unaligned(),
                    local_a4,
                );
            }
        }
    }
}
