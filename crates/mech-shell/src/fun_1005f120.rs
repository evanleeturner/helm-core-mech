// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6728: i32;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1005f8c0(this_: *mut u8, param_1: u32) -> bool;
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
    fn FUN_1005f760(this_: *mut u8, param_1: u32) -> u32;
}

// `__cdecl`: smackw32's thunks (SmackWait, SmackGoto, SmackNextFrame,
// SmackDoFrame); the original pops their arguments after each call and
// each export ends on a bare `ret`.
#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006e76a(param_1: u32) -> i32;
    fn FUN_1006e764(param_1: u32, param_2: u32);
    fn FUN_1006e75e(param_1: u32);
    fn FUN_1006e74c(param_1: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

#[inline(always)]
unsafe fn rd(p: *mut u8, off: usize) -> u32 {
    // SAFETY: p points at the game object; the field lies within it.
    unsafe { mech_trace::rd!("1005f120", p.byte_add(off).cast::<u32>(), read_unaligned) }
}

#[inline(always)]
unsafe fn wr(p: *mut u8, off: usize, v: u32) {
    // SAFETY: p points at the game object; the field lies within it.
    unsafe {
        mech_trace::wr!(
            "1005f120",
            p.byte_add(off).cast::<u32>(),
            write_unaligned,
            v
        )
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_1005f120(param_1: *mut u8) {
    mech_trace::enter!("1005f120", param_1);
    let p = param_1;
    // SAFETY: the object, the callees and the globals are the game's own;
    // reads and writes happen at the original's offsets and order.
    unsafe {
        if mech_trace::cr!("1005f120", "1005f8c0", FUN_1005f8c0(p, 0x40000000)) {
            return;
        }
        if mech_trace::cr!("1005f120", "1005f8c0", FUN_1005f8c0(p, 0x20000000)) {
            let base = mech_trace::gr!("1005f120", "100a6728", DAT_100a6728);
            let t = mech_trace::rd!(
                "1005f120",
                (base.wrapping_add(0x2b) as usize as *const u32),
                read_unaligned
            );
            wr(p, 0x110, t);
            let t = mech_trace::rd!(
                "1005f120",
                (base.wrapping_add(0x2f) as usize as *const u32),
                read_unaligned
            );
            wr(p, 0x114, t);
        }

        if rd(p, 0x100) != 0 {
            if rd(p, 0x108) != 0 && !mech_trace::cr!("1005f120", "1005f8c0", FUN_1005f8c0(p, 0x20))
            {
                wr(p, 0x10c, rd(p, 0x10c) | 0x10);
                if mech_trace::cr!("1005f120", "1005f8c0", FUN_1005f8c0(p, 0x100)) {
                    FUN_10012ca0(
                        rd(p, 0x140) as usize as *mut u8,
                        rd(p, 0x108),
                        rd(p, 0x110) as i32,
                        rd(p, 0x114) as i32,
                        rd(p, 0x118) as i32,
                        rd(p, 0x11c) as i32,
                    );
                } else {
                    FUN_10012d40(
                        rd(p, 0x140) as usize as *mut u8,
                        rd(p, 0x108),
                        rd(p, 0x110) as i32,
                        rd(p, 0x114) as i32,
                        rd(p, 0x118) as i32,
                        rd(p, 0x11c) as i32,
                    );
                }
                wr(p, 0x10c, rd(p, 0x10c) & 0xfffffeff);
            }
            if mech_trace::cr!("1005f120", "1005f8c0", FUN_1005f8c0(p, 1)) {
                return;
            }
            if mech_trace::cr!("1005f120", "1006e76a", FUN_1006e76a(rd(p, 0x100))) != 0 {
                return;
            }
            let i_var2 = rd(p, 300) as i32;
            let i_var3 = i_var2.wrapping_add(1);
            wr(p, 300, i_var3 as u32);
            if (rd(p, 0x130) as i32) < i_var3 {
                wr(p, 300, i_var2 as u32);
                if mech_trace::cr!("1005f120", "1005f8c0", FUN_1005f8c0(p, 0x200)) {
                    wr(p, 0x10c, rd(p, 0x10c) | 1);
                    wr(p, 300, 1);
                    FUN_1006e764(rd(p, 0x100), 1);
                } else {
                    if mech_trace::cr!("1005f120", "1005f8c0", FUN_1005f8c0(p, 4)) {
                        wr(p, 0x10c, rd(p, 0x10c) | 1);
                        return;
                    }
                    if !mech_trace::cr!("1005f120", "1005f8c0", FUN_1005f8c0(p, 8)) {
                        wr(p, 0x10c, rd(p, 0x10c) | 0x40000001);
                        return;
                    }
                    wr(p, 300, 1);
                    FUN_1006e764(rd(p, 0x100), 1);
                }
            } else if i_var3 != 1 {
                FUN_1006e75e(rd(p, 0x100));
            }
            FUN_1006e74c(rd(p, 0x100));
            wr(p, 0x10c, rd(p, 0x10c) | 0x100);
            if !mech_trace::cr!("1005f120", "1005f8c0", FUN_1005f8c0(p, 2)) {
                return;
            }
            wr(p, 0x10c, rd(p, 0x10c) | 0x10);
            return;
        }

        if rd(p, 0x104) == 0 {
            return;
        }
        if !mech_trace::cr!("1005f120", "1005f8c0", FUN_1005f8c0(p, 0x20)) {
            wr(p, 0x10c, rd(p, 0x10c) | 0x10);
            if mech_trace::cr!("1005f120", "1005f8c0", FUN_1005f8c0(p, 0x100)) {
                FUN_10013090(
                    rd(p, 0x140) as usize as *mut u8,
                    rd(p, 0x104) as i32,
                    rd(p, 300) as i32,
                    rd(p, 0x110) as i32,
                    rd(p, 0x114) as i32,
                    rd(p, 0x118) as i32,
                    rd(p, 0x11c) as i32,
                );
            } else {
                FUN_10013100(
                    rd(p, 0x140) as usize as *mut u8,
                    rd(p, 0x104) as i32,
                    rd(p, 300) as i32,
                    rd(p, 0x110) as i32,
                    rd(p, 0x114) as i32,
                    rd(p, 0x118) as i32,
                    rd(p, 0x11c) as i32,
                );
            }
            wr(p, 0x10c, rd(p, 0x10c) & 0xfffffeff);
        }
        if mech_trace::cr!("1005f120", "1005f8c0", FUN_1005f8c0(p, 1)) {
            return;
        }
        let d_var4 = mech_trace::cr!("1005f120", "timeGetTime", timeGetTime());
        let u_var5 = mech_trace::cr!("1005f120", "1005f760", FUN_1005f760(p, d_var4));
        if (u_var5 as u8) != 0 {
            return;
        }
        let i_var2 = rd(p, 300) as i32;
        let i_var3 = i_var2.wrapping_add(1);
        wr(p, 300, i_var3 as u32);
        if (rd(p, 0x130) as i32) <= i_var3 {
            wr(p, 300, i_var2 as u32);
            if rd(p, 0x10c) & 4 == 0 {
                if mech_trace::cr!("1005f120", "1005f8c0", FUN_1005f8c0(p, 8)) {
                    wr(p, 300, 0);
                    wr(p, 0x10c, rd(p, 0x10c) | 0x100);
                    return;
                }
                let u_var6 = rd(p, 0x10c) | 0x40000001;
                wr(p, 0x10c, u_var6);
            } else {
                let u_var6 = rd(p, 0x10c) | 1;
                wr(p, 0x10c, u_var6);
            }
        }
        wr(p, 0x10c, rd(p, 0x10c) | 0x100);
    }
}
