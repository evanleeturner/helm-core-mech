// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40cc: u32;
    static mut DAT_100a4104: u32;
    static mut DAT_100aa1ac: u32;
    static mut DAT_100ab094: u32;
    static mut DAT_100adf38: u32;
    static mut DAT_100b3218: u32;
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10008323(param_1: i32) -> u8;
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16) -> u8;
    fn FUN_100037f0(param_1: u32, param_2: u32, param_3: u32, param_4: u32, param_5: i32) -> u8;
    fn FUN_10030052(param_1: i32, param_2: u32) -> i32;
    fn FUN_1002f28d(param_1: i32) -> u32;
    fn FUN_1001cac9(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u32;
    fn FUN_1004aa97(
        param_1: i32,
        param_2: u32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: u32,
        param_8: i32,
    ) -> u8;
    fn FUN_10005207(param_1: i32, param_2: *mut i32, param_3: u32) -> u8;
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_10004b19(param_1: i32, param_2: *mut i32) -> u8;
    fn FUN_1005d650(param_1: *mut i32, param_2: i32) -> u8;
}

unsafe fn l32(a: i32) -> i32 {
    // SAFETY: the address is a game structure field.
    unsafe { (a as *const i32).read_unaligned() }
}

unsafe fn l16s(a: i32) -> i32 {
    // SAFETY: the address is a game structure field.
    unsafe { (a as *const i16).read_unaligned() as i32 }
}

unsafe fn l16u(a: i32) -> u32 {
    // SAFETY: the address is a game structure field.
    unsafe { (a as *const u16).read_unaligned() as u32 }
}

unsafe fn s32(a: i32, v: i32) {
    // SAFETY: the address is a game structure field.
    unsafe { (a as *mut i32).write_unaligned(v) }
}

unsafe fn s16(a: i32, v: i32) {
    // SAFETY: the address is a game structure field.
    unsafe { (a as *mut i16).write_unaligned(v as i16) }
}

fn pw(b: i32, k: i32) -> i32 {
    b.wrapping_add(k.wrapping_mul(4))
}

unsafe fn me(b: i32) -> bool {
    // SAFETY: param_2[0] points at a game object; the global is the game's own.
    unsafe { (l32(l32(b).wrapping_add(4)) as u32) == DAT_100adf38 }
}

unsafe fn snd(b: i32, param_5: i32, id: i32) {
    // SAFETY: game structures, globals and functions as in the original.
    unsafe {
        if param_5 == 0 && me(b) {
            if l32(pw(b, 0xf5)) == 2 {
                FUN_10003742(0xe1, 100, 0x40, 5, 0x50);
            }
            FUN_10008323(id);
        }
    }
}

