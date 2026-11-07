// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a410c: i32;
    static mut DAT_100aa1b4: u16;
    static mut DAT_100aa1f4: u32;
    static mut DAT_100aa210: u32;
    static mut DAT_100adf38: u32;
    static mut DAT_100b38fc: u32;
    static mut DAT_100eef40: u32;
    static mut DAT_100ef030: u8;
    static mut DAT_100ef032: u8;
    static mut DAT_101eb4f0: u8;
    static mut DAT_101eb520: u8;
    static mut DAT_100aa264: i8;
    static mut DAT_100aa28c: i8;
    static mut DAT_100aa274: i8;
    static mut DAT_100aa29c: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10011e80(param_1: i32) -> i32;
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32) -> u8;
    fn FUN_1002f668(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: u32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
    fn FUN_100300e2(param_1: i32, param_2: i32, param_3: u32) -> u8;
    fn FUN_10005207(param_1: i32, param_2: *mut i32, param_3: u32) -> u8;
    fn FUN_10004b19(param_1: i32, param_2: *mut i32) -> u8;
    fn _sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1005be24(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32) -> u32;
    fn FUN_10071150(param_1: i32, param_2: i32, param_3: u32, param_4: u32) -> u32;
}

unsafe fn r32(a: i32) -> i32 {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { mech_trace::rd!("10012c5d", (a as *const i32), read_unaligned) }
}
unsafe fn r16(a: i32) -> u16 {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { mech_trace::rd!("10012c5d", (a as *const u16), read_unaligned) }
}
unsafe fn r8(a: i32) -> u8 {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { mech_trace::rd!("10012c5d", (a as *const u8), read) }
}
unsafe fn w32(a: i32, v: i32) {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { mech_trace::wr!("10012c5d", (a as *mut i32), write_unaligned, v) }
}
unsafe fn w16(a: i32, v: u16) {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { mech_trace::wr!("10012c5d", (a as *mut u16), write_unaligned, v) }
}
unsafe fn w8(a: i32, v: u8) {
    // SAFETY: the address is the game's own memory, as in the original.
    unsafe { mech_trace::wr!("10012c5d", (a as *mut u8), write, v) }
}

unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own signed divide, faulting as the original does.
    unsafe {
        mech_trace::log::debug!(
            f = "10012c5d", b = "idiv", a:% = mech_trace::Args([
                mech_trace::__shown!(b),
                mech_trace::__shown!(a),
            ]); "asm.in"
        );
        asm!("cdq", "idiv {b:e}", b = in(reg) b, inout("eax") a => q, out("edx") _);
        mech_trace::log::debug!(
            f = "10012c5d", b = "idiv", r = "eax", v:% = mech_trace::__shown!(q); "asm.out"
        );
    }
    q
}

unsafe fn calc(l28: i32, v: i32, u: u32) -> i32 {
    // SAFETY: l28 points at the game's record; the divisor global is the game's.
    unsafe {
        let d = mech_trace::gr!("10012c5d", "100a410c", DAT_100a410c);
        let inner = r32(l28 + 8).wrapping_add(idiv(v, d));
        0xf_i32.wrapping_sub(idiv(inner.wrapping_mul(3), (u.wrapping_shl(16)) as i32))
    }
}

