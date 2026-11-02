// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b378c: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100b3790: i32;
    static mut DAT_100b38fc: i32;
    static mut DAT_100b3904: i32;
    static mut DAT_100ab094: i32;
    static mut DAT_100aaf68: *mut u8;
    static mut DAT_100a7ba0: [u8; 0];
    static mut DAT_100a7ba4: [u8; 0];
    static mut DAT_100a7ba8: [u8; 0];
    static mut DAT_100a7bb8: [u8; 0];
    static mut DAT_100a7bbc: [u8; 0];
    static mut DAT_100a7bc0: [u8; 0];
    static mut DAT_100a7bc8: [u8; 0];
    static mut DAT_100a7bcc: [u8; 0];
    static mut DAT_100a7bd4: [u8; 0];
    static mut DAT_100a7be4: [u8; 0];
    static mut DAT_100a7bf4: [u8; 0];
    static mut DAT_100a7bf8: [u8; 0];
    static mut DAT_100f2ff0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100082c4(a: i32, b: i32);
    fn FUN_10063930(a: i32);
    fn FUN_100636e5(a: *mut i32);
    fn FUN_10063818(a: *mut i32);
    fn FUN_1006312b(a: i32, b: i32) -> i32;
    fn FUN_10044cc4(a: i32, b: *mut i32);
    fn FUN_10003742(a: i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_10062d5f(a: i32, b: i32);
    fn FUN_1004ab11(a: u32, b: i32);
    fn FUN_10003783(a: i32, b: i32, c: u32, d: i32, e: i32, f: i32);
    fn FUN_10003686(a: u32, b: i32, c: u32, d: u32, e: i32, f: i32);
}