unsafe fn dec_f9(b: i32) {
    // SAFETY: game structure field.
    unsafe {
        s32(pw(b, 0xf9), l32(pw(b, 0xf9)).wrapping_add(-0x199a));
        if l32(pw(b, 0xf9)) < 0 {
            s32(pw(b, 0xf9), 0);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10005624(
    param_1: i32,
    param_2: *mut i32,
    param_3: u32,
    param_4: i32,
    param_5: i32,
) {
    let b = param_2 as i32;
    // SAFETY: param_2 points at the game's own structures; offsets follow the original.
    unsafe {
        let i3 = l32(pw(b, 0x16)).wrapping_add((param_3 as i32).wrapping_mul(0x28));
        if i3.wrapping_add(-0x28) == 0 {
            return;
        }
        if l16s(i3.wrapping_add(-4)) <= param_4 {
            return;
        }
        if param_4 < 0 {
            return;
        }
        let v: u32 = l16u(i3.wrapping_add(-0x1c).wrapping_add(param_4.wrapping_mul(2)));
        let mut u1: u32 = v / 100;
        if u1 < 0x42 {
            let mut q = l32(pw(b, 0x15));
            let mut i = 0;
            while i < 0xe {
                if l32(q.wrapping_add(4)) >= 0 && (l32(q.wrapping_add(0x2c)) as u32) == v {
                    if param_5 == 0 && me(b) && l32(q) != 0 {
                        match u1 {
                            0..=3 => {
                                FUN_10008323(0x16);
                            }
                            4..=6 => {
                                FUN_10008323(0x17);
                            }
                            7..=9 => {
                                FUN_10008323(0x18);
                            }
                            10 => {
                                FUN_10008323(0x19);
                            }
                            0xb => {
                                FUN_10008323(0x1a);
                            }
                            0xc..=0xf => {
                                FUN_10008323(0x1b);
                            }
                            0x10..=0x13 => {
                                FUN_10008323(0x1c);
                            }
                            0x14 => {
                                FUN_10008323(0);
                            }
                            0x15 => {
                                FUN_10008323(0x1d);
                            }
                            0x16 => {
                                FUN_10008323(0x1e);
                            }
                            0x17 => {
                                FUN_10008323(0x1f);
                            }
                            0x18 => {
                                FUN_10008323(0x20);
                            }
                            0x19 => {
                                FUN_10008323(0x21);
                            }
                            0x1a => {
                                FUN_10008323(0x22);
                            }
                            0x1b => {
                                FUN_10008323(0x23);
                            }
                            0x1d => {
                                FUN_10008323(3);
                            }
                            _ => {}
                        }
                        if l32(pw(b, 0xf5)) == 2 {
                            FUN_10003742(0xe1, 100, 0x40, 5, 0x50);
                        }
                        FUN_10008323(0);
                    }
                    s32(q, 0);
                    s32(q.wrapping_add(8), -1);
                    s32(q.wrapping_add(0x38), 1);
                    let mut l40 = l32(pw(b, 0x16));
                    let mut l1c = 0;
                    while l1c < 8 {
                        let mut l34 = 0;
                        while l34 < l16s(l40.wrapping_add(0x24)) {
                            if l16u(l40.wrapping_add(0xc).wrapping_add(l34 * 2)) == v {
                                let mut l44 = l34;
                                while l44 < l16s(l40.wrapping_add(0x24)).wrapping_add(-1) {
                                    if l44 < 0xb {
                                        s16(
                                            l40.wrapping_add(0xc).wrapping_add(l44 * 2),
                                            l16u(l40.wrapping_add(0xe).wrapping_add(l44 * 2))
                                                as i32,
                                        );
                                    }
                                    l44 += 1;
                                }
                                s16(l40.wrapping_add(0xc).wrapping_add(l44 * 2), 0);
                                s16(
                                    l40.wrapping_add(0x24),
                                    l16s(l40.wrapping_add(0x24)).wrapping_add(-1),
                                );
                            } else {
                                l34 += 1;
                            }
                        }
                        l40 = l40.wrapping_add(0x28);
                        l1c += 1;
                    }
                    return;
                }
                q = q.wrapping_add(0x40);
                i += 1;
            }
        } else if u1 > 100 {
            let mut n = l32(pw(b, 0x17));
            let mut i = 0;
            while i < l32(pw(b, 0xff)) {
                if (l16s(n.wrapping_add(4)) as u32) == v {
                    if l32(n.wrapping_add(0x12)) < l32(pw(b, 0x18)) || l16s(n.wrapping_add(2)) < 1 {
                        break;
                    }
                    let i2 = l32(n.wrapping_add(0x12));
                    let base = b.wrapping_add(i2.wrapping_mul(0x3a));
                    if l32(base.wrapping_add(0x66)) < 1 {
                        break;
                    }
                    if l32(n.wrapping_add(0x12)) < l32(pw(b, 0x18)) {
                        let a = base.wrapping_add(0x66);
                        s32(a, l32(a).wrapping_sub(l16s(n.wrapping_add(2))));
                        if l32(base.wrapping_add(0x66)) == 0 {
                            let mut q = l32(pw(b, 0x15));
                            let mut k = 0;
                            while k < 0xe {
                                if l32(q.wrapping_add(4)) >= 0
                                    && l32(q.wrapping_add(0x20)) == l32(n.wrapping_add(0x12))
                                {
                                    s32(q, 0);
                                    s32(q.wrapping_add(8), -1);
                                }
                                q = q.wrapping_add(0x40);
                                k += 1;
                            }
                        }
                    }
                    if param_5 == 0 && l16s(n.wrapping_add(2)) != 0 {
                        let mut x20: u32 = 0;
                        let mut x2c: i32 = 0;
                        let mut x38: u32 = 0;
                        if me(b) {
                            if l32(pw(b, 0xf5)) == 2 {
                                FUN_10003742(0xe1, 100, 0x40, 5, 0x50);
                            }
                            let g = DAT_100aaf68 as i32;
                            x20 = l32(l32(b).wrapping_add(0x50)).wrapping_sub(l32(g)) as u32;
                            x2c =
                                l32(l32(b).wrapping_add(0x54)).wrapping_sub(l32(g.wrapping_add(4)));
                            x38 = l32(l32(b).wrapping_add(0x58))
                                .wrapping_sub(l32(g.wrapping_add(8)))
                                as u32;
                            DAT_100b3218 = 1;
                            FUN_100037f0(x20, x2c as u32, x38, 0xcd, DAT_100ab094 as i32);
                            FUN_10008323(1);
                        }
                        let mut l3c = FUN_10030052(l32(l32(b).wrapping_add(0x40)), param_3);
                        if l3c == 0 {
                            l3c = FUN_10030052(l32(l32(b).wrapping_add(0x40)), 3);
                        }
                        if l3c != 0 {
                            let p6 = &raw mut x38;
                            let p5 = &raw mut x2c;
                            let p4 = &raw mut x20;
                            let r = FUN_1002f28d(l3c) as i32;
                            FUN_1001cac9(r, p4, p5.cast::<u32>(), p6);
                            FUN_1004aa97(
                                param_1, 7, x20 as i32, x2c, x38 as i32, x20 as i32, x2c as u32,
                                x38 as i32,
                            );
                        }
                        let a20 = i3.wrapping_add(-0x20);
                        s32(
                            a20,
                            l32(a20).wrapping_add(
                                l16s(n.wrapping_add(2))
                                    .wrapping_mul(l16s(n.wrapping_add(8)))
                                    .wrapping_mul(-0x10000),
                            ),
                        );
                        if l32(a20) < 0 {
                            s32(i3.wrapping_add(-0x24), 0);
                            s32(i3.wrapping_add(-0x28), l32(i3.wrapping_add(-0x24)));
                            s32(a20, 0);
                            s16(
                                i3.wrapping_add(-0x1c).wrapping_add(param_4.wrapping_mul(2)),
                                0,
                            );
                            FUN_10005207(param_1, param_2, param_3);
                            if (l16u(i3.wrapping_add(-2)) & 0x100) == 0 {
                                let w = l32(a20) as u32;
                                let sg = ((w as i32) >> 31) as u32;
                                let l8 = (w ^ sg).wrapping_sub(sg) as i32;
                                let l14: u32 = match param_3 {
                                    2 => 3,
                                    3 => 0,
                                    4 => 3,
                                    5 => 2,
                                    6 => 4,
                                    7 => 2,
                                    8 => 4,
                                    _ => 0,
                                };
                                if l14 != 0 {
                                    let i2b = l32(pw(b, 0x16))
                                        .wrapping_add((l14 as i32).wrapping_mul(0x28));
                                    let a = i2b.wrapping_add(-0x20);
                                    s32(a, l32(a).wrapping_sub(l8));
                                    let i7 = 0;
                                    let r = FUN_10051f43(l16s(i2b.wrapping_add(-4)));
                                    FUN_10005624(param_1, param_2, l14, r, i7);
                                }
                            }
                        }
                        if DAT_100a4104 != 0 && (me(b) || DAT_100aa1ac == 0) {
                            s32(pw(b, 0xf5), 5);
                            FUN_10004b19(param_1, param_2);
                        }
                    }
                    s16(n.wrapping_add(2), 0);
                }
                n = n.wrapping_add(0x1e);
                i += 1;
            }
        } else {
            u1 = (v / 10).wrapping_mul(10);
            if u1 < 0x1cb7 {
                if u1 == 0x1cb6 {
                    snd(b, param_5, 0xc);
                    FUN_1005d650(param_2, 0);
                } else if u1 == 0x1c84 {
                    snd(b, param_5, 9);
                    FUN_1005d650(param_2, 0);
                }
            } else if u1 < 0x1d1b {
                if u1 == 0x1d1a || u1 == 0x1ce8 {
                    snd(b, param_5, 0xc);
                    FUN_1005d650(param_2, 0);
                }
            } else if u1 < 0x1d7f {
                if u1 == 0x1d7e {
                    snd(b, param_5, 10);
                    dec_f9(b);
                } else if u1 == 0x1d4c {
                    snd(b, param_5, 0xb);
                    dec_f9(b);
                }
            } else if u1 < 0x1de3 {
                if u1 == 0x1de2 || u1 == 0x1db0 {
                    snd(b, param_5, 10);
                    dec_f9(b);
                }
            } else if u1 < 0x1e47 {
                if u1 == 0x1e46 {
                    if me(b) || DAT_100aa1ac == 0 {
                        FUN_10004b19(param_1, param_2);
                    }
                } else if u1 == 0x1e14 {
                    snd(b, param_5, 9);
                    FUN_1005d650(param_2, 1);
                }
            } else if u1 < 0x1eab {
                if u1 == 0x1eaa {
                    snd(b, param_5, 7);
                    dec_f9(b);
                } else if u1 == 0x1e78 {
                    snd(b, param_5, 8);
                    dec_f9(b);
                    s32(pw(b, 0xfd), -2);
                    s32(pw(b, 0x108), 0);
                    s32(pw(b, 0xfe), 0);
                }
            } else if u1 < 0x1f41 {
                if u1 == 8000 {
                    if param_5 == 0 && me(b) && l32(pw(b, 0xf4)) != 0 {
                        if l32(pw(b, 0xf5)) == 2 {
                            FUN_10003742(0xe1, 100, 0x40, 5, 0x50);
                        }
                        FUN_10008323(5);
                    }
                    if l32(pw(b, 0xf4)) < 1 {
                        s32(pw(b, 0xf4), 0);
                    } else {
                        s32(pw(b, 0xf4), l32(pw(b, 0xf4)).wrapping_add(-0x32));
                    }
                } else if u1 == 0x1edc {
                    snd(b, param_5, 6);
                    if DAT_100a40cc != 0 && (me(b) || DAT_100aa1ac == 0) {
                        FUN_10004b19(param_1, param_2);
                    }
                }
            } else if u1 == 0x2134 {
                if param_5 == 0 && me(b) && l32(pw(b, 0xfd)) != 2 {
                    if l32(pw(b, 0xf5)) == 2 {
                        FUN_10003742(0xe1, 100, 0x40, 5, 0x50);
                    }
                    FUN_10008323(4);
                }
                if l32(pw(b, 0xfe)) < 1 {
                    s32(pw(b, 0x108), 0);
                } else {
                    let d = l32(pw(b, 0x108)) / l32(pw(b, 0xfe));
                    s32(pw(b, 0x108), l32(pw(b, 0x108)).wrapping_sub(d));
                }
                if 0 < l32(pw(b, 0xfe)) {
                    s32(pw(b, 0xfe), l32(pw(b, 0xfe)).wrapping_add(-1));
                }
                if l32(pw(b, 0xfe)) == 0 {
                    s32(pw(b, 0xfd), -2);
                }
            } else if u1 == 9000 {
                if param_5 == 0 {
                    let r = FUN_10051f43(l16s(i3.wrapping_add(-4)));
                    FUN_10005624(param_1, param_2, param_3, r, 1);
                    return;
                }
            } else if u1 == 0x251c && param_5 == 0 {
                let r = FUN_10051f43(l16s(i3.wrapping_add(-4)));
                FUN_10005624(param_1, param_2, param_3, r, 1);
                return;
            }
            if l32(pw(b, 0xf9)) != 0 && l32(pw(b, 0xf9)) < 0x6666 {
                s32(pw(b, 0xf9), 0x6666);
            }
        }
        let mut l30 = param_4;
        while l30 < l16s(i3.wrapping_add(-4)).wrapping_add(-1) {
            s16(
                i3.wrapping_add(-0x1c).wrapping_add(l30.wrapping_mul(2)),
                l16u(i3.wrapping_add(-0x1a).wrapping_add(l30.wrapping_mul(2))) as i32,
            );
            l30 += 1;
        }
        s16(i3.wrapping_add(-0x1c).wrapping_add(l30.wrapping_mul(2)), 0);
        s16(
            i3.wrapping_add(-4),
            l16s(i3.wrapping_add(-4)).wrapping_add(-1),
        );
    }
}
