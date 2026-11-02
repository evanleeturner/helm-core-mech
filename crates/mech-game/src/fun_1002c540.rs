// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aafa4: u32;
    static mut DAT_100aafa8: u32;
    static mut DAT_100aafb0: u32;
    static mut DAT_100aafc0: u32;
    static mut DAT_100ad6b4: u32;
    static mut DAT_10115100: [u32; 1];
    static mut DAT_10115160: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007d8c1(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
    fn FUN_1007d40c(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u8;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

unsafe fn rec(param_1: i32) -> u32 {
    // SAFETY: param_1 points at the caller's object; offset 0x20 holds a pointer.
    unsafe {
        mech_trace::rd!(
            "1002c540",
            (param_1.wrapping_add(0x20) as *const u32),
            read_unaligned
        )
    }
}

unsafe fn rec_u16(param_1: i32) -> u32 {
    // SAFETY: the record pointer at +0x20 points at a readable word.
    unsafe { mech_trace::rd!("1002c540", (rec(param_1) as *const u16), read_unaligned) as u32 }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002c540(
    param_1: i32,
    param_2: i32,
    param_3: u32,
    param_4: i32,
) -> u32 {
    mech_trace::enter!("1002c540", param_1, param_2, param_3, param_4);
    let mut param_3 = param_3;
    // SAFETY: globals and callees are the game's own; pointers come from the
    // caller's structures and are accessed where the original accesses them.
    unsafe {
        let mut local_14: u32 = 0;
        let mut local_18: u32 = param_3 & 0xf000;
        let mut u_var4: u32 = mech_trace::rd!(
            "1002c540",
            ((rec(param_1).wrapping_add(2)) as *const u16),
            read_unaligned
        ) as u32;
        let mut local_1c: u32 = 0;
        let local_10: u32;

        if mech_trace::gr!("1002c540", "100aafa4", DAT_100aafa4) != 0 {
            if mech_trace::gr!("1002c540", "100aafa8", DAT_100aafa8) == 1 {
                let v = mech_trace::rd!(
                    "1002c540",
                    ((rec(param_1).wrapping_add(0x24)) as *const u32),
                    read_unaligned
                );
                return mech_trace::rt!(
                    "1002c540",
                    match v {
                        0 => 0xd,
                        1 => 3,
                        2 => 6,
                        3 => 2,
                        5 => 8,
                        6 => 7,
                        7 => 0xf,
                        _ => 0xff,
                    }
                );
            }
            if mech_trace::gr!("1002c540", "100aafa8", DAT_100aafa8) == 2 {
                u_var4 = rec_u16(param_1) & 0x10f;
                if u_var4 < 0x101 {
                    if u_var4 == 0x100 {
                        return mech_trace::rt!("1002c540", 0xf);
                    }
                    match u_var4 {
                        0 => return mech_trace::rt!("1002c540", 0xd),
                        1 => return mech_trace::rt!("1002c540", 6),
                        2 => return mech_trace::rt!("1002c540", 8),
                        4 => return mech_trace::rt!("1002c540", 1),
                        _ => {}
                    }
                } else {
                    if u_var4 == 0x101 {
                        return mech_trace::rt!("1002c540", 7);
                    }
                    if u_var4 == 0x102 {
                        return mech_trace::rt!("1002c540", 0xb);
                    }
                    if u_var4 == 0x104 {
                        return mech_trace::rt!("1002c540", 3);
                    }
                }
                return mech_trace::rt!("1002c540", 0xff);
            }
            if mech_trace::gr!("1002c540", "100aafa4", DAT_100aafa4) == 1 {
                if local_18 == 0x8000 {
                    return mech_trace::rt!("1002c540", param_3);
                }
                if local_18 == 0x9000 {
                    return mech_trace::rt!("1002c540", param_3);
                }
                if (u_var4 & 0x200) != 0 {
                    return mech_trace::rt!("1002c540", 7);
                }
                if (u_var4 & 0x400) != 0 {
                    if local_18 != 0x1000 && local_18 != 0 {
                        return mech_trace::rt!("1002c540", param_3);
                    }
                    return mech_trace::rt!("1002c540", 0xb);
                }
                if (u_var4 & 0x100) == 0 {
                    if (u_var4 & 0x70) == 0 && param_3 != 0 {
                        return mech_trace::rt!("1002c540", 8);
                    }
                    return mech_trace::rt!("1002c540", 0);
                }
                let u_var5 = (rec_u16(param_1) & 0xf0) >> 4;
                if u_var5 == 0 {
                    return mech_trace::rt!("1002c540", 7);
                }
                if u_var5 < 0xc {
                    return mech_trace::rt!("1002c540", 3);
                }
                return mech_trace::rt!("1002c540", 0xb);
            }
            if mech_trace::gr!("1002c540", "100aafa4", DAT_100aafa4) == 3 {
                if (u_var4 & 0x200) != 0 {
                    return mech_trace::rt!("1002c540", 7);
                }
                if (u_var4 & 0x400) != 0 {
                    return mech_trace::rt!("1002c540", 0xb);
                }
                if (u_var4 & 0x100) != 0 {
                    let u_var5 = (rec_u16(param_1) & 0xf0) >> 4;
                    if u_var5 == 0 {
                        param_3 = 4;
                    } else if u_var5 < 0xc {
                        param_3 = 0;
                    } else {
                        param_3 = 8;
                    }
                    let i_var2 = mech_trace::cr!(
                        "1002c540",
                        "1007d8c1",
                        FUN_1007d8c1(param_1, param_2, 0xff, param_4)
                    );
                    let q = (i_var2.wrapping_mul(0x30) / 0xf) >> 4;
                    return mech_trace::rt!("1002c540", param_3.wrapping_add(q as u32) | 0x1000);
                }
                return mech_trace::rt!("1002c540", 8);
            }
        }

        let mut to_9c0 = false;
        let mut skip_default = false;
        if local_18 < 0x2001 {
            if local_18 == 0x2000 {
                to_9c0 = true;
            } else if local_18 == 0 {
                local_1c = (param_3 & 0xf0) >> 4;
                if mech_trace::gr!("1002c540", "100aafb0", DAT_100aafb0) == 0 {
                    local_10 = (param_3 & 0xf00) >> 4;
                } else {
                    local_1c = ((local_1c as i32) >> 2) as u32;
                    local_10 = 0xf0;
                }
                return mech_trace::rt!("1002c540", local_10 | local_1c);
            }
        } else if local_18 < 0x5001 {
            if local_18 == 0x5000 {
                to_9c0 = true;
            } else if local_18 == 0x3000 {
                if mech_trace::gr!("1002c540", "100aafc0", DAT_100aafc0) == 0 {
                    return mech_trace::rt!("1002c540", param_3);
                }
                if (u_var4 & mech_trace::gr!("1002c540", "100aafc0", DAT_100aafc0)) == 0 {
                    return mech_trace::rt!("1002c540", param_3);
                }
                local_18 = 0x1000;
                local_1c = param_3 & 0xf0;
                skip_default = true;
            }
        } else if local_18 < 0x7001 {
            if local_18 == 0x7000 || local_18 == 0x6000 {
                to_9c0 = true;
            }
        } else {
            if local_18 == 0x8000 {
                return mech_trace::rt!("1002c540", param_3);
            }
            if local_18 == 0x9000 {
                return mech_trace::rt!("1002c540", param_3);
            }
        }

        if to_9c0 {
            local_1c = 0xff;
            if mech_trace::gr!("1002c540", "100aafc0", DAT_100aafc0) == 0 {
                local_14 = param_3 & 0xff;
            } else if (mech_trace::gr!("1002c540", "100aafc0", DAT_100aafc0) & 0x100) == 0
                || ((u_var4 & 0x100) == 0 && ((u_var4 as u8) & 0xf0) != 0x50)
            {
                if (u_var4 & mech_trace::gr!("1002c540", "100aafc0", DAT_100aafc0)) == 0 {
                    local_14 = param_3 & 0xff;
                } else {
                    local_18 = 0x1000;
                    local_1c = 0xd0;
                }
            } else {
                local_18 = 0x1000;
                local_1c = 0xa0;
            }
            skip_default = true;
        }

        if !skip_default {
            local_1c = param_3 & 0xff;
            if (u_var4 & 0x400) != 0 {
                return mech_trace::rt!("1002c540", param_3);
            }
        }

        let mut local_8: u32;
        if (u_var4 & 0x400) == 0 {
            local_8 = mech_trace::cr!(
                "1002c540",
                "1007d8c1",
                FUN_1007d8c1(param_1, param_2, local_1c as i32, param_4)
            ) as u32;
            mech_trace::cr!(
                "1002c540",
                "1007d40c",
                FUN_1007d40c(param_1, param_2, 0x10115100, local_1c as i32)
            );
        } else {
            local_8 = 0xf;
        }

        if mech_trace::gr!("1002c540", "100aafa4", DAT_100aafa4) != 3
            && ((u_var4 & 0x100) != 0 || ((u_var4 as u8) & 0xf0) == 0x50)
        {
            let u_var5 = (rec_u16(param_1) & 0xf0) >> 4;
            if u_var5 != 0 {
                let arr = (&raw mut DAT_10115100).cast::<u32>();
                if mech_trace::gr!("1002c540", "100ad6b4", DAT_100ad6b4) == 0 {
                    let i_var2 = (0x10u32.wrapping_sub(u_var5).wrapping_mul(0x1000)) as i32;
                    local_8 = mech_trace::cr!(
                        "1002c540",
                        "10003d60",
                        FUN_10003d60(i_var2, local_8 as i32)
                    ) as u32;
                    local_8 &= 0xf;
                    let mut local_24: i32 = 0;
                    while local_24 < mech_trace::gr!("1002c540", "10115160", DAT_10115160) {
                        let i_var3 =
                            mech_trace::rd!("1002c540", arr.add(local_24 as usize), read) as i32;
                        let p = i_var3.wrapping_add(0x2c) as *mut i32;
                        let r = mech_trace::cr!(
                            "1002c540",
                            "10003d60",
                            FUN_10003d60(i_var2, mech_trace::rd!("1002c540", p, read_unaligned))
                        ) as u32;
                        mech_trace::wr!("1002c540", p, write_unaligned, (r & 0xf) as i32);
                        local_24 = local_24.wrapping_add(1);
                    }
                } else {
                    let i_var2 = mech_trace::cr!(
                        "1002c540",
                        "10002570",
                        FUN_10002570(0xfu32.wrapping_sub(local_8), 0xf)
                    ) as i32;
                    let i_var2 = mech_trace::cr!(
                        "1002c540",
                        "10003d60",
                        FUN_10003d60(u_var5 as i32, i_var2)
                    );
                    local_8 = local_8.wrapping_add(i_var2 as u32);
                    let mut local_24: i32 = 0;
                    while local_24 < mech_trace::gr!("1002c540", "10115160", DAT_10115160) {
                        let i_var2 =
                            mech_trace::rd!("1002c540", arr.add(local_24 as usize), read) as i32;
                        let p = i_var2.wrapping_add(0x2c) as *mut i32;
                        let i_var3 = mech_trace::cr!(
                            "1002c540",
                            "10002570",
                            FUN_10002570(
                                0xfu32.wrapping_sub(
                                    mech_trace::rd!("1002c540", p, read_unaligned) as u32
                                ),
                                0xf
                            )
                        ) as i32;
                        let i_var3 = mech_trace::cr!(
                            "1002c540",
                            "10003d60",
                            FUN_10003d60(u_var5 as i32, i_var3)
                        );
                        mech_trace::wr!(
                            "1002c540",
                            p,
                            write_unaligned,
                            mech_trace::rd!("1002c540", p, read_unaligned).wrapping_add(i_var3)
                        );
                        local_24 = local_24.wrapping_add(1);
                    }
                }
            }
        }

        if local_18 == 0x7000 || local_18 == 0x5000 || local_18 == 0x6000 || local_18 == 0x2000 {
            local_8 = local_8.wrapping_shl(8);
            local_10 = 0;
        } else if mech_trace::gr!("1002c540", "100aafb0", DAT_100aafb0) == 0 {
            local_10 = (param_3 & 0xf00) >> 4;
        } else {
            local_10 = 0xf0;
        }
        mech_trace::rt!("1002c540", local_8 | local_10 | local_14 | local_18)
    }
}