unsafe fn ld(a: i32) -> i32 {
    // SAFETY: the address is a game memory location the original reads.
    unsafe { (a as usize as *const i32).read_unaligned() }
}
unsafe fn st(a: i32, v: i32) {
    // SAFETY: the address is a game memory location the original writes.
    unsafe { (a as usize as *mut i32).write_unaligned(v) }
}
unsafe fn ldb(a: i32) -> i8 {
    // SAFETY: the address is a game memory location the original reads.
    unsafe { (a as usize as *const i8).read() }
}
unsafe fn stb(a: i32, v: i8) {
    // SAFETY: the address is a game memory location the original writes.
    unsafe { (a as usize as *mut i8).write(v) }
}
fn a(x: i32, y: i32) -> i32 {
    x.wrapping_add(y)
}
fn s(x: i32, y: i32) -> i32 {
    x.wrapping_sub(y)
}
fn m(x: i32, y: i32) -> i32 {
    x.wrapping_mul(y)
}
unsafe fn obj_addr(p: *mut i32, off: i32) -> i32 {
    // SAFETY: param_1 points at the game's object structure.
    unsafe { a(ld(a(p.read(), 0x4c)), off) }
}
unsafe fn obj(p: *mut i32, off: i32) -> i8 {
    // SAFETY: see obj_addr.
    unsafe { ldb(obj_addr(p, off)) }
}
unsafe fn set_obj(p: *mut i32, off: i32, v: i8) {
    // SAFETY: see obj_addr.
    unsafe { stb(obj_addr(p, off), v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10061d8b(param_1: *mut i32) {
    // SAFETY: param_1 and every address derived from it or from the game's
    // tables are the game's own memory, accessed where the original does.
    unsafe {
        let p1 = param_1 as usize as i32;
        let t_ba0 = (&raw mut DAT_100a7ba0) as usize as i32;
        let t_ba4 = (&raw mut DAT_100a7ba4) as usize as i32;
        let t_ba8 = (&raw mut DAT_100a7ba8) as usize as i32;
        let t_bb8 = (&raw mut DAT_100a7bb8) as usize as i32;
        let t_bbc = (&raw mut DAT_100a7bbc) as usize as i32;
        let t_bc0 = (&raw mut DAT_100a7bc0) as usize as i32;
        let t_bc8 = (&raw mut DAT_100a7bc8) as usize as i32;
        let t_bcc = (&raw mut DAT_100a7bcc) as usize as i32;
        let t_bd4 = (&raw mut DAT_100a7bd4) as usize as i32;
        let t_be4 = (&raw mut DAT_100a7be4) as usize as i32;
        let t_bf4 = (&raw mut DAT_100a7bf4) as usize as i32;
        let t_bf8 = (&raw mut DAT_100a7bf8) as usize as i32;
        let t_ff0 = (&raw mut DAT_100f2ff0) as usize as i32;
        let flags = param_1.add(0x110) as *mut u16;
        let rf = || flags.read_unaligned();

        if param_1.add(0xf5).read() == 2 {
            param_1.add(0xfb).write(param_1.add(0xf8).read());
            if obj(param_1, 0x2c) != 0 {
                if DAT_100b378c == 0 {
                    DAT_100b378c = 1;
                    FUN_100082c4(6, 1);
                } else {
                    DAT_100b378c = 0;
                    FUN_100082c4(7, 1);
                }
            }
            if (rf() & 0x4000) == 0 {
                if param_1.add(0xf8).read() != -1 {
                    let i_var3 = a(m(param_1.add(0xf8).read(), 0x40), param_1.add(0x15).read());
                    let mut i_var4 = m(ld(a(i_var3, 4)), 0x68);
                    if DAT_100b378c == 0
                        && ld(a(param_1.read(), 4)) == DAT_100adf38
                        && obj(param_1, 0x2b) == 0
                    {
                        set_obj(param_1, 0x27, obj(param_1, 0x25));
                        set_obj(param_1, 0x25, 0);
                    }
                    if obj(param_1, 0x27) == 0 {
                        if obj(param_1, 0x2d) == 0 {
                            if obj(param_1, 0x28) == 0 {
                                if obj(param_1, 0x29) == 0 {
                                    if obj(param_1, 0x2a) == 0 {
                                        if obj(param_1, 0x25) == 0 {
                                            if (rf() & 1) != 0 {
                                                flags.write_unaligned(rf() & 0xfffe);
                                                FUN_10062d5f(p1, 0);
                                            }
                                        } else if (rf() & 1) == 0 {
                                            flags.write_unaligned(rf() | 1);
                                            if ld(a(i_var3, 8)) == 1 {
                                                st(a(i_var3, 8), 2);
                                                st(a(i_var3, 0x24), ld(a(t_bcc, i_var4)));
                                                st(a(i_var3, 0x10), 0);
                                                FUN_10044cc4(i_var3, param_1);
                                                if ld(a(t_bc8, i_var4)) == 0 || (rf() & 0x80) == 0 {
                                                    st(a(i_var3, 0x1c), 0);
                                                    st(a(i_var3, 0x18), ld(a(i_var3, 0x1c)));
                                                } else {
                                                    st(
                                                        a(i_var3, 0x18),
                                                        ld(a(param_1.read(), 0xdc)) & 0xff,
                                                    );
                                                    st(
                                                        a(i_var3, 0x1c),
                                                        ld(a(param_1.read(), 0xdc)) & 0xf00,
                                                    );
                                                }
                                            } else {
                                                FUN_10003742(0x15d, 100, 0x40, 5, 0x32);
                                            }
                                        }
                                    } else if (rf() & 1) == 0 {
                                        flags.write_unaligned(rf() | 1);
                                        i_var4 = FUN_1006312b(p1, 2);
                                        if i_var4 != 0 {
                                            FUN_100636e5(param_1);
                                        }
                                    }
                                } else if (rf() & 1) == 0 {
                                    flags.write_unaligned(rf() | 1);
                                    i_var4 = FUN_1006312b(p1, 1);
                                    if i_var4 != 0 {
                                        FUN_100636e5(param_1);
                                    }
                                }
                            } else if (rf() & 1) == 0 {
                                flags.write_unaligned(rf() | 1);
                                i_var4 = FUN_1006312b(p1, 0);
                                if i_var4 != 0 {
                                    FUN_100636e5(param_1);
                                }
                            }
                        } else {
                            if (rf() & 1) == 0 {
                                flags.write_unaligned(rf() | 1);
                                FUN_10063818(param_1);
                            }
                            set_obj(param_1, 0x2d, 0);
                        }
                    } else if (rf() & 1) == 0 {
                        flags.write_unaligned(rf() | 1);
                        FUN_100636e5(param_1);
                    }
                }
            } else {
                FUN_10063930(p1);
            }

            let mut local_20: i32 = 0;
            while local_20 < 0xe {
                let i_var4 = a(param_1.add(0x15).read(), m(local_20, 0x40));
                if -1 < ld(a(i_var4, 4)) {
                    let i_var2 = ld(a(i_var4, 4));
                    let i_var3 = m(i_var2, 0x68);
                    let pvar1 = a(t_ba0, i_var3);
                    if ld(pvar1) != 0 {
                        let f = core::mem::transmute::<i32, unsafe extern "C" fn(i32, *mut i32, i32)>(
                            ld(pvar1),
                        );
                        f(param_1.read(), pvar1 as usize as *mut i32, i_var4);
                    }
                    let i_var6 = ld(a(i_var4, 8));
                    if i_var6 == 0 {
                        if ld(a(t_bf4, i_var3)) <= s(DAT_100b38fc, ld(a(i_var4, 0x10))) {
                            st(a(i_var4, 8), 1);
                            st(a(i_var4, 0x10), 0);
                        }
                    } else if i_var6 == 2 {
                        if ld(a(i_var4, 0x24)) < 1 {
                            st(a(i_var4, 0x10), DAT_100b38fc);
                            st(a(i_var4, 8), 0);
                            st(a(i_var4, 0x24), 0);
                        } else {
                            if ld(a(i_var4, 0x24)) == ld(a(t_bcc, i_var3)) {
                                DAT_100b3790 = 1;
                            }
                            let mut local_18 = a(ld(a(i_var4, 0x10)), DAT_100b3904);
                            'lab: {
                                loop {
                                    let local_24;
                                    loop {
                                        if local_18 < ld(a(t_bf8, i_var3))
                                            || ld(a(i_var4, 0x24)) < 1
                                        {
                                            break 'lab;
                                        }
                                        let idx = ld(a(i_var4, 0x28));
                                        st(
                                            a(param_1.read(), 0x48),
                                            param_1.offset(idx as isize).add(0xe6).read(),
                                        );
                                        let mut l24 = 0;
                                        let fp = ld(a(t_ba4, m(m(i_var2, 0x1a), 4)));
                                        if fp != 0 {
                                            let f = core::mem::transmute::<
                                                i32,
                                                unsafe extern "C" fn(i32, *mut i32, i32) -> i32,
                                            >(
                                                fp
                                            );
                                            l24 = f(
                                                param_1.read(),
                                                pvar1 as usize as *mut i32,
                                                i_var4,
                                            );
                                        }
                                        if l24 != 0 {
                                            local_24 = l24;
                                            break;
                                        }
                                        flags.write_unaligned(rf() & 0xfffe);
                                        st(a(i_var4, 0x24), 0);
                                    }
                                    FUN_1004ab11(ld(a(t_bb8, i_var3)) as u32, param_1.read());
                                    FUN_1004ab11(ld(a(t_bbc, i_var3)) as u32, param_1.read());
                                    st(a(i_var4, 0x24), s(ld(a(i_var4, 0x24)), local_24));
                                    local_18 = s(local_18, ld(a(t_bf8, i_var3)));
                                    st(a(i_var4, 0x10), local_18);
                                    param_1
                                        .add(0xf1)
                                        .write(a(param_1.add(0xf1).read(), ld(a(t_be4, i_var3))));
                                    if ld(a(i_var4, 4)) == 0x1e {
                                        FUN_100082c4(0x23, -1);
                                    }
                                    let fl = ldb(a(t_ba8, i_var3)) as u8;
                                    if ((fl & 1) != 0 && DAT_100b3790 != 0) || (fl & 2) != 0 {
                                        DAT_100b3790 = 0;
                                        if ld(a(param_1.read(), 4)) == DAT_100adf38 {
                                            st(a(t_ff0, m(ld(a(i_var4, 0x30)), 4)), 1);
                                        }
                                        if 0 < ld(a(t_bd4, i_var3)) {
                                            if ld(a(param_1.read(), 4)) == DAT_100adf38
                                                && DAT_100ab094 != 0
                                            {
                                                let local_28 = match ld(a(i_var4, 0x28)) {
                                                    2 | 5 => 0x4f,
                                                    4 | 6 => 0x2f,
                                                    _ => 0x40,
                                                };
                                                if (fl & 2) == 0 {
                                                    FUN_10003783(
                                                        0,
                                                        0,
                                                        ld(a(t_bd4, i_var3)) as u32,
                                                        0x32,
                                                        local_28,
                                                        0x32,
                                                    );
                                                } else {
                                                    FUN_10003783(
                                                        0,
                                                        0,
                                                        ld(a(t_bd4, i_var3)) as u32,
                                                        0x32,
                                                        local_28,
                                                        0x432,
                                                    );
                                                }
                                            } else {
                                                let pd = DAT_100aaf68 as usize as i32;
                                                let u_var5 =
                                                    s(ld(pd), ld(a(param_1.read(), 0x50))) as u32;
                                                let i_var6b =
                                                    s(ld(a(pd, 4)), ld(a(param_1.read(), 0x54)));
                                                let u_var7 =
                                                    s(ld(a(pd, 8)), ld(a(param_1.read(), 0x58)))
                                                        as u32;
                                                if (fl & 2) == 0 {
                                                    FUN_10003686(
                                                        u_var5,
                                                        i_var6b,
                                                        u_var7,
                                                        ld(a(t_bd4, i_var3)) as u32,
                                                        DAT_100ab094,
                                                        0x32,
                                                    );
                                                } else {
                                                    FUN_10003686(
                                                        u_var5,
                                                        i_var6b,
                                                        u_var7,
                                                        ld(a(t_bd4, i_var3)) as u32,
                                                        DAT_100ab094,
                                                        0x432,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    let cont = (ld(a(i_var4, 0x24)) != 0
                                        || ld(a(t_bc0, i_var3)) == 0)
                                        || (obj(param_1, 0x25) == 0
                                            && ((obj(param_1, 0x27) == 0
                                                && obj(param_1, 0x28) == 0)
                                                && obj(param_1, 0x29) == 0
                                                && obj(param_1, 0x2a) == 0));
                                    if !cont {
                                        break;
                                    }
                                }
                                st(a(i_var4, 0x10), DAT_100b38fc);
                                st(a(i_var4, 8), 3);
                                st(a(i_var4, 0x24), ld(a(t_bcc, i_var3)));
                            }
                            if ld(a(i_var4, 8)) != 3 {
                                st(a(i_var4, 0x10), local_18);
                            }
                        }
                    } else if i_var6 == 3
                        && ld(a(t_bf4, i_var3)) <= s(DAT_100b38fc, ld(a(i_var4, 0x10)))
                    {
                        if a(m(param_1.add(0xf8).read(), 0x40), param_1.add(0x15).read()) == i_var4
                            || (obj(param_1, 0x27) != 0 || obj(param_1, 0x28) != 0)
                            || obj(param_1, 0x29) != 0
                            || obj(param_1, 0x2a) != 0
                        {
                            st(a(i_var4, 8), 2);
                        } else {
                            st(a(i_var4, 8), 1);
                        }
                        st(a(i_var4, 0x10), 0);
                    }
                }
                local_20 += 1;
            }
        }
    }
}