unsafe fn flagptr(l20: *mut i32) -> i32 {
    // SAFETY: l20 points at the game's record.
    unsafe { r32(mech_trace::rd!("10012c5d", l20, read_unaligned) + 0x4c) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10012c5d(param_1: i32, param_2: i32) {
    mech_trace::enter!("10012c5d", param_1, param_2);
    // SAFETY: all addresses are the game's own memory, accessed in the original's order.
    unsafe {
        let mut local_c: i32 = 0;
        let mut local_10: i32 = 0;
        let local_8 = param_1;
        let base_eb4f0 = (&raw mut DAT_101eb4f0) as i32;
        if (r8(param_1 + 2) as u32) == mech_trace::gr!("10012c5d", "100adf38", DAT_100adf38) {
            return;
        }
        if !(r32(base_eb4f0.wrapping_add(param_2.wrapping_mul(4))) < r32(param_1 + 0x55)) {
            return;
        }
        w32(
            base_eb4f0.wrapping_add(param_2.wrapping_mul(4)),
            r32(param_1 + 0x55),
        );
        let local_14 = mech_trace::rd!(
            "10012c5d",
            (&raw mut DAT_100eef40)
                .cast::<u32>()
                .offset(param_2 as isize),
            read
        ) as i32;
        let local_20 = r32(local_14 + 0x20) as *mut i32;
        w32(local_14 + 0xdc, r16(param_1 + 3) as i16 as i32);
        if (((r16(param_1 + 3) >> 8) as u8) & 0xf) == 4 && (r16(param_1 + 7) & 0x400) != 0 {
            let local_18 = (r32(local_14 + 0xdc) as u32) & 0xff;
            let a0 =
                ((&raw mut DAT_100ef030) as i32).wrapping_add(local_18.wrapping_mul(0x40) as i32);
            w16(a0, r16(a0) | 0x20);
            let a1 =
                ((&raw mut DAT_100ef032) as i32).wrapping_add(local_18.wrapping_mul(0x40) as i32);
            let sh = (r8(local_14 + 8) & 0x1f) as u32;
            w16(a1, r16(a1) | (1u32.wrapping_shl(sh) as u16));
        }
        w32(local_14 + 0x13c, r16(param_1 + 5) as i16 as i32);
        w16(local_14 + 0x14, r16(local_14 + 0x14) | 1);
        let i_var1 = mech_trace::cr!(
            "10012c5d",
            "10011e80",
            FUN_10011e80(mech_trace::gr!("10012c5d", "100aa210", DAT_100aa210) as i32)
        );
        if i_var1 == r32(local_14 + 4) {
            mech_trace::gw!("10012c5d", "100aa1b4", DAT_100aa1b4, 1);
            if mech_trace::gr!("10012c5d", "100aa1f4", DAT_100aa1f4) == 2 {
                mech_trace::gw!(
                    "10012c5d",
                    "100b38fc",
                    DAT_100b38fc,
                    r32(local_8 + 0x55) as u32
                );
            }
        }
        w32(local_14 + 0x50, r32(local_8 + 0x59));
        w32(local_14 + 0x54, r32(local_8 + 0x5d));
        w32(local_14 + 0x58, r32(local_8 + 0x61));
        w32(local_14 + 0x5c, r32(local_8 + 0x65));
        w32(local_14 + 0x60, r32(local_8 + 0x69));
        w32(local_14 + 100, r32(local_8 + 0x6d));
        mech_trace::cr!(
            "10012c5d",
            "1002f37a",
            FUN_1002f37a(
                r32(local_14 + 0x40),
                r32(local_14 + 0x50) as u32,
                r32(local_14 + 0x54) as u32,
                r32(local_14 + 0x58) as u32,
            )
        );
        mech_trace::cr!(
            "10012c5d",
            "1002f668",
            FUN_1002f668(
                r32(local_14 + 0x40),
                r32(local_14 + 0x5c),
                r32(local_14 + 0x60),
                r32(local_14 + 100),
                0,
            )
        );
        mech_trace::cr!(
            "10012c5d",
            "1002fbc0",
            FUN_1002fbc0(r32(local_14 + 0x40) as *mut i32)
        );
        mech_trace::wr!(
            "10012c5d",
            local_20.add(0xb),
            write_unaligned,
            r32(local_8 + 0x71)
        );
        mech_trace::wr!(
            "10012c5d",
            local_20.add(10),
            write_unaligned,
            r32(local_8 + 0x71)
        );
        mech_trace::wr!(
            "10012c5d",
            local_20.add(0x13),
            write_unaligned,
            r32(local_8 + 0x75)
        );
        mech_trace::wr!(
            "10012c5d",
            local_20.add(0x12),
            write_unaligned,
            r32(local_8 + 0x75)
        );
        mech_trace::wr!(
            "10012c5d",
            local_20.add(0xe),
            write_unaligned,
            r32(local_8 + 0x79)
        );
        mech_trace::wr!(
            "10012c5d",
            local_20.add(2),
            write_unaligned,
            r32(local_8 + 0x7d)
        );
        mech_trace::wr!(
            "10012c5d",
            local_20.add(6),
            write_unaligned,
            r32(local_8 + 0x81)
        );
        mech_trace::wr!(
            "10012c5d",
            local_20.add(0xf5),
            write_unaligned,
            (r16(local_8 + 7) & 0xf) as i32
        );
        let a110 = (local_20.add(0x110)) as i32;
        if (r16(local_8 + 7) & 0x800) == 0 {
            w16(a110, r16(a110) & 0xff7f);
        } else {
            w16(a110, r16(a110) | 0x80);
        }
        let flags: [(u16, i32); 5] = [
            (0x10, 0x30),
            (0x20, 0x1d),
            (0x40, 0x1e),
            (0x80, 0x1f),
            (0x100, 0x20),
        ];
        for (m, off) in flags {
            if (r16(local_8 + 7) & m) == 0 {
                w8(flagptr(local_20) + off, 0);
            } else {
                w8(flagptr(local_20) + off, 1);
            }
        }
        if (r16(local_8 + 7) & 0x200) == 0 {
            w8(flagptr(local_20) + 0x21, 0);
        } else {
            w8(flagptr(local_20) + 0x21, 1);
        }
        let mut local_24: i32 = 0;
        while local_24 < 8 {
            local_10 = 0;
            local_c = 0;
            let local_28 = mech_trace::rd!("10012c5d", local_20.add(0x16), read_unaligned)
                .wrapping_add(local_24.wrapping_mul(0x28));
            w32(local_28, r32(local_8 + 0x15 + local_24 * 4));
            w32(local_28 + 8, r32(local_8 + 0x35 + local_24 * 4));
            let src = if local_24 == 1 {
                0x11
            } else if local_24 == 2 {
                0xd
            } else if local_24 == 3 {
                9
            } else {
                0
            };
            if src != 0 {
                w32(local_28 + 4, r32(local_8 + src));
                let u2 = (((r16(local_28 + 0x26) as i16 as i32) & 0xf0) as u32) >> 4;
                if u2 != 0 {
                    local_10 = calc(local_28, r32(local_28 + 4), u2);
                }
            }
            let local_1c = ((r16(local_28 + 0x26) as i16 as i32) & 0xf) as u32;
            if local_1c != 0 {
                local_c = calc(local_28, r32(local_28), local_1c);
            }
            let mut i_var1 = local_c;
            if local_c <= local_10 {
                i_var1 = local_10;
            }
            mech_trace::cr!(
                "10012c5d",
                "100300e2",
                FUN_100300e2(
                    r32(mech_trace::rd!("10012c5d", local_20, read_unaligned) + 0x40),
                    i_var1,
                    (local_24 + 1) as u32,
                )
            );
            if r32(local_28 + 8) < 1 && (r16(local_28 + 0x26) & 0x2000) == 0 {
                mech_trace::cr!(
                    "10012c5d",
                    "10005207",
                    FUN_10005207(r32(local_14 + 0x13c), local_20, (local_24 + 1) as u32)
                );
            }
            local_24 += 1;
        }
        let state = mech_trace::rd!("10012c5d", local_20.add(0xf5), read_unaligned);
        let flag_base = (&raw mut DAT_101eb520) as i32;
        if state == 4 || state == 5 {
            if r8(flag_base.wrapping_add(r32(local_14 + 4))) != 1 {
                let mut local_50 = [0i8; 40];
                mech_trace::cr!(
                    "10012c5d",
                    "10004b19",
                    FUN_10004b19(r32(local_14 + 0x13c), local_20)
                );
                if r32(local_14 + 0x13c) < 0 || 7 < r32(local_14 + 0x13c) {
                    mech_trace::cr!(
                        "10012c5d",
                        "_sprintf",
                        _sprintf(
                            local_50.as_mut_ptr(),
                            (&raw const DAT_100aa28c).cast::<i8>(),
                            local_14 + 0x110,
                        )
                    );
                } else if r32(local_14 + 4) == r32(local_14 + 0x13c) {
                    mech_trace::cr!(
                        "10012c5d",
                        "_sprintf",
                        _sprintf(
                            local_50.as_mut_ptr(),
                            (&raw const DAT_100aa264).cast::<i8>(),
                            local_14 + 0x110,
                        )
                    );
                } else {
                    let other = mech_trace::rd!(
                        "10012c5d",
                        (&raw mut DAT_100eef40)
                            .cast::<u32>()
                            .offset(r32(local_14 + 0x13c) as isize),
                        read
                    ) as i32;
                    mech_trace::cr!(
                        "10012c5d",
                        "_sprintf",
                        _sprintf(
                            local_50.as_mut_ptr(),
                            (&raw const DAT_100aa274).cast::<i8>(),
                            local_14 + 0x110,
                            other.wrapping_add(0x110),
                        )
                    );
                }
                mech_trace::cr!(
                    "10012c5d",
                    "1005be24",
                    FUN_1005be24(local_50.as_mut_ptr(), 1, 0x712, 0x50)
                );
                w8(flag_base.wrapping_add(r32(local_14 + 4)), 1);
            }
        } else if r8(flag_base.wrapping_add(r32(local_14 + 4))) == 1 {
            let mut local_78 = [0i8; 40];
            mech_trace::cr!(
                "10012c5d",
                "10071150",
                FUN_10071150(r32(local_14 + 4), 0, 0, 0)
            );
            mech_trace::cr!(
                "10012c5d",
                "_sprintf",
                _sprintf(
                    local_78.as_mut_ptr(),
                    (&raw const DAT_100aa29c).cast::<i8>(),
                    local_14 + 0x110,
                )
            );
            mech_trace::cr!(
                "10012c5d",
                "1005be24",
                FUN_1005be24(local_78.as_mut_ptr(), 1, 0x712, 0x50)
            );
            w8(flag_base.wrapping_add(r32(local_14 + 4)), 0);
        }
        let _ = (local_c, local_10);
    }
}
