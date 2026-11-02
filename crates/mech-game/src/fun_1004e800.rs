// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: i32;
    static mut DAT_100b0710: i32;
    static mut DAT_10123120: i32;
    static mut DAT_10123230: i32;
    static mut DAT_10123130: [i32; 0];
    static mut DAT_10123170: [i32; 0];
    static mut DAT_100eef40: [i32; 0];
    static mut DAT_101600e0: [u8; 0];
    static mut DAT_10123240: [u8; 0];
    static mut DAT_10123248: [u8; 0];
    static mut DAT_10123278: [u8; 0];
    static mut DAT_1012327a: [u8; 0];
    static mut DAT_1012327c: [u8; 0];
    static mut DAT_101232e1: [u8; 0];
    static mut DAT_101232e5: [u8; 0];
    static mut DAT_101232e9: [u8; 0];
    static mut DAT_101232ed: [u8; 0];
    static mut DAT_101232ee: [u8; 0];
    static mut DAT_101232ef: [u8; 0];
    static mut DAT_10123323: [u8; 0];
    static mut DAT_10123325: [u8; 0];
    static mut DAT_10123368: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d91a(param_1: *mut u8) -> i32;
    fn FUN_1004e6d2(param_1: i32, param_2: i32) -> i32;
    fn FUN_1004d9d6(param_1: *mut u8, param_2: u8) -> i32;
    fn FUN_1004ddbe(param_1: *mut u8, param_2: i32) -> i32;
    fn FUN_1004db2b(param_1: *mut u8, param_2: i32) -> i32;
    fn FUN_1004e457(param_1: i32, param_2: u32);
    fn FUN_1004e12e(param_1: i32, param_2: i32, param_3: u32);
}

#[inline(always)]
fn off(p: *mut u8, n: i32) -> *mut u8 {
    p.wrapping_offset(n as isize)
}

#[inline(always)]
unsafe fn r32(p: *mut u8) -> i32 {
    // SAFETY: caller passes a valid game-memory address.
    unsafe { p.cast::<i32>().read_unaligned() }
}

