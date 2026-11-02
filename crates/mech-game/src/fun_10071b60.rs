// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ad6c4: [u8; 8];
    static mut DAT_100b4e6c: [i8; 1];
    static mut DAT_100adac0: i32;
    static mut DAT_100adf74: *mut c_void;
    static mut _DAT_100b4e58: i32;
    static mut DAT_100b4e60: i32;
    static mut DAT_100b4e64: i32;
    static mut DAT_100ecbd0: [i32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100814bb(param_1: i32, param_2: i32) -> *mut i32;
    fn FUN_1005d8e0() -> i32;
    fn FUN_1005db1f(param_1: i32) -> *mut i32;
    fn strchr(s: *const i8, c: i32) -> *mut i8;
    fn sscanf(s: *const i8, fmt: *const i8, ...) -> i32;
    fn atoi(s: *const i8) -> i32;
    fn FUN_10032c10(param_1: i32) -> i32;
    fn FUN_1000c859(param_1: i32) -> i32;
    fn FUN_10050a68(param_1: i32) -> i32;
    fn FUN_1000c6d4(param_1: i32) -> i32;
    fn FUN_1001d5cb(param_1: i32) -> i32;
    fn FUN_1002f3a7(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_1002f6a8(param_1: *mut c_void, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_1002fbc0(param_1: *mut i32);
    fn FUN_10072715(param_1: i32, param_2: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(hheap: *mut c_void, flags: u32, mem: *mut c_void) -> i32;
}

unsafe fn ri(a: i32) -> i32 {
    // SAFETY: the address is a game object address supplied by the original logic.
    unsafe {
        mech_trace::rd!(
            "10071b60",
            (a as u32 as usize as *const i32),
            read_unaligned
        )
    }
}

unsafe fn wi(a: i32, v: i32) {
    // SAFETY: the address is a game object address supplied by the original logic.
    unsafe {
        mech_trace::wr!(
            "10071b60",
            (a as u32 as usize as *mut i32),
            write_unaligned,
            v
        )
    }
}

unsafe fn rb(a: i32) -> u8 {
    // SAFETY: the address is a game object address supplied by the original logic.
    unsafe { mech_trace::rd!("10071b60", (a as u32 as usize as *const u8), read) }
}

unsafe fn rc(a: i32) -> i32 {
    // SAFETY: the address is a game object address supplied by the original logic.
    unsafe { mech_trace::rd!("10071b60", (a as u32 as usize as *const i8), read) as i32 }
}

unsafe fn g(l: *mut i32, k: usize) -> i32 {
    // SAFETY: l points at the 11-int record.
    unsafe { mech_trace::rd!("10071b60", l.add(k), read_unaligned) }
}

unsafe fn s(l: *mut i32, k: usize, v: i32) {
    // SAFETY: l points at the 11-int record.
    unsafe { mech_trace::wr!("10071b60", l.add(k), write_unaligned, v) }
}

unsafe fn ea(l: *mut i32, idx: i32, off: i32) -> i32 {
    // SAFETY: table and entries are the game's own memory.
    unsafe {
        ri(g(l, 0).wrapping_add(0x10))
            .wrapping_add(idx.wrapping_mul(8))
            .wrapping_add(off)
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10071b60(param_1: i32, str: *mut i8, param_3: i32) -> i32 {
    mech_trace::enter!("10071b60", param_1, str, param_3);
    // SAFETY: all globals, callees and record/table/owner pointers are the
    // game's own memory, accessed exactly where the original does.
    unsafe {
        let mut local_5c: i32 = 0;
        let mut local_2c: i32 = 0;
        let mut local_4c: i32 = 0;
        let mut local_1c: u32 = 0;
        let mut local_30: i32;
        let mut local_40: i32 = 0;

        if param_1 == 0 {
            let heap_arg = mech_trace::rd!(
                "10071b60",
                ((&raw const DAT_100ad6c4).cast::<u8>().add(4) as *const i32),
                read_unaligned
            );
            let l = mech_trace::cr!("10071b60", "100814bb", FUN_100814bb(0x2c, heap_arg));
            if l.is_null() {
                return mech_trace::rt!("10071b60", 0);
            }
            if mech_trace::gr!("10071b60", "100adac0", DAT_100adac0) == 0 {
                mech_trace::gw!("10071b60", "100b4e58", _DAT_100b4e58, 1);
                mech_trace::cr!(
                    "10071b60",
                    "HeapFree",
                    HeapFree(
                        mech_trace::gr!("10071b60", "100adf74", DAT_100adf74),
                        1,
                        l as *mut c_void
                    )
                );
                return mech_trace::rt!("10071b60", 0);
            }
            let i3 = mech_trace::cr!("10071b60", "1005d8e0", FUN_1005d8e0());
            let pp = mech_trace::cr!("10071b60", "1005db1f", FUN_1005db1f(i3));
            mech_trace::wr!("10071b60", pp, write_unaligned, l as usize as i32);
            s(l, 2, 0);
            s(l, 0, 0);
            s(l, 8, 0);
            s(l, 4, 0);
            s(l, 10, 0);
            s(l, 3, mech_trace::gr!("10071b60", "100adac0", DAT_100adac0));
            let p14 = mech_trace::cr!("10071b60", "strchr", strchr(str, 0x3b));
            if !p14.is_null() {
                mech_trace::dw!("10071b60", p14, 0);
                let p14 = p14.add(1);
                mech_trace::cr!(
                    "10071b60",
                    "sscanf",
                    sscanf(
                        p14,
                        (&raw const DAT_100b4e6c).cast::<i8>(),
                        &raw mut local_5c,
                        &raw mut local_2c,
                        &raw mut local_4c,
                    )
                );
                local_4c =
                    local_4c.wrapping_add(mech_trace::gr!("10071b60", "100b4e64", DAT_100b4e64));
                if mech_trace::gr!("10071b60", "100b4e60", DAT_100b4e60) < local_4c {
                    mech_trace::gw!("10071b60", "100b4e60", DAT_100b4e60, local_4c);
                }
                if local_2c == 0 {
                    s(l, 8, 0);
                    mech_trace::gw!("10071b60", "100b4e58", _DAT_100b4e58, 2);
                } else {
                    s(l, 8, 1);
                    if 0x7f < local_2c {
                        s(l, 10, g(l, 10) | 1);
                    }
                }
                s(l, 4, local_5c);
                let tv = mech_trace::rd!(
                    "10071b60",
                    (&raw mut DAT_100ecbd0)
                        .cast::<i32>()
                        .wrapping_offset(local_4c as isize),
                    read_unaligned
                );
                s(l, 0, tv);
                let tv = mech_trace::rd!(
                    "10071b60",
                    (&raw mut DAT_100ecbd0)
                        .cast::<i32>()
                        .wrapping_offset(local_4c as isize),
                    read_unaligned
                );
                s(l, 1, tv);
                s(l, 9, -1);
                s(l, 5, 0);
                s(l, 6, 0);
                s(l, 7, param_3);
                wi(g(l, 3).wrapping_add(0x94), local_5c);
                wi(g(l, 3).wrapping_add(0x84), -1);
            }
            let i3 = mech_trace::cr!("10071b60", "atoi", atoi(str));
            let i3 = mech_trace::cr!("10071b60", "10032c10", FUN_10032c10(i3));
            let local_24 = mech_trace::cr!("10071b60", "1000c859", FUN_1000c859(i3));
            if local_24 == -1 {
                let local_58 = mech_trace::cr!("10071b60", "1000c6d4", FUN_1000c6d4(i3));
                if local_58 != 0 {
                    let u2 = mech_trace::cr!("10071b60", "1001d5cb", FUN_1001d5cb(local_58));
                    s(l, 2, u2);
                }
            } else {
                let u2 = mech_trace::cr!("10071b60", "10050a68", FUN_10050a68(local_24));
                s(l, 2, u2);
            }
            if g(l, 2) == 0 {
                mech_trace::gw!("10071b60", "100b4e58", _DAT_100b4e58, 3);
            }
        } else if param_1 == 1 {
            let i3 = mech_trace::cr!("10071b60", "1005d8e0", FUN_1005d8e0());
            let pp = mech_trace::cr!("10071b60", "1005db1f", FUN_1005db1f(i3));
            let l = mech_trace::rd!("10071b60", pp, read_unaligned) as u32 as usize as *mut i32;
            if l.is_null() {
                return mech_trace::rt!("10071b60", 0);
            }
            if g(l, 2) == 0 {
                return mech_trace::rt!("10071b60", 0);
            }
            if g(l, 3) == 0 {
                mech_trace::gw!("10071b60", "100b4e58", _DAT_100b4e58, 5);
                return mech_trace::rt!("10071b60", 0);
            }
            if g(l, 8) == 0 {
                mech_trace::gw!("10071b60", "100b4e58", _DAT_100b4e58, 4);
                return mech_trace::rt!("10071b60", 0);
            }
            let o = g(l, 3);
            let local_c = param_3.wrapping_sub(g(l, 7));
            s(l, 7, param_3);

            if rb(o.wrapping_add(0x80)) & 0x10 == 0 {
                if rb(o.wrapping_add(0x80)) & 1 == 0 {
                    if (g(l, 10) as u8 & 1) != 0
                        && (ri(o.wrapping_add(0x88)) != -1 || ri(o.wrapping_add(0x84)) != -1)
                    {
                        wi(o.wrapping_add(0x80), ri(o.wrapping_add(0x80)) | 1);
                        let i3 = mech_trace::cr!(
                            "10071b60",
                            "10072715",
                            FUN_10072715(ri(o.wrapping_add(0x8c)), g(l, 4))
                        );
                        wi(o.wrapping_add(0x94), i3);
                    }
                } else {
                    if g(l, 4) == 0 {
                        mech_trace::gw!("10071b60", "100b4e58", _DAT_100b4e58, 6);
                        return mech_trace::rt!("10071b60", 0);
                    }
                    if ri(o.wrapping_add(0x94)) == 0 {
                        mech_trace::gw!("10071b60", "100b4e58", _DAT_100b4e58, 7);
                        return mech_trace::rt!("10071b60", 0);
                    }
                    let mut local_34: i32;
                    if g(l, 5) < 1 {
                        local_34 = g(l, 6);
                    } else {
                        local_34 = g(l, 6).wrapping_mul(local_c) / g(l, 5);
                    }
                    if g(l, 6) < 0 {
                        if local_34 < g(l, 6) {
                            local_34 = g(l, 6);
                        }
                    } else if g(l, 6) < local_34 {
                        local_34 = g(l, 6);
                    }

                    if ri(g(l, 0).wrapping_add(8)) < 3 {
                        let mut a = 0;
                        let mut b = 0;
                        let mut c = 0;
                        let k = ri(g(l, 0).wrapping_add(8));
                        if k == 0 {
                            c = local_34;
                        } else if k == 1 {
                            b = local_34;
                        } else if k == 2 {
                            a = local_34;
                        }
                        FUN_1002f3a7(g(l, 2), c, b, a);
                    } else {
                        let mut a = 0;
                        let mut b = 0;
                        let mut c = 0;
                        let k = ri(g(l, 0).wrapping_add(8));
                        if k == 3 {
                            c = local_34;
                        } else if k == 4 {
                            b = local_34;
                        } else if k == 5 {
                            a = local_34;
                        }
                        FUN_1002f6a8(g(l, 2) as u32 as usize as *mut c_void, c, b, a, 0);
                    }
                    FUN_1002fbc0(g(l, 2) as u32 as usize as *mut i32);

                    s(l, 6, g(l, 6).wrapping_sub(local_34));
                    s(l, 5, g(l, 5).wrapping_sub(local_c));
                    if g(l, 9) == -1 {
                        s(l, 9, 0);
                        s(l, 5, 0);
                        wi(o.wrapping_add(0x84), 0);
                    }

                    if g(l, 5) < 1 {
                        let local_54 = g(l, 9);
                        s(l, 9, g(l, 9).wrapping_add(1));
                        s(l, 5, g(l, 5).wrapping_add(ri(o.wrapping_add(0x94))));
                        if g(l, 5) < 0i32.wrapping_sub(ri(o.wrapping_add(0x94))) {
                            s(l, 5, 0i32.wrapping_sub(ri(o.wrapping_add(0x94))));
                        }

                        if rb(ea(l, local_54, 0)) & 0x40 != 0 {
                            local_40 = rc(ea(l, local_54, 5));
                            if ri(o.wrapping_add(0x88)) == local_40 {
                                local_1c = 1;
                            } else {
                                local_1c = 0;
                            }
                        }
                        if rb(ea(l, local_54, 0)) & 0x80 != 0 {
                            local_40 = rc(ea(l, local_54, 5));
                            if ri(o.wrapping_add(0x88)) != -1
                                && ri(o.wrapping_add(0x84)) != ri(o.wrapping_add(0x88))
                            {
                                local_1c |= 1;
                            }
                        }
                        if rb(ea(l, local_54, 1)) & 1 != 0 {
                            local_40 = rc(ea(l, local_54, 6));
                            if ri(o.wrapping_add(0x88)) == -1 {
                                local_1c |= 1;
                            }
                        }

                        if local_1c != 0 {
                            local_1c = 0;
                            let local_3c: i32 = if ri(o.wrapping_add(0x84)) < local_40 {
                                1
                            } else {
                                -1
                            };
                            local_30 = 0;
                            let mut local_8 = local_54;
                            while local_30 == 0 {
                                local_8 = local_8.wrapping_add(local_3c);
                                if ri(g(l, 0).wrapping_add(4)) <= local_8 {
                                    local_8 = -1;
                                }
                                if local_8 == -1 {
                                    local_30 = 1;
                                } else if rb(ea(l, local_8, 0)) & 1 != 0 {
                                    let local_48 = rc(ea(l, local_8, 4));
                                    if local_48 == local_40 {
                                        local_30 = 1;
                                    }
                                }
                            }
                            if local_8 != -1 {
                                local_30 = 0;
                                while local_30 == 0 {
                                    if local_8 < ri(g(l, 0).wrapping_add(4)) {
                                        if rb(ea(l, local_8, 1)) & 2 != 0 {
                                            let local_48 = rc(ea(l, local_8, 7));
                                            if ri(o.wrapping_add(0x84)) == local_48 {
                                                local_30 = 1;
                                            }
                                        }
                                        local_8 = local_8.wrapping_add(1);
                                    } else {
                                        local_8 = -1;
                                        local_30 = 1;
                                    }
                                }
                            }
                            if local_8 != -1 {
                                s(l, 9, local_8);
                                local_1c = 1;
                            }
                        }

                        if local_1c == 0 && (rb(ea(l, local_54, 0)) & 0xc) != 0 {
                            let mut b4 = true;
                            if rb(ea(l, local_54, 0)) & 4 != 0 {
                                b4 = false;
                                let mut i = 0i32;
                                while i < 4 {
                                    if rc(ea(l, local_54, 4i32.wrapping_add(i))) == -1 {
                                        break;
                                    }
                                    b4 = (rc(ea(l, local_54, 4i32.wrapping_add(i))) as i8)
                                        == (rc(o.wrapping_add(0x88)) as i8);
                                    if b4 {
                                        break;
                                    }
                                    i += 1;
                                }
                            }
                            let mut i3 = local_54;
                            if b4 {
                                let mut local_28;
                                loop {
                                    local_28 = i3.wrapping_sub(1);
                                    if rb(ea(l, local_28, 0)) & 2 != 0 {
                                        break;
                                    }
                                    i3 = local_28;
                                    if !(-1 < local_28) {
                                        break;
                                    }
                                }
                                s(l, 9, local_28);
                            }
                        }

                        if (local_1c == 0 && (rb(ea(l, local_54, 1)) & 4) != 0)
                            || ri(g(l, 0).wrapping_add(4)) <= g(l, 9)
                        {
                            s(l, 5, 0);
                            s(l, 9, -1);
                        }
                        if g(l, 9) != -1 {
                            let v =
                                ri(ri(g(l, 0).wrapping_add(0xc))
                                    .wrapping_add(g(l, 9).wrapping_mul(4)));
                            s(l, 6, g(l, 6).wrapping_add(v));
                        }

                        if (g(l, 10) as u8 & 1) != 0 {
                            if local_1c != 0 {
                                wi(o.wrapping_add(0x84), local_40);
                            }
                            wi(o.wrapping_add(0x80), ri(o.wrapping_add(0x80)) & !2);
                            wi(o.wrapping_add(0x80), ri(o.wrapping_add(0x80)) & !8);
                            if g(l, 9) == -1 {
                                if ri(o.wrapping_add(0x88)) == -1 {
                                    wi(o.wrapping_add(0x80), ri(o.wrapping_add(0x80)) & !1);
                                    wi(o.wrapping_add(0x84), -1);
                                }
                            } else {
                                let i3 = mech_trace::cr!(
                                    "10071b60",
                                    "10072715",
                                    FUN_10072715(ri(o.wrapping_add(0x8c)), g(l, 4))
                                );
                                wi(o.wrapping_add(0x94), i3);
                                if rb(ea(l, g(l, 9), 0)) & 0x10 != 0 {
                                    wi(o.wrapping_add(0x80), ri(o.wrapping_add(0x80)) | 2);
                                }
                                if rb(ea(l, g(l, 9), 1)) & 8 != 0 {
                                    wi(o.wrapping_add(0x80), ri(o.wrapping_add(0x80)) | 8);
                                }
                            }
                        }
                    }
                }
            } else {
                s(l, 0, g(l, 1));
                s(l, 9, -1);
                s(l, 5, 0);
                s(l, 6, 0);
                s(l, 7, param_3);
                if (g(l, 10) as u8 & 1) != 0 {
                    wi(o.wrapping_add(0x94), g(l, 4));
                    wi(
                        o.wrapping_add(0x80),
                        ri(o.wrapping_add(0x80)) & 0xffffffeeu32 as i32,
                    );
                    wi(o.wrapping_add(0x84), -1);
                }
            }
        }
        mech_trace::rt!("10071b60", 1)
    }
}
