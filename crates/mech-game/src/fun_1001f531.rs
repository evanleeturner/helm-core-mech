// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100a40a8: i32;
    static mut DAT_100b3904: i32;
    static mut DAT_100ab2f0: i32;
    static mut DAT_100c6b10: i32;
    static mut DAT_100aa554: i32;
    static mut DAT_100aa550: i32;
    static mut DAT_100aa54c: i32;
    static mut DAT_100aa1ac: i32;
    static mut DAT_100aa55c: u32;
    static mut DAT_100aa558: i32;
    static mut DAT_100aa560: i32;
    static mut DAT_100ab094: i32;
    static mut DAT_100ab314: i32;
    static mut _DAT_100ab310: i32;
    static mut DAT_100adf34: i32;
    static mut DAT_100ab2f4: i32;
    static mut DAT_100aaf68: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10067af1(a: *mut i32);
    fn FUN_10003d60(a: i32, b: i32) -> i32;
    fn FUN_1000fc30(a: i32, b: i32, c: i32) -> i32;
    fn FUN_10011e60(a: i32, b: i32, c: i32) -> i32;
    fn FUN_10002570(a: i32, b: i32) -> i32;
    fn FUN_1001b520(a: *mut i32, b: *mut i32, c: i32, d: i32);
    fn FUN_1007ee80(
        a: *mut i32,
        b: *mut i32,
        c: *mut i32,
        d: i32,
        e: i32,
        f: i32,
        g: *mut i32,
        h: *mut i32,
        i: *mut i32,
    ) -> i32;
    fn FUN_10080048(a: *mut i32);
    fn FUN_100142b4(a: i32, b: i32, c: i32) -> i32;
    fn FUN_1001355e(a: i32, b: i32, c: i32, d: i32);
    fn FUN_10003742(a: u32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_1006b6af(a: i32, b: i32, c: i32);
    fn FUN_1007fc1d(a: *mut i32, b: *mut i32);
    fn FUN_1007fecd(a: *mut i32, b: i32);
    fn FUN_1006b60b(a: *mut i32, b: i32);
    fn FUN_100065c4(a: i32, b: *mut i32, c: i32, d: i32);
    fn FUN_10027cd0(a: i32, b: *mut i32);
    fn FUN_1002f37a(a: i32, b: i32, c: i32, d: i32);
    fn FUN_1002f668(a: i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_1002f28d(a: i32) -> i32;
    fn FUN_1001cac9(a: i32, b: *mut i32, c: *mut i32, d: *mut i32) -> i32;
    fn FUN_1002f322(a: i32, b: *mut i32, c: *mut i32, d: *mut i32);
    fn FUN_1002f6a8(a: *mut c_void, b: i32, c: i32, d: i32, e: i32);
    fn FUN_10089cc9(a: *mut i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32);
    fn FUN_10089118(a: *mut i32, b: *mut i32, c: *mut i32, d: *mut i32);
    fn FUN_1002f3a7(a: i32, b: i32, c: i32, d: i32);
    fn FUN_100037f0(a: u32, b: i32, c: u32, d: i32, e: i32);
    fn FUN_10063e41(a: i32);
    fn FUN_1002f2d6(a: *mut c_void, b: i32, c: *mut i32, d: *mut i32, e: *mut i32);
    fn FUN_10063fbd(a: i32, b: *mut i32);
    fn FUN_10063f34(a: i32) -> i32;
    fn FUN_1002650e(a: *mut i32, b: i32);
    fn FUN_10022b50(a: i32, b: i32) -> i32;
    fn FUN_10006bbb(a: u32) -> i32;
    fn FUN_1002fbc0(a: *mut i32);
    fn FUN_10006b20(a: i32) -> i32;
    fn FUN_10006b9a(a: i32) -> i32;
}

#[inline(always)]
fn ad(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}
#[inline(always)]
fn sb(a: i32, b: i32) -> i32 {
    a.wrapping_sub(b)
}
#[inline(always)]
fn ml(a: i32, b: i32) -> i32 {
    a.wrapping_mul(b)
}
#[inline(always)]
fn ng(a: i32) -> i32 {
    a.wrapping_neg()
}

#[inline(always)]
unsafe fn rd(a: i32) -> i32 {
    // SAFETY: caller supplies an address of the game's memory.
    unsafe { mech_trace::rd!("1001f531", (a as *const i32), read_unaligned) }
}
#[inline(always)]
unsafe fn wr(a: i32, v: i32) {
    // SAFETY: caller supplies an address of the game's memory.
    unsafe { mech_trace::wr!("1001f531", (a as *mut i32), write_unaligned, v) }
}
#[inline(always)]
unsafe fn rb(a: i32) -> i8 {
    // SAFETY: caller supplies an address of the game's memory.
    unsafe { mech_trace::rd!("1001f531", (a as *const i8), read) }
}
#[inline(always)]
unsafe fn rw(a: i32) -> u16 {
    // SAFETY: caller supplies an address of the game's memory.
    unsafe { mech_trace::rd!("1001f531", (a as *const u16), read_unaligned) }
}
#[inline(always)]
unsafe fn ww(a: i32, v: u16) {
    // SAFETY: caller supplies an address of the game's memory.
    unsafe { mech_trace::wr!("1001f531", (a as *mut u16), write_unaligned, v) }
}

#[inline(always)]
unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: raw machine divide; faults exactly as the original does.
    unsafe {
        core::arch::asm!(
            "cdq",
            "idiv {b:e}",
            b = in(reg) b,
            inout("eax") a => q,
            out("edx") _,
        );
    }
    q
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    unused_assignments,
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_upper_case_globals
)]
pub unsafe extern "C" fn FUN_1001f531(param_1: *mut i32) {
    mech_trace::enter!("1001f531", param_1);
    // SAFETY: all memory touched is the game's own object graph, globals and
    // callees, accessed at the offsets the original uses.
    unsafe {
        let mut local_8: i32 = 0;
        let mut local_38: i32 = 0;
        let mut local_20: i32 = 0;
        if param_1.is_null() {
            return;
        }
        let p = param_1 as usize as i32;

        macro_rules! m {
            ($i:expr) => {
                rd(p.wrapping_add($i * 4))
            };
        }
        macro_rules! setm {
            ($i:expr, $v:expr) => {{
                let v = $v;
                wr(p.wrapping_add($i * 4), v)
            }};
        }
        macro_rules! pm {
            ($i:expr) => {
                p.wrapping_add($i * 4) as *mut i32
            };
        }
        macro_rules! obj {
            ($o:expr) => {
                rd(rd(p).wrapping_add($o))
            };
        }
        macro_rules! seto {
            ($o:expr, $v:expr) => {{
                let v = $v;
                wr(rd(p).wrapping_add($o), v)
            }};
        }
        macro_rules! subo {
            ($o:expr) => {
                rd(obj!(0x4c).wrapping_add($o))
            };
        }
        macro_rules! subc {
            ($o:expr) => {
                rb(obj!(0x4c).wrapping_add($o))
            };
        }

        if (m!(0xf5) != 4 || obj!(0x78) == 0) && (rw(p.wrapping_add(0x440)) & 0x100) == 0 {
            setm!(0xf1, 0);
            let local_1c: u32 = (rd(rd(p).wrapping_add(4))
                == mech_trace::gr!("1001f531", "100adf38", DAT_100adf38))
                as u32;
            let mut local_74: i32 = 0;
            if (rw(rd(p).wrapping_add(0x14)) & 1) == 0 {
                FUN_10067af1(pm!(1));
                FUN_10067af1(pm!(5));
                FUN_10067af1(pm!(0xd));
                FUN_10067af1(pm!(0x11));
                FUN_10067af1(pm!(9));
                let mut local_7c: i32 = m!(0x10a);
                let mut local_88: i32 = m!(0x10b);
                let mut local_10: i32 = m!(0x10c);
                let mut local_84: i32 = 0;
                let mut local_c: i32 = ng(mech_trace::gr!("1001f531", "100a40a8", DAT_100a40a8));
                let mut local_30: i32 = 0;
                let mut local_2c: i32 = 0;
                let mut local_3c: i32 = 0;
                let mut local_28: i32;
                let mut local_4c: i32;
                let mut local_50: i32;
                if obj!(0x78) == 0 || (subc!(0x1d) != 0 && 0 < m!(0xfd)) {
                    local_28 = mech_trace::cr!(
                        "1001f531",
                        "1000fc30",
                        FUN_1000fc30(local_7c, 0, local_10)
                    );
                    local_4c =
                        mech_trace::cr!("1001f531", "10011e60", FUN_10011e60(0x1400, m!(0xee), 2));
                    if local_28 < 1 || local_4c < 1 || m!(0x108) < 1 {
                        local_2c = 0;
                        local_3c = 0;
                    } else {
                        let local_a0 = mech_trace::cr!(
                            "1001f531",
                            "10003d60",
                            FUN_10003d60(local_4c, local_4c)
                        );
                        let iVar1 = m!(0x108);
                        let iVar2 = mech_trace::cr!(
                            "1001f531",
                            "10002570",
                            FUN_10002570(m!(0x108), local_4c)
                        );
                        let mut local_94 = sb(iVar1, iVar2 / 3);
                        let tmp = ml(local_4c, 3);
                        let local_9c = sb(
                            local_a0,
                            (((tmp as u32).wrapping_add(((tmp >> 0x1f) as u32) & 3)) as i32) >> 2,
                        );
                        local_94 = mech_trace::cr!(
                            "1001f531",
                            "10002570",
                            FUN_10002570(local_94, local_9c)
                        );
                        let iVar1 = m!(0x108);
                        let iVar2 = mech_trace::cr!(
                            "1001f531",
                            "10003d60",
                            FUN_10003d60(local_94, local_a0)
                        );
                        let mut local_98 = sb(iVar1, iVar2);
                        local_98 = mech_trace::cr!(
                            "1001f531",
                            "10002570",
                            FUN_10002570(local_98, local_4c)
                        );
                        let iVar1 = local_28;
                        let iVar2 = mech_trace::cr!(
                            "1001f531",
                            "10003d60",
                            FUN_10003d60(local_94, local_28)
                        );
                        local_50 = mech_trace::cr!(
                            "1001f531",
                            "10003d60",
                            FUN_10003d60(ad(local_98, iVar2), iVar1)
                        );
                        local_50 = ng(local_50);
                        local_2c = mech_trace::cr!(
                            "1001f531",
                            "10011e60",
                            FUN_10011e60(local_50, local_7c, local_28)
                        );
                        local_3c = mech_trace::cr!(
                            "1001f531",
                            "10011e60",
                            FUN_10011e60(local_50, local_10, local_28)
                        );
                    }
                } else {
                    let mut local_24 = m!(0xb);
                    if local_24 < 0x20 && -0x20 < local_24 {
                        local_24 = 0;
                    }
                    let local_40 =
                        mech_trace::cr!("1001f531", "10003d60", FUN_10003d60(local_24, obj!(0xbc)));
                    let local_68 =
                        mech_trace::cr!("1001f531", "10003d60", FUN_10003d60(local_24, obj!(0xb8)));
                    if mech_trace::gr!("1001f531", "100b3904", DAT_100b3904) < 0x2d {
                        local_84 = sb(local_40, m!(0x10a)) / 0x2d;
                        local_30 = sb(local_68, m!(0x10c)) / 0x2d;
                    } else if mech_trace::gr!("1001f531", "100b3904", DAT_100b3904) < 1 {
                        local_84 = 0;
                        local_30 = 0;
                    } else {
                        local_84 = sb(local_40, m!(0x10a))
                            / mech_trace::gr!("1001f531", "100b3904", DAT_100b3904);
                        local_30 = sb(local_68, m!(0x10c))
                            / mech_trace::gr!("1001f531", "100b3904", DAT_100b3904);
                    }
                }
                local_50 = 0;
                let mut local_34: i32 = 0;
                let mut local_18: i32 = 0;
                let mut local_90: i32 = 0;

                macro_rules! set_c {
                    () => {
                        if local_88 < 1 {
                            local_c = 0;
                        } else {
                            local_c = ng(mech_trace::gr!("1001f531", "100a40a8", DAT_100a40a8));
                        }
                    };
                }

                if subc!(0x1d) == 0 || m!(0xfd) < 1 || m!(0xf5) != 2 {
                    if local_88 == 0 && obj!(0x78) != 0 {
                        local_c = 0;
                    }
                } else {
                    local_7c = m!(0x10a);
                    local_10 = m!(0x10c);
                    if 0 < local_88 {
                        local_50 = mech_trace::cr!(
                            "1001f531",
                            "10003d60",
                            FUN_10003d60(
                                ml(
                                    sb(
                                        mech_trace::gr!("1001f531", "100a40a8", DAT_100a40a8),
                                        m!(0x108)
                                    ),
                                    0xe24
                                ),
                                local_88
                            )
                        );
                        local_50 = idiv(
                            local_50,
                            mech_trace::gr!("1001f531", "100ab2f0", DAT_100ab2f0),
                        );
                    }
                    local_c = ad(ad(local_c, m!(0x108)), local_50);
                    if subo!(0xc) == 0 {
                        if subc!(0x1e) == 0 {
                            if subc!(0x1f) == 0 {
                                if subc!(0x20) == 0 {
                                    if subc!(0x21) != 0 {
                                        set_c!();
                                        let iVar1 = mech_trace::cr!(
                                            "1001f531",
                                            "10003d60",
                                            FUN_10003d60(obj!(0xbc), m!(0x108))
                                        );
                                        local_84 = ad(local_84, sb(local_2c, iVar1));
                                        let iVar1 = mech_trace::cr!(
                                            "1001f531",
                                            "10003d60",
                                            FUN_10003d60(obj!(0xb8), m!(0x108))
                                        );
                                        local_30 = ad(local_30, sb(local_3c, iVar1));
                                    }
                                } else {
                                    set_c!();
                                    let iVar1 = mech_trace::cr!(
                                        "1001f531",
                                        "10003d60",
                                        FUN_10003d60(obj!(0xbc), m!(0x108))
                                    );
                                    local_84 = ad(ad(local_84, local_2c), iVar1);
                                    let iVar1 = mech_trace::cr!(
                                        "1001f531",
                                        "10003d60",
                                        FUN_10003d60(obj!(0xb8), m!(0x108))
                                    );
                                    local_30 = ad(ad(local_30, local_3c), iVar1);
                                }
                            } else {
                                set_c!();
                                let iVar1 = mech_trace::cr!(
                                    "1001f531",
                                    "10003d60",
                                    FUN_10003d60(obj!(0xb8), m!(0x108))
                                );
                                local_84 = ad(ad(local_84, local_2c), iVar1);
                                let iVar1 = mech_trace::cr!(
                                    "1001f531",
                                    "10003d60",
                                    FUN_10003d60(obj!(0xbc), m!(0x108))
                                );
                                local_30 = ad(local_30, sb(local_3c, iVar1));
                            }
                        } else {
                            set_c!();
                            let iVar1 = mech_trace::cr!(
                                "1001f531",
                                "10003d60",
                                FUN_10003d60(obj!(0xb8), m!(0x108))
                            );
                            local_84 = ad(local_84, sb(local_2c, iVar1));
                            let iVar1 = mech_trace::cr!(
                                "1001f531",
                                "10003d60",
                                FUN_10003d60(obj!(0xbc), m!(0x108))
                            );
                            local_30 = ad(ad(local_30, local_3c), iVar1);
                        }
                    } else {
                        set_c!();
                    }
                }
                FUN_1001b520(
                    &raw mut local_90,
                    &raw mut local_7c,
                    local_84,
                    mech_trace::gr!("1001f531", "100b3904", DAT_100b3904),
                );
                FUN_1001b520(
                    &raw mut local_18,
                    &raw mut local_88,
                    local_c,
                    mech_trace::gr!("1001f531", "100b3904", DAT_100b3904),
                );
                FUN_1001b520(
                    &raw mut local_34,
                    &raw mut local_10,
                    local_30,
                    mech_trace::gr!("1001f531", "100b3904", DAT_100b3904),
                );
                if local_7c < 0x1000 && -0x1000 < local_7c {
                    local_7c = 0;
                }
                if local_10 < 0x1000 && -0x1000 < local_10 {
                    local_10 = 0;
                }
                setm!(0x10d, local_7c);
                setm!(0x10e, local_88);
                setm!(0x10f, local_10);
                let mut local_80: i32 = obj!(0x50);
                let mut local_8c: i32 = obj!(0x54);
                let mut local_14: i32 = obj!(0x58);
                local_20 = 0;
                seto!(0x7c, -1);
                let iVar1 = mech_trace::cr!(
                    "1001f531",
                    "1007ee80",
                    FUN_1007ee80(
                        param_1,
                        &raw mut local_38,
                        &raw mut local_20,
                        local_90,
                        local_18,
                        local_34,
                        &raw mut local_80,
                        &raw mut local_8c,
                        &raw mut local_14,
                    )
                );
                if iVar1 == 0 {
                    if local_1c != 0 {
                        mech_trace::gw!("1001f531", "100c6b10", DAT_100c6b10, 0);
                    }
                    mech_trace::gw!("1001f531", "100aa554", DAT_100aa554, 0);
                    mech_trace::gw!("1001f531", "100aa550", DAT_100aa550, 0);
                    mech_trace::gw!("1001f531", "100aa54c", DAT_100aa54c, 0);
                    let iVar1 = mech_trace::cr!(
                        "1001f531",
                        "100142b4",
                        FUN_100142b4(local_80, local_8c, local_14)
                    );
                    seto!(0x74, iVar1);
                    local_8 = sb(sb(local_8c, m!(0x100)), obj!(0x74));
                } else {
                    local_90 = sb(local_80, obj!(0x50));
                    local_18 = sb(local_8c, obj!(0x54));
                    local_34 = sb(local_14, obj!(0x58));
                    local_7c = m!(0x10a);
                    local_88 = m!(0x10b);
                    local_10 = m!(0x10c);
                    let local_b4 = m!(0xf6);
                    let mut local_b0: i32 = 0;
                    let mut local_a4: i32 = 0;
                    let mut local_a8: i32 = 0;
                    let mut local_ac: i32 = 0;
                    let iVar1 = mech_trace::cr!(
                        "1001f531",
                        "1007ee80",
                        FUN_1007ee80(
                            param_1,
                            &raw mut local_b0,
                            &raw mut local_20,
                            local_90,
                            local_18,
                            local_34,
                            &raw mut local_a4,
                            &raw mut local_a8,
                            &raw mut local_ac,
                        )
                    );
                    if iVar1 != 0 {
                        if local_b0 != local_38 {
                            local_80 = obj!(0x50);
                            local_8c = obj!(0x54);
                            local_14 = obj!(0x58);
                        }
                        setm!(0x10a, ml(local_7c, 2));
                        local_7c = m!(0x10a);
                        setm!(0x10b, ml(local_88, 2));
                        local_88 = m!(0x10b);
                        setm!(0x10c, ml(local_10, 2));
                        local_10 = m!(0x10c);
                    }
                    setm!(0xf6, local_b4);
                    if m!(0xf5) == 4 {
                        FUN_10080048(param_1);
                        seto!(0x78, 1);
                        return;
                    }
                    mech_trace::gw!("1001f531", "100aa554", DAT_100aa554, 0);
                    mech_trace::gw!("1001f531", "100aa550", DAT_100aa550, 0);
                    mech_trace::gw!("1001f531", "100aa54c", DAT_100aa54c, 0);
                    let iVar1 = mech_trace::cr!(
                        "1001f531",
                        "100142b4",
                        FUN_100142b4(local_80, local_8c, local_14)
                    );
                    seto!(0x74, iVar1);
                    local_8 = sb(sb(local_8c, m!(0x100)), obj!(0x74));
                    if mech_trace::gr!("1001f531", "100aa1ac", DAT_100aa1ac) == 0
                        || (mech_trace::gr!("1001f531", "100aa55c", DAT_100aa55c) as i32) < 0xddb5
                        || local_20 == 0
                    {
                        if (mech_trace::gr!("1001f531", "100aa55c", DAT_100aa55c) as i32) < 0xb506
                            || 999 < local_8
                            || local_8 < -9999
                        {
                            if local_1c != 0
                                && mech_trace::gr!("1001f531", "100ab094", DAT_100ab094) != 0
                                && mech_trace::gr!("1001f531", "100c6b10", DAT_100c6b10) == 0
                            {
                                mech_trace::gw!("1001f531", "100c6b10", DAT_100c6b10, 1);
                                let local_78 = mech_trace::cr!(
                                    "1001f531",
                                    "1000fc30",
                                    FUN_1000fc30(m!(0x10d), m!(0x10e), m!(0x10f))
                                );
                                if 200000 < local_78 {
                                    let local_c4: u32;
                                    if local_20 == 0 {
                                        if local_38 == 0
                                            || (rw(local_38.wrapping_add(2)) & 0x200) == 0
                                        {
                                            local_c4 = 0xfa;
                                        } else {
                                            local_c4 = 0xd9;
                                        }
                                    } else {
                                        if mech_trace::gr!("1001f531", "100aa1ac", DAT_100aa1ac)
                                            != 0
                                        {
                                            FUN_1001355e(
                                                rd(local_20.wrapping_add(4)),
                                                mech_trace::gr!(
                                                    "1001f531",
                                                    "100aa558",
                                                    DAT_100aa558
                                                ),
                                                mech_trace::gr!(
                                                    "1001f531",
                                                    "100aa55c",
                                                    DAT_100aa55c
                                                )
                                                    as i32,
                                                mech_trace::gr!(
                                                    "1001f531",
                                                    "100aa560",
                                                    DAT_100aa560
                                                ),
                                            );
                                        }
                                        local_c4 = 0x105;
                                    }
                                    let mut local_c0 = local_78;
                                    if 1500000 < local_78 {
                                        local_c0 = 1500000;
                                    }
                                    local_c0 = mech_trace::cr!(
                                        "1001f531",
                                        "10011e60",
                                        FUN_10011e60(200, local_c0, 1500000)
                                    );
                                    FUN_10003742(local_c4, local_c0, 0x40, 5, 0x32);
                                    FUN_1006b6af(
                                        mech_trace::gr!("1001f531", "100aa558", DAT_100aa558),
                                        mech_trace::gr!("1001f531", "100aa55c", DAT_100aa55c)
                                            as i32,
                                        mech_trace::gr!("1001f531", "100aa560", DAT_100aa560),
                                    );
                                }
                            }
                            if local_20 == 0 {
                                FUN_1007fecd(param_1, local_38);
                            } else {
                                FUN_1007fc1d(
                                    param_1,
                                    rd(local_20.wrapping_add(0x20)) as usize as *mut i32,
                                );
                            }
                        } else {
                            setm!(0xf6, 0);
                            local_8 = -1;
                        }
                    } else {
                        FUN_1001355e(
                            rd(local_20.wrapping_add(4)),
                            mech_trace::gr!("1001f531", "100aa558", DAT_100aa558),
                            mech_trace::gr!("1001f531", "100aa55c", DAT_100aa55c) as i32,
                            mech_trace::gr!("1001f531", "100aa560", DAT_100aa560),
                        );
                        if local_1c != 0
                            && mech_trace::gr!("1001f531", "100ab094", DAT_100ab094) != 0
                            && mech_trace::gr!("1001f531", "100c6b10", DAT_100c6b10) == 0
                        {
                            mech_trace::gw!("1001f531", "100c6b10", DAT_100c6b10, 1);
                            let local_78 = mech_trace::cr!(
                                "1001f531",
                                "1000fc30",
                                FUN_1000fc30(m!(0x10d), m!(0x10e), m!(0x10f))
                            );
                            if 200000 < local_78 {
                                let local_bc: u32 = 0x105;
                                let mut local_b8 = local_78;
                                if 1500000 < local_78 {
                                    local_b8 = 1500000;
                                }
                                local_b8 = mech_trace::cr!(
                                    "1001f531",
                                    "10011e60",
                                    FUN_10011e60(200, local_b8, 1500000)
                                );
                                FUN_10003742(local_bc, local_b8, 0x40, 5, 0x32);
                            }
                        }
                    }
                }
                seto!(0x78, 0);
                if m!(0x100) < local_8 {
                    seto!(0x80, obj!(0x80) & -2);
                    seto!(0x80, obj!(0x80) | 4);
                    if local_1c != 0 && mech_trace::gr!("1001f531", "100ab314", DAT_100ab314) != 0 {
                        mech_trace::gw!("1001f531", "100ab310", _DAT_100ab310, 1);
                    }
                } else {
                    seto!(0x80, obj!(0x80) & -5);
                    if local_8 < 1 {
                        seto!(0x78, 1);
                        if local_8 < 0 {
                            if local_88 < -0x56276 {
                                if m!(0xf5) == 4 {
                                    FUN_10080048(param_1);
                                    return;
                                }
                                FUN_1006b60b(param_1, local_88);
                            }
                            if local_88 < -0x102762
                                && rb(mech_trace::gr!("1001f531", "100adf34", DAT_100adf34)
                                    .wrapping_add(3))
                                    != 0
                            {
                                let mut local_c8 = mech_trace::cr!(
                                    "1001f531",
                                    "10002570",
                                    FUN_10002570(ad(local_88, 0x102762), -0x204ec4)
                                );
                                local_c8 = ml(local_c8, 0x32);
                                FUN_100065c4(
                                    mech_trace::gr!("1001f531", "100adf38", DAT_100adf38),
                                    param_1,
                                    local_c8,
                                    8,
                                );
                                FUN_100065c4(
                                    mech_trace::gr!("1001f531", "100adf38", DAT_100adf38),
                                    param_1,
                                    local_c8,
                                    7,
                                );
                            }
                            local_88 = 0;
                            setm!(0x10b, 0);
                            local_8c = ad(obj!(0x74), m!(0x100));
                        }
                    }
                }
                if (mech_trace::gr!("1001f531", "100aa54c", DAT_100aa54c) != 0
                    || mech_trace::gr!("1001f531", "100aa554", DAT_100aa554) != 0)
                    && obj!(0x78) != 0
                {
                    let iVar1 = mech_trace::cr!(
                        "1001f531",
                        "10003d60",
                        FUN_10003d60(
                            obj!(0xb8),
                            mech_trace::gr!("1001f531", "100aa554", DAT_100aa554)
                        )
                    );
                    let mut local_cc = mech_trace::cr!(
                        "1001f531",
                        "10003d60",
                        FUN_10003d60(
                            obj!(0xbc),
                            mech_trace::gr!("1001f531", "100aa54c", DAT_100aa54c)
                        )
                    );
                    local_cc = ad(iVar1, local_cc);
                    if mech_trace::gr!("1001f531", "100ab2f4", DAT_100ab2f4) < local_cc
                        || (ng(local_cc) != mech_trace::gr!("1001f531", "100ab2f4", DAT_100ab2f4)
                            && local_cc
                                <= ng(mech_trace::gr!("1001f531", "100ab2f4", DAT_100ab2f4)))
                    {
                        let iVar1 = mech_trace::gr!("1001f531", "100b3904", DAT_100b3904);
                        let iVar2 = mech_trace::cr!(
                            "1001f531",
                            "10003d60",
                            FUN_10003d60(
                                mech_trace::gr!("1001f531", "100a40a8", DAT_100a40a8),
                                local_cc
                            )
                        );
                        let iVar1 =
                            mech_trace::cr!("1001f531", "10003d60", FUN_10003d60(iVar2, iVar1));
                        setm!(0xb, ad(m!(0xb), iVar1));
                    }
                }
                if mech_trace::gr!("1001f531", "100aa1ac", DAT_100aa1ac) == 0 {
                    if obj!(0) == 2 {
                        local_8c = m!(0x100);
                    }
                    if obj!(0) == 8 {
                        FUN_10027cd0(rd(p), &raw mut local_8c);
                    }
                }
                seto!(0x50, local_80);
                seto!(0x54, local_8c);
                seto!(0x58, local_14);
                seto!(
                    0x60,
                    ad(
                        obj!(0x60),
                        ml(
                            m!(0xf),
                            mech_trace::gr!("1001f531", "100b3904", DAT_100b3904)
                        ) / 0xb5
                    )
                );
                seto!(0x60, obj!(0x60) % 0x1680000);
                setm!(0x10a, local_7c);
                setm!(0x10b, local_88);
                setm!(0x10c, local_10);
                FUN_1002f37a(obj!(0x40), obj!(0x50), obj!(0x54), obj!(0x58));
                FUN_1002f668(obj!(0x40), obj!(0x5c), obj!(0x60), obj!(100), 0);
                let uVar3 = sb(sb(obj!(0x54), obj!(0x74)), m!(0x100)) as u32;
                let uVar4 = ((uVar3 as i32) >> 0x1f) as u32;
                local_8 = ((uVar3 ^ uVar4).wrapping_sub(uVar4)) as i32;

                let mut local_180: [u32; 14] = [0; 14];
                let mut local_12c: [i32; 12] = [0; 12];
                let mut local_1d0: [i32; 6] = [0; 6];

                if (rw(p.wrapping_add(0x442)) & 4) == 0
                    || (obj!(0x78) == 0 && 100 < local_8)
                    || (mech_trace::gr!("1001f531", "100aa1ac", DAT_100aa1ac) != 0 && local_1c == 0)
                {
                    if (rw(p.wrapping_add(0x442)) & 4) != 0
                        && obj!(0x78) == 0
                        && m!(0x111) != 0
                        && (mech_trace::gr!("1001f531", "100aa1ac", DAT_100aa1ac) == 0
                            || local_1c != 0)
                    {
                        let mut local_144: i32 = 0;
                        local_180[1] = 0;
                        let mut local_140: i32 = m!(0x11a);
                        if -1 < m!(0x114) {
                            setm!(0x114, -0x190000);
                        }
                        if m!(0x111) < 1 {
                            seto!(0x5c, sb(obj!(0x5c), m!(0x111)));
                            setm!(0x111, 0);
                        } else {
                            FUN_1001b520(
                                pm!(0x111),
                                pm!(0x114),
                                -0x640000,
                                mech_trace::gr!("1001f531", "100b3904", DAT_100b3904),
                            );
                        }
                        let local_148 = sb(m!(0x111), obj!(0x5c));
                        seto!(0x5c, ad(obj!(0x5c), local_148));
                        FUN_10089cc9(
                            local_180.as_mut_ptr().add(2).cast::<i32>(),
                            m!(0x111),
                            0,
                            0,
                            0,
                            0,
                            0,
                        );
                        FUN_10089118(
                            local_180.as_mut_ptr().add(2).cast::<i32>(),
                            local_180.as_mut_ptr().add(1).cast::<i32>(),
                            &raw mut local_140,
                            &raw mut local_144,
                        );
                        setm!(0x100, local_140);
                        FUN_1002f6a8(obj!(0x40) as usize as *mut c_void, local_148, 0, 0, 0);
                        ww(p.wrapping_add(0x442), rw(p.wrapping_add(0x442)) & 0xfff7);
                    }
                } else {
                    let mut local_54: i32 = 0;
                    let mut local_60: i32 = 0;
                    let mut local_64: i32 = 0;
                    let mut local_6c: i32;
                    let mut local_44: i32;
                    if m!(0xe4) != 0 && rd(m!(0xe4).wrapping_add(0x6c)) != 0 {
                        let local_d0 =
                            mech_trace::cr!("1001f531", "1002f28d", FUN_1002f28d(m!(0xe4)));
                        local_6c = mech_trace::cr!(
                            "1001f531",
                            "1001cac9",
                            FUN_1001cac9(
                                local_d0,
                                &raw mut local_54,
                                &raw mut local_60,
                                &raw mut local_64,
                            )
                        );
                        FUN_1002f322(
                            m!(0xe4),
                            &raw mut local_54,
                            &raw mut local_60,
                            &raw mut local_64,
                        );
                        local_60 = sb(local_60, local_6c >> 1);
                        local_44 = mech_trace::cr!(
                            "1001f531",
                            "100142b4",
                            FUN_100142b4(local_54, local_60, local_64)
                        );
                        if local_60 <= local_44 {
                            local_74 = 1;
                        }
                    }
                    if m!(0xe5) != 0 && rd(m!(0xe5).wrapping_add(0x6c)) != 0 {
                        let local_d0 =
                            mech_trace::cr!("1001f531", "1002f28d", FUN_1002f28d(m!(0xe5)));
                        local_6c = mech_trace::cr!(
                            "1001f531",
                            "1001cac9",
                            FUN_1001cac9(
                                local_d0,
                                &raw mut local_54,
                                &raw mut local_60,
                                &raw mut local_64,
                            )
                        );
                        FUN_1002f322(
                            m!(0xe5),
                            &raw mut local_54,
                            &raw mut local_60,
                            &raw mut local_64,
                        );
                        local_60 = sb(local_60, local_6c >> 1);
                        local_44 = mech_trace::cr!(
                            "1001f531",
                            "100142b4",
                            FUN_100142b4(local_54, local_60, local_64)
                        );
                        if local_60 <= local_44 {
                            local_74 = 1;
                        }
                    }
                    if local_74 == 0 {
                        let mut local_e4: i32 = 0;
                        let mut local_130: i32 = 0;
                        let mut local_f0: i32 = 0;
                        let mut local_e0: i32 = 0;
                        let mut local_e8: i32 = m!(0x11a);
                        let local_fc: i32 = m!(0x111);
                        let mut local_d4: i32 = m!(0x114);
                        let mut local_dc: i32 = local_fc;
                        let mut local_d8: i32 = local_e8;
                        FUN_1001b520(
                            &raw mut local_dc,
                            &raw mut local_d4,
                            0x640000,
                            mech_trace::gr!("1001f531", "100b3904", DAT_100b3904),
                        );
                        local_dc = local_dc % 0x1680000;
                        if (0 < local_d4 && 0 < m!(0x114))
                            || (local_d4 < 0 && m!(0x114) < 0)
                            || (-0x50000 < m!(0x111) && m!(0x111) < 0x50000)
                        {
                            setm!(0x111, local_dc);
                            setm!(0x114, local_d4);
                            let local_48 = sb(m!(0x111), obj!(0x5c));
                            seto!(0x5c, ad(obj!(0x5c), local_48));
                            FUN_1002f6a8(obj!(0x40) as usize as *mut c_void, local_48, 0, 0, 0);
                            FUN_10089cc9(local_12c.as_mut_ptr(), local_fc, 0, 0, 0, 0, 0);
                            FUN_10089118(
                                local_12c.as_mut_ptr(),
                                &raw mut local_130,
                                &raw mut local_d8,
                                &raw mut local_e0,
                            );
                            FUN_10089cc9(local_12c.as_mut_ptr(), m!(0x111), 0, 0, 0, 0, 0);
                            FUN_10089118(
                                local_12c.as_mut_ptr(),
                                &raw mut local_e4,
                                &raw mut local_e8,
                                &raw mut local_f0,
                            );
                            let mut local_ec = sb(local_e4, local_130);
                            let mut local_f4 = sb(local_e8, local_d8);
                            let mut local_f8 = sb(local_f0, local_e0);
                            FUN_1002f3a7(obj!(0x40), local_ec, local_f4, local_f8);
                            FUN_10089cc9(local_12c.as_mut_ptr(), 0, obj!(0x60), 0, 0, 0, 0);
                            FUN_10089118(
                                local_12c.as_mut_ptr(),
                                &raw mut local_ec,
                                &raw mut local_f4,
                                &raw mut local_f8,
                            );
                            setm!(0x100, local_e8);
                            seto!(0x50, ad(obj!(0x50), local_ec));
                            seto!(0x54, ad(obj!(0x54), local_f4));
                            seto!(0x58, ad(obj!(0x58), local_f8));
                        }
                    } else if (rw(p.wrapping_add(0x442)) & 8) == 0 {
                        let base =
                            mech_trace::gr!("1001f531", "100aaf68", DAT_100aaf68) as usize as i32;
                        let local_134 = sb(obj!(0x50), rd(base)) as u32;
                        let local_138 = sb(obj!(0x54), rd(base.wrapping_add(4)));
                        let local_13c = sb(obj!(0x58), rd(base.wrapping_add(8))) as u32;
                        FUN_100037f0(local_134, local_138, local_13c, 0xee, 0);
                        ww(p.wrapping_add(0x442), rw(p.wrapping_add(0x442)) | 8);
                    }
                }
                if local_1c != 0 {
                    FUN_10063e41(rd(p));
                }
                let local_70: i32 = m!(7) >> 1;
                if m!(0xe5) != 0 {
                    let mut local_18c: i32 = 0;
                    let mut local_190: i32 = 0;
                    let mut local_194: i32 = 0;
                    let mut local_1d4: i32 = 0;
                    if (rw(p.wrapping_add(0x442)) & 4) != 0 {
                        FUN_1002f2d6(
                            rw(p.wrapping_add(0x442)) as usize as *mut c_void,
                            obj!(0x44),
                            &raw mut local_18c,
                            &raw mut local_190,
                            &raw mut local_194,
                        );
                        local_1d4 = m!(7);
                        setm!(7, sb(m!(7), local_18c));
                    }
                    FUN_10063fbd(rd(p), local_1d0.as_mut_ptr());
                    if (rw(p.wrapping_add(0x442)) & 4) != 0 {
                        setm!(7, local_1d4);
                    }
                    let local_198 = mech_trace::cr!("1001f531", "10063f34", FUN_10063f34(rd(p)));
                    FUN_1002650e(local_1d0.as_mut_ptr(), local_198);
                    let mut local_1d8: i32 = 0;
                    let mut local_188: i32 = 0;
                    FUN_1002f322(
                        m!(0xe5),
                        &raw mut local_1d8,
                        local_180.as_mut_ptr().cast::<i32>(),
                        &raw mut local_188,
                    );
                    local_1d8 = sb(local_1d0[3], local_1d8);
                    local_180[0] = sb(local_1d0[4], local_180[0] as i32) as u32;
                    local_188 = sb(local_1d0[5], local_188);
                    let iVar1 = mech_trace::cr!(
                        "1001f531",
                        "1000fc30",
                        FUN_1000fc30(local_1d8, local_180[0] as i32, local_188)
                    );
                    let mut local_184 = mech_trace::cr!(
                        "1001f531",
                        "10022b50",
                        FUN_10022b50(local_180[0] as i32, iVar1)
                    ) as u32;
                    let iVar1 = mech_trace::cr!("1001f531", "10006bbb", FUN_10006bbb(local_184));
                    local_184 = sb(ng(local_70), iVar1) as u32;
                    FUN_1002f668(m!(0xe5), local_184 as i32, 0, 0, 0);
                }
                if m!(0xe4) != 0 {
                    if (rw(p.wrapping_add(0x440)) & 0x2000) == 0 {
                        seto!(0x68, 0);
                    } else {
                        seto!(0x68, local_70);
                    }
                    seto!(0x6c, m!(3));
                    seto!(0x70, 0);
                    FUN_1002f668(m!(0xe4), obj!(0x68), obj!(0x6c), obj!(0x70), 1);
                }
                FUN_1002fbc0(obj!(0x40) as usize as *mut i32);
            }
            let local_5c = obj!(0x60);
            let iVar1 = mech_trace::cr!("1001f531", "10006b20", FUN_10006b20(local_5c));
            seto!(0xbc, iVar1 >> 0xd);
            let iVar1 = mech_trace::cr!("1001f531", "10006b9a", FUN_10006b9a(local_5c));
            seto!(0xb8, iVar1 >> 0xd);
        }
    }
}