#[inline(always)]
unsafe fn w32(p: *mut u8, v: i32) {
    // SAFETY: caller passes a valid game-memory address.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}

#[inline(always)]
unsafe fn r16(p: *mut u8) -> i16 {
    // SAFETY: caller passes a valid game-memory address.
    unsafe { p.cast::<i16>().read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004e800(param_1: i32, param_2: i32) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // offsets mirror the original's record layout.
    unsafe {
        let d_1012327a = (&raw mut DAT_1012327a).cast::<u8>();
        let d_1012327c = (&raw mut DAT_1012327c).cast::<u8>();
        let d_101232e1 = (&raw mut DAT_101232e1).cast::<u8>();
        let d_101232e5 = (&raw mut DAT_101232e5).cast::<u8>();
        let d_101232e9 = (&raw mut DAT_101232e9).cast::<u8>();
        let d_101232ed = (&raw mut DAT_101232ed).cast::<u8>();
        let d_101232ee = (&raw mut DAT_101232ee).cast::<u8>();
        let d_101232ef = (&raw mut DAT_101232ef).cast::<u8>();
        let d_10123240 = (&raw mut DAT_10123240).cast::<u8>();
        let d_10123248 = (&raw mut DAT_10123248).cast::<u8>();
        let d_10123278 = (&raw mut DAT_10123278).cast::<u8>();
        let d_10123323 = (&raw mut DAT_10123323).cast::<u8>();
        let d_10123325 = (&raw mut DAT_10123325).cast::<u8>();
        let d_10123368 = (&raw mut DAT_10123368).cast::<u8>();
        let d_101600e0 = (&raw mut DAT_101600e0).cast::<u8>();
        let d_10123130 = (&raw mut DAT_10123130).cast::<i32>();
        let d_10123170 = (&raw mut DAT_10123170).cast::<i32>();
        let d_100eef40 = (&raw mut DAT_100eef40).cast::<i32>();

        let mut b_var2: bool = true;
        let mut b_var3: bool = true;
        let mut local_1c: i8 = 3;
        let mut local_14: i32 = 0;
        let i_var5: i32 = param_1
            .wrapping_mul(0x3c0a)
            .wrapping_add(param_2.wrapping_mul(0x13f));
        let pc_var6: *mut u8 = off(d_1012327a, i_var5);
        let cnt_p: *mut u8 = off(d_10123368, i_var5);

        if DAT_100aa1ac != 0 && *pc_var6 == 5 {
            let tmp = r32(off(d_1012327c, i_var5));
            if tmp == 2 {
                local_14 = 0;
                while local_14 < *cnt_p as i32 {
                    let e = off(pc_var6, local_14.wrapping_mul(2).wrapping_add(0xef));
                    if b_var2 && FUN_1004d91a(e) != 0 {
                        b_var2 = true;
                    } else {
                        b_var2 = false;
                    }
                    local_14 = local_14.wrapping_add(1);
                }
                if !b_var2 {
                    w32(off(d_101232e1, i_var5), -1);
                    w32(off(d_101232e5, i_var5), -1);
                    *pc_var6 = 3;
                }
            }
        }

        if *pc_var6 == 5 || *pc_var6 == 6 || FUN_1004e6d2(param_1, param_2) == 0 {
            *off(d_101232ed, i_var5) = 0;
            return;
        }

        if r32(off(d_101232e1, i_var5)) == -1 {
            w32(off(d_101232e1, i_var5), DAT_10123120);
            *off(d_101232ed, i_var5) = 1;
        }

        let mut i_var7: i32 = r32(off(d_1012327c, i_var5));
        if i_var7 < 9 {
            if i_var7 == 8 {
                local_14 = 0;
                while local_14 < *cnt_p as i32 {
                    let e = off(pc_var6, local_14.wrapping_mul(2).wrapping_add(0xef));
                    if b_var2 && FUN_1004d9d6(e, param_1 as u8) != 0 {
                        b_var2 = true;
                    } else {
                        b_var2 = false;
                    }
                    local_14 = local_14.wrapping_add(1);
                }
            } else {
                match i_var7 {
                    1 | 2 => {
                        local_14 = 0;
                        while local_14 < *cnt_p as i32 {
                            let e = off(pc_var6, local_14.wrapping_mul(2).wrapping_add(0xef));
                            if b_var2 && FUN_1004d91a(e) != 0 {
                                b_var2 = true;
                            } else {
                                b_var2 = false;
                            }
                            local_14 = local_14.wrapping_add(1);
                        }
                    }
                    4 => {
                        local_14 = 0;
                        while local_14 < *cnt_p as i32 {
                            let e = off(pc_var6, local_14.wrapping_mul(2).wrapping_add(0xef));
                            let u_var4 = FUN_1004d91a(e);
                            if b_var2 && u_var4 == 0 {
                                b_var2 = true;
                            } else {
                                b_var2 = false;
                            }
                            if b_var3 && u_var4 != 0 {
                                b_var3 = true;
                            } else {
                                b_var3 = false;
                            }
                            local_14 = local_14.wrapping_add(1);
                        }
                    }
                    _ => {}
                }
            }
        } else if i_var7 < 0x21 {
            if i_var7 == 0x20 {
                local_14 = 0;
                while local_14 < *cnt_p as i32 {
                    let e = off(pc_var6, local_14.wrapping_mul(2).wrapping_add(0xef));
                    if b_var2 && FUN_1004d9d6(e, param_1 as u8) != 0 {
                        b_var2 = true;
                    } else {
                        b_var2 = false;
                    }
                    local_14 = local_14.wrapping_add(1);
                }
                let base = param_1.wrapping_mul(0x3c0a);
                local_14 = 0;
                loop {
                    let tmp = r32(off(d_10123240, base));
                    if !(local_14 < tmp) {
                        break;
                    }
                    let rec = base.wrapping_add(local_14.wrapping_mul(0x13f));
                    let tmp2 = r32(off(d_101232ef, rec));
                    if tmp2 != 0 && local_14 != param_2 {
                        if b_var2 && *off(d_1012327a, rec) == 5 {
                            b_var2 = true;
                        } else {
                            b_var2 = false;
                        }
                    }
                    local_14 = local_14.wrapping_add(1);
                }
            }
        } else if i_var7 < 0x81 {
        } else if i_var7 < 0x201 {
            if i_var7 == 0x100 {
                if param_1 != DAT_100b0710 && DAT_100aa1ac == 0 {
                    if b_var2 && *d_10123130.offset(param_1 as isize) == param_2 {
                        b_var2 = true;
                    } else {
                        b_var2 = false;
                    }
                }
                local_14 = 0;
                while local_14 < *cnt_p as i32 {
                    let e = off(pc_var6, local_14.wrapping_mul(2).wrapping_add(0xef));
                    if DAT_100aa1ac == 0 {
                        if b_var2 && FUN_1004db2b(e, param_1) != 0 {
                            b_var2 = true;
                        } else {
                            b_var2 = false;
                        }
                    } else if b_var2 && FUN_1004ddbe(e, param_1) != 0 {
                        b_var2 = true;
                    } else {
                        b_var2 = false;
                    }
                    local_14 = local_14.wrapping_add(1);
                }
            }
        } else if i_var7 < 0x801 {
        } else if i_var7 < 0x2001 {
            if i_var7 == 0x2000 {
                local_14 = 0;
                while local_14 < *cnt_p as i32 {
                    let e = off(pc_var6, local_14.wrapping_mul(2).wrapping_add(0xef));
                    if DAT_100aa1ac == 0 {
                        if b_var2 && FUN_1004db2b(e, param_1) == 0 {
                            b_var2 = true;
                        } else {
                            b_var2 = false;
                        }
                    } else if b_var2 && FUN_1004ddbe(e, param_1) == 0 {
                        b_var2 = true;
                    } else {
                        b_var2 = false;
                    }
                    local_14 = local_14.wrapping_add(1);
                }
            }
        }

        let tmp = r32(off(d_101232e9, i_var5));
        let tmp2 = r32(off(d_101232e1, i_var5));
        let b_var1: bool = if tmp < 0 || tmp2 < 0 {
            false
        } else {
            tmp < (DAT_10123120 as u32).wrapping_sub(tmp2 as u32) as i32
        };

        i_var7 = r32(off(d_1012327c, i_var5));

        'end: {
            match i_var7 {
                4 => {
                    if b_var1 {
                        if b_var2 {
                            local_1c = 5;
                            FUN_1004e12e(param_1, param_2, 5);
                        } else {
                            local_1c = 6;
                        }
                    } else {
                        if !b_var2 {
                            if param_1 == DAT_100b0710 {
                                local_1c = 6;
                            } else {
                                local_1c = 8;
                            }
                            FUN_1004e12e(param_1, param_2, 6);
                        }
                        if b_var3 {
                            local_1c = 6;
                        }
                    }
                    break 'end;
                }
                0 | 0x200 | 0x400 | 0x800 => {
                    if b_var1 {
                        let idx = r32(off(d_101600e0, param_1.wrapping_mul(0x3c)));
                        let ptr = *d_100eef40.offset(idx as isize);
                        if ptr != 0 {
                            let us = (ptr as usize as *mut u8)
                                .byte_add(0x14)
                                .cast::<u16>()
                                .read_unaligned();
                            if (us & 2) != 0 && DAT_100aa1ac != 0 {
                                return;
                            }
                        }
                        local_1c = 5;
                        FUN_1004e12e(param_1, param_2, 5);
                    }
                    break 'end;
                }
                0x10000 => {
                    let s1 = r16(off(d_10123323, i_var5));
                    if (s1 as i32) <= DAT_10123230 {
                        let iv = (s1 as i32).wrapping_mul(0x3c0a);
                        *off(d_10123278, iv) = 2;
                        FUN_1004e457(s1 as i32, *off(d_10123278, iv) as u32);
                        w32(off(d_10123248, iv), DAT_10123120);
                        local_1c = 5;
                        FUN_1004e12e(param_1, param_2, 5);
                    }
                    break 'end;
                }
                0x20000 => {
                    let s1 = r16(off(d_10123323, i_var5));
                    if (s1 as i32) <= DAT_10123230 {
                        let iv = (s1 as i32).wrapping_mul(0x3c0a);
                        *off(d_10123278, iv) = 3;
                        FUN_1004e457(s1 as i32, *off(d_10123278, iv) as u32);
                        w32(off(d_10123248, iv), DAT_10123120);
                        local_1c = 5;
                        FUN_1004e12e(param_1, param_2, 5);
                    }
                    break 'end;
                }
                0x40000 => {
                    let s1 = r16(off(d_10123323, i_var5));
                    if (s1 as i32) <= DAT_10123230 {
                        let s2 = r16(off(d_10123325, i_var5));
                        let t = r32(off(d_10123240, (s1 as i32).wrapping_mul(0x3c0a)));
                        if (s2 as i32) <= t {
                            let iv = (s1 as i32)
                                .wrapping_mul(0x3c0a)
                                .wrapping_add((s2 as i32).wrapping_mul(0x13f));
                            let p = off(d_101232ee, iv);
                            if *p == 0 {
                                *p = 1;
                            } else {
                                *p = 0;
                            }
                            local_1c = 5;
                            FUN_1004e12e(param_1, param_2, 5);
                        }
                    }
                    break 'end;
                }
                0x80000 | 0x100000 => {
                    let new_state: u8 = if i_var7 == 0x100000 { 5 } else { 6 };
                    let s1 = r16(off(d_10123323, i_var5));
                    if (s1 as i32) <= DAT_10123230 {
                        let s2 = r16(off(d_10123325, i_var5));
                        let t = r32(off(d_10123240, (s1 as i32).wrapping_mul(0x3c0a)));
                        if (s2 as i32) <= t {
                            let iv = (s1 as i32)
                                .wrapping_mul(0x3c0a)
                                .wrapping_add((s2 as i32).wrapping_mul(0x13f));
                            let pb_var8 = off(d_1012327a, iv);
                            if *pb_var8 != 6 && *pb_var8 != 5 {
                                *pb_var8 = new_state;
                                w32(off(d_101232e5, iv), DAT_10123120);
                                let s1b = r16(off(d_10123323, i_var5));
                                let s2b = r16(off(d_10123325, i_var5));
                                FUN_1004e12e(s1b as i32, s2b as i32, *pb_var8 as u32);
                                local_1c = 5;
                                FUN_1004e12e(param_1, param_2, 5);
                            }
                        }
                    }
                    break 'end;
                }
                0x200000 => {
                    let s1 = r16(off(d_10123323, i_var5));
                    if (s1 as i32) <= DAT_10123230 {
                        let iv = (s1 as i32).wrapping_mul(0x3c0a);
                        let pi_var9 = off(d_10123240, iv);
                        let mut local_38: i32 = 0;
                        loop {
                            let t = r32(pi_var9);
                            if !(local_38 < t) {
                                break;
                            }
                            let o = local_38.wrapping_mul(0x13f);
                            *off(pi_var9, o.wrapping_add(0x3a)) = 3;
                            w32(off(pi_var9, o.wrapping_add(0xa1)), -1);
                            w32(off(pi_var9, o.wrapping_add(0xa5)), -1);
                            let t2 = r32(off(pi_var9, o.wrapping_add(0x3c)));
                            if t2 != 0x10 {
                                *d_10123170.offset(local_38 as isize) = 0;
                            }
                            local_38 = local_38.wrapping_add(1);
                        }
                        *off(d_10123278, iv) = 0;
                        w32(off(d_10123248, iv), -1);
                        local_1c = 5;
                        FUN_1004e12e(param_1, param_2, 5);
                    }
                    break 'end;
                }
                0x400000 => {
                    let s1 = r16(off(d_10123323, i_var5));
                    if (s1 as i32) <= DAT_10123230 {
                        let s2 = r16(off(d_10123325, i_var5));
                        let t = r32(off(d_10123240, (s1 as i32).wrapping_mul(0x3c0a)));
                        if (s2 as i32) <= t {
                            let iv = (s1 as i32)
                                .wrapping_mul(0x3c0a)
                                .wrapping_add((s2 as i32).wrapping_mul(0x13f));
                            *off(d_1012327a, iv) = 3;
                            w32(off(d_101232e1, iv), -1);
                            w32(off(d_101232e5, iv), -1);
                            let t2 = r32(off(d_1012327c, iv));
                            if t2 != 0x10 {
                                *d_10123170.offset(local_14 as isize) = 0;
                            }
                            local_1c = 5;
                            FUN_1004e12e(param_1, param_2, 5);
                        }
                    }
                    break 'end;
                }
                _ => {}
            }

            if b_var2 {
                local_1c = 5;
                FUN_1004e12e(param_1, param_2, 5);
            } else if b_var1 {
                local_1c = 6;
                FUN_1004e12e(param_1, param_2, 6);
            }
        }

        *pc_var6 = local_1c as u8;
        if local_1c != 3 {
            w32(off(d_101232e5, i_var5), DAT_10123120);
        }
    }
}
