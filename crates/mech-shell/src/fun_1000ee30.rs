// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6724: i32;
    static mut DAT_1009821c: i32;
    static mut DAT_1008da60: [i32; 0x260];
    static mut DAT_1008da7c: i32;
    static mut DAT_100a672c: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn AIL_serve();
    fn FUN_1000ed70();
    fn timeGetTime() -> u32;
}

// `__cdecl`: smackw32's thunks (SmackDoFrame, SmackWait, SmackGoto,
// SmackNextFrame); the original pops their arguments after each call and
// each export ends on a bare `ret`.
#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006e74c(param_1: u32);
    fn FUN_1006e76a(param_1: u32) -> i32;
    fn FUN_1006e764(param_1: u32, param_2: u32);
    fn FUN_1006e75e(param_1: u32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000f290(param_1: i32);
    fn FUN_1000ece0(param_1: i32, param_2: u32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10012f10(this_: *mut u8, param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_10013380(this_: *mut u8, param_1: i32);
    fn FUN_10012ca0(
        this_: *mut u8,
        param_1: u32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    );
    fn FUN_10012d40(
        this_: *mut u8,
        param_1: u32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    );
    fn FUN_10013090(
        this_: *mut u8,
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
    fn FUN_10013100(
        this_: *mut u8,
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
}

#[inline(always)]
unsafe fn rd(p: *mut i32, i: usize) -> i32 {
    // SAFETY: p points at a record in the game's table; i is within the record.
    unsafe { mech_trace::rd!("1000ee30", p.add(i), read_unaligned) }
}

#[inline(always)]
unsafe fn wr(p: *mut i32, i: usize, v: i32) {
    // SAFETY: p points at a record in the game's table; i is within the record.
    unsafe { mech_trace::wr!("1000ee30", p.add(i), write_unaligned, v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000ee30() {
    mech_trace::enter!("1000ee30");
    // SAFETY: all globals and callees are the game's own; the record pointer
    // walks the game's 0x20-record table exactly as the original does.
    unsafe {
        if mech_trace::gr!("1000ee30", "100a6724", DAT_100a6724) != 0 {
            AIL_serve();
        }
        if mech_trace::gr!("1000ee30", "1009821c", DAT_1009821c) != 0 {
            return;
        }
        if (mech_trace::gr!("1000ee30", "1008da7c", DAT_1008da7c) as u32 & 0x80000000) != 0
            && (mech_trace::gr!("1000ee30", "1008da7c", DAT_1008da7c) as u32 & 0x1000) != 0
        {
            FUN_1000ed70();
            return;
        }
        let mut i_var5: i32 = 0;
        let mut p: *mut i32 = (&raw mut DAT_1008da60).cast::<i32>();
        loop {
            if (rd(p, 7) as u32 & 0x80000000) != 0 {
                if rd(p, 0) != 0 {
                    if rd(p, 0xf) == 0 && (rd(p, 7) as u32 & 1) == 0 {
                        FUN_1006e74c(rd(p, 0) as u32);
                        if (rd(p, 7) as u32 & 2) != 0 {
                            wr(p, 7, (rd(p, 7) as u32 | 0x10) as i32);
                        }
                        wr(p, 0xf, 1);
                    }
                    if rd(p, 0) != 0 && (rd(p, 7) as u32 & 2) == 0 {
                        wr(p, 0xe, rd(p, 0xf));
                    }
                }
                if rd(p, 8) != rd(p, 0xc)
                    || rd(p, 0xd) != rd(p, 9)
                    || rd(p, 0xe) != rd(p, 0xf)
                    || (rd(p, 7) as u32 & 0x40000020) != 0
                {
                    if (rd(p, 7) as u32 & 0x10) != 0 {
                        FUN_10012f10(
                            mech_trace::gr!("1000ee30", "100a672c", DAT_100a672c),
                            rd(p, 0xc),
                            rd(p, 0xd),
                            rd(p, 10),
                            rd(p, 0xb),
                        );
                    }
                    wr(p, 7, (rd(p, 7) as u32 | 0x100) as i32);
                }
                wr(p, 0xc, rd(p, 8));
                wr(p, 0xd, rd(p, 9));
                wr(p, 0xe, rd(p, 0xf));
                let u_var3 = rd(p, 7) as u32;
                wr(p, 7, (u_var3 & 0xffffffef) as i32);
                if (u_var3 & 0x40000000) != 0 {
                    FUN_1000f290(i_var5);
                }
            }
            i_var5 = i_var5.wrapping_add(1);
            p = p.add(0x13);
            if i_var5 >= 0x20 {
                break;
            }
        }
        p = (&raw mut DAT_1008da60).cast::<i32>();
        FUN_10013380(mech_trace::gr!("1000ee30", "100a672c", DAT_100a672c), 0);
        let mut n: i32 = 0x20;
        loop {
            let u_var3 = rd(p, 7) as u32;
            if (u_var3 & 0x80000000) != 0 {
                if rd(p, 0) == 0 {
                    if rd(p, 5) != 0 {
                        if (u_var3 & 0x20) == 0 {
                            wr(p, 7, (u_var3 | 0x10) as i32);
                            if (u_var3 & 0x100) == 0 {
                                FUN_10013100(
                                    mech_trace::gr!("1000ee30", "100a672c", DAT_100a672c),
                                    rd(p, 5),
                                    rd(p, 0xf),
                                    rd(p, 8),
                                    rd(p, 9),
                                    rd(p, 10),
                                    rd(p, 0xb),
                                );
                            } else {
                                FUN_10013090(
                                    mech_trace::gr!("1000ee30", "100a672c", DAT_100a672c),
                                    rd(p, 5),
                                    rd(p, 0xf),
                                    rd(p, 8),
                                    rd(p, 9),
                                    rd(p, 10),
                                    rd(p, 0xb),
                                );
                            }
                            wr(p, 7, (rd(p, 7) as u32 & 0xfffffeff) as i32);
                        }
                        if (rd(p, 7) as u32 & 1) == 0 {
                            let d_var4 = mech_trace::cr!("1000ee30", "timeGetTime", timeGetTime());
                            let i_var1 = mech_trace::cr!(
                                "1000ee30",
                                "1000ece0",
                                FUN_1000ece0(p as i32, d_var4)
                            );
                            if i_var1 == 0 {
                                let i_var1 = rd(p, 0xf);
                                let i_var2 = i_var1.wrapping_add(1);
                                wr(p, 0xf, i_var2);
                                if rd(p, 0x10) <= i_var2 {
                                    wr(p, 0xf, i_var1);
                                    let mut u = rd(p, 7) as u32;
                                    if (u & 4) == 0 {
                                        if (u & 8) != 0 {
                                            wr(p, 0xf, 0);
                                        } else {
                                            u |= 0x40000001;
                                            wr(p, 7, u as i32);
                                        }
                                    } else {
                                        u |= 1;
                                        wr(p, 7, u as i32);
                                    }
                                }
                                let u = rd(p, 7) as u32 | 0x100;
                                wr(p, 7, u as i32);
                            }
                        }
                    }
                } else {
                    if rd(p, 6) != 0 && (u_var3 & 0x20) == 0 {
                        wr(p, 7, (u_var3 | 0x10) as i32);
                        if (u_var3 & 0x100) == 0 {
                            FUN_10012d40(
                                mech_trace::gr!("1000ee30", "100a672c", DAT_100a672c),
                                rd(p, 6) as u32,
                                rd(p, 8),
                                rd(p, 9),
                                rd(p, 10),
                                rd(p, 0xb),
                            );
                        } else {
                            FUN_10012ca0(
                                mech_trace::gr!("1000ee30", "100a672c", DAT_100a672c),
                                rd(p, 6) as u32,
                                rd(p, 8),
                                rd(p, 9),
                                rd(p, 10),
                                rd(p, 0xb),
                            );
                        }
                        wr(p, 7, (rd(p, 7) as u32 & 0xfffffeff) as i32);
                    }
                    if (rd(p, 7) as u32 & 1) == 0 {
                        let i_var1 =
                            mech_trace::cr!("1000ee30", "1006e76a", FUN_1006e76a(rd(p, 0) as u32));
                        if i_var1 == 0 {
                            let i_var1 = rd(p, 0xf);
                            let i_var2 = i_var1.wrapping_add(1);
                            wr(p, 0xf, i_var2);
                            let mut go030 = false;
                            if rd(p, 0x10) < i_var2 {
                                wr(p, 0xf, i_var1);
                                let mut u = rd(p, 7) as u32;
                                if (u & 4) == 0 {
                                    if (u & 8) != 0 {
                                        wr(p, 0xf, 1);
                                        FUN_1006e764(rd(p, 0) as u32, 1);
                                        go030 = true;
                                    } else {
                                        u |= 0x40000001;
                                        wr(p, 7, u as i32);
                                    }
                                } else {
                                    u |= 1;
                                    wr(p, 7, u as i32);
                                }
                            } else {
                                if i_var2 != 1 {
                                    FUN_1006e75e(rd(p, 0) as u32);
                                }
                                go030 = true;
                            }
                            if go030 {
                                FUN_1006e74c(rd(p, 0) as u32);
                                let u = rd(p, 7) as u32;
                                wr(p, 7, (u | 0x100) as i32);
                                if (u & 2) != 0 {
                                    wr(p, 7, (u | 0x110) as i32);
                                }
                            }
                        }
                    }
                }
            }
            p = p.add(0x13);
            n -= 1;
            if n == 0 {
                break;
            }
        }
        FUN_10013380(mech_trace::gr!("1000ee30", "100a672c", DAT_100a672c), 1);
    }
}
