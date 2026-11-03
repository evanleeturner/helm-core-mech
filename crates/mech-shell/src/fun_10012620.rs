// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008eb68: u32;
    static mut DAT_10098214: u32;
    static mut DAT_100a7248: u32;
    static mut DAT_100a724c: u32;
    fn FUN_1000e070(a: i32, b: i32, c: i32, d: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10012270();
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_100133d0(rect_owner: i32);
}

unsafe fn rd(base: i32, off: i32) -> i32 {
    // SAFETY: caller guarantees base+off is a valid readable address.
    unsafe {
        mech_trace::rd!(
            "10012620",
            (base.wrapping_add(off) as *const i32),
            read_unaligned
        )
    }
}

unsafe fn wr(base: i32, off: i32, v: i32) {
    // SAFETY: caller guarantees base+off is a valid writable address.
    unsafe {
        mech_trace::wr!(
            "10012620",
            (base.wrapping_add(off) as *mut i32),
            write_unaligned,
            v
        )
    }
}

unsafe fn slot(table: u32, off: u32) -> usize {
    // SAFETY: table points at the game's function table.
    unsafe {
        mech_trace::rd!(
            "10012620",
            (table.wrapping_add(off) as *const u32),
            read_unaligned
        ) as usize
    }
}

unsafe fn call0(table: u32, off: u32) {
    // SAFETY: the table slot holds a valid cdecl function address.
    unsafe {
        let f: extern "C" fn() = core::mem::transmute(slot(table, off));
        mech_trace::cr!("10012620", "ind", f());
    }
}

unsafe fn call4(table: u32, off: u32, a: i32, b: i32, c: i32, d: i32) {
    // SAFETY: the table slot holds a valid cdecl function address.
    unsafe {
        let f: extern "C" fn(i32, i32, i32, i32) = core::mem::transmute(slot(table, off));
        mech_trace::cr!("10012620", "ind", f(a, b, c, d));
    }
}

unsafe fn copy_bytes(src: i32, dst: i32, count: i32) {
    let mut s = src as *const u8;
    let mut d = dst as *mut u8;
    let mut n = count;
    while n != 0 {
        // SAFETY: the original copies byte by byte between these buffers.
        unsafe {
            mech_trace::dw!("10012620", d, mech_trace::dr!("10012620", s));
            s = s.wrapping_add(1);
            d = d.wrapping_add(1);
        }
        n = n.wrapping_sub(1);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_10012620(rect_owner: i32) {
    mech_trace::enter!("10012620", rect_owner);
    let r = rect_owner;
    // SAFETY: r points at the game's rect owner object; globals and
    // function tables are the game's own memory.
    unsafe {
        if rd(r, 0x82) < rd(r, 0x5a) {
            wr(r, 0x82, rd(r, 0x5a));
        }
        if rd(r, 0x86) < rd(r, 0x5e) {
            wr(r, 0x86, rd(r, 0x5e));
        }
        if rd(r, 0x62) < rd(r, 0x8a) {
            wr(r, 0x8a, rd(r, 0x62));
        }
        if rd(r, 0x66) < rd(r, 0x8e) {
            wr(r, 0x8e, rd(r, 0x66));
        }
        if rd(r, 0x1e) == 0 {
            if rd(r, 0x82) <= rd(r, 0x8a) && rd(r, 0x86) <= rd(r, 0x8e) {
                if mech_trace::gr!("10012620", "10098214", DAT_10098214) == 0 {
                    call0(mech_trace::gr!("10012620", "100a724c", DAT_100a724c), 0x1c);
                } else {
                    mech_trace::cr!(
                        "10012620",
                        "1000e070",
                        FUN_1000e070(rd(r, 0x82), rd(r, 0x86), rd(r, 0x8a), rd(r, 0x8e))
                    );
                }
            }
        } else {
            if rd(r, 0x22) == 0 {
                if rd(r, 0x3aa) == 0 {
                    call4(
                        mech_trace::gr!("10012620", "100a7248", DAT_100a7248),
                        0x14,
                        0,
                        0x100,
                        r.wrapping_add(0x92),
                        0,
                    );
                    call4(
                        mech_trace::gr!("10012620", "100a724c", DAT_100a724c),
                        0x1c,
                        rd(r, 0x5a),
                        rd(r, 0x5e),
                        rd(r, 0x62),
                        rd(r, 0x66),
                    );
                } else {
                    copy_bytes(
                        rd(r, 0x2e),
                        rd(r, 0x42),
                        rd(r, 0x392).wrapping_mul(rd(r, 0x396)),
                    );
                    FUN_100133d0(r);
                    call4(
                        mech_trace::gr!("10012620", "100a7248", DAT_100a7248),
                        0x14,
                        0,
                        0x100,
                        r.wrapping_add(0x92),
                        rd(r, 0x22),
                    );
                    copy_bytes(
                        rd(r, 0x42),
                        rd(r, 0x2e),
                        rd(r, 0x392).wrapping_mul(rd(r, 0x396)),
                    );
                    call0(mech_trace::gr!("10012620", "100a724c", DAT_100a724c), 0x18);
                    wr(r, 0x3aa, 0);
                }
            } else if mech_trace::gr!("10012620", "1008eb68", DAT_1008eb68) == 0 {
                FUN_100133d0(r);
                call4(
                    mech_trace::gr!("10012620", "100a7248", DAT_100a7248),
                    0x14,
                    0,
                    0x100,
                    r.wrapping_add(0x92),
                    rd(r, 0x22),
                );
                copy_bytes(
                    rd(r, 0x42),
                    rd(r, 0x2e),
                    rd(r, 0x392).wrapping_mul(rd(r, 0x396)),
                );
                call0(mech_trace::gr!("10012620", "100a724c", DAT_100a724c), 0x18);
            } else {
                FUN_10012270();
                call0(mech_trace::gr!("10012620", "100a724c", DAT_100a724c), 0x18);
                call4(
                    mech_trace::gr!("10012620", "100a7248", DAT_100a7248),
                    0x14,
                    0,
                    0x100,
                    r.wrapping_add(0x92),
                    rd(r, 0x22),
                );
                call0(mech_trace::gr!("10012620", "100a724c", DAT_100a724c), 0x18);
                mech_trace::gw!("10012620", "1008eb68", DAT_1008eb68, 0);
            }
            wr(r, 0x1e, 0);
        }
        wr(r, 0x82, rd(r, 0x62));
        wr(r, 0x86, rd(r, 0x66));
        wr(r, 0x8a, rd(r, 0x5a));
        wr(r, 0x8e, rd(r, 0x5e));
    }
}
