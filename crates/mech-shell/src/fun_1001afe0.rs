// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c49d0: u8;
    static mut DAT_100c48d0: u8;
    static mut DAT_100c47d0: u8;
    static mut DAT_100a672c: u32;
    static mut DAT_1008f900: i32;
    static mut DAT_10083508: u32;
    static mut DAT_100c41c8: u8;
    static mut s_Warning__Could_not_load_RMG_conf_1008fc98: u8;
    static mut DAT_1008fcc0: u8;
    static mut DAT_1008fcc4: u8;
    static mut DAT_1008fcc8: u8;
    static mut DAT_1008f988: u8;
    static mut PTR_DAT_1008fb44: u32;
    static mut DAT_100a6724: *mut i8;
    static mut DAT_100a6734: *mut u8;
    static mut DAT_100a6730: i32;
    static mut DAT_100a6760: u32;
    static mut PTR_DAT_1008f8f8: i32;
    static mut DAT_100a683c: u8;
    static mut DAT_1008fb3c: *mut i32;
    static mut DAT_1008fb40: *mut i32;
    static mut DAT_1008fb10: *mut i32;
    static mut DAT_1008fb14: *mut i8;
    static mut DAT_1008fb18: *mut i8;
    static mut DAT_1008fb1c: *mut i8;
    static mut DAT_1008fb20: *mut i8;
    static mut DAT_1008fb24: *mut i8;
    static mut DAT_1008fb2c: *mut i8;
    static mut DAT_1008fb30: *mut i8;
    static mut DAT_1008fb34: *mut i8;
    static mut DAT_1008fb38: *mut u32;
    static mut s_iaaprev_1008fcd0: u8;
    static mut s_iaanext_1008fcd8: u8;
    static mut s_iaaprev_1008fce0: u8;
    static mut s_iaanext_1008fce8: u8;
    static mut s_iaaexit_1008fcf0: u8;
    static mut s_iaamisp_1008fcf8: u8;
    static mut s_iaalnch_1008fd00: u8;
    static mut s_iaamlab_1008fd08: u8;
    static mut LAB_1001bc10: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1001af40();
    fn FUN_1001add0();
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10012ac0(this: u32, param_1: *mut u8, param_2: i32);
    fn FUN_10010ae0(this: *mut u8, param_1: *mut i8, param_2: *mut u8, param_3: i32) -> *mut i32;
    fn FUN_10010e30(this: *mut i32, param_1: i32);
    fn FUN_1005ee60(
        this: *mut u8,
        param_1: *mut i8,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
        param_6: i32,
    ) -> *mut i8;
    fn FUN_10064610(
        this: *mut u8,
        param_1: u32,
        param_2: u32,
        param_3: u8,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    ) -> *mut u32;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10010d60(param_1: *mut i32);
    fn FUN_10010d80(param_1: *mut i32);
    fn FUN_1005f810(param_1: *mut i8);
}

#[allow(non_snake_case, non_upper_case_globals)]
unsafe extern "C" {
    fn FUN_1000cf60(param_1: *mut u8) -> u32;
    fn FUN_1005b1b0(param_1: *mut i8, param_2: i32);
    fn _time(t: *mut i32) -> i32;
    fn FUN_100706c0(param_1: u32);
    fn FUN_10055d70(param_1: *mut i16) -> u32;
    fn FUN_10037d80(param_1: u32);
    fn FUN_10037e50(param_1: *mut i8);
    fn FUN_100361e0(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_10063ee0(param_1: *mut i32);
    fn FUN_1006f200(param_1: u32) -> *mut u8;
    fn FUN_10016b50(param_1: u32);
    fn FUN_10063d30(param_1: *mut i32);
}

unsafe fn copy_str(src: *const u8, dst: *mut u8) {
    let mut i: usize = 0;
    // SAFETY: src is a NUL-terminated game string and dst is the game's
    // buffer, large enough for it, as in the original.
    unsafe {
        loop {
            let c = mech_trace::dr!("1001afe0", src.add(i));
            mech_trace::dw!("1001afe0", dst.add(i), c);
            i += 1;
            if c == 0 {
                break;
            }
        }
    }
}

unsafe fn zero_buf(b: *mut u8) {
    // SAFETY: b is the game's 243-byte buffer.
    unsafe {
        for i in 0..0x3cusize {
            mech_trace::wr!("1001afe0", b.add(i * 4).cast::<u32>(), write_unaligned, 0);
        }
        mech_trace::wr!("1001afe0", b.add(0xf0).cast::<u16>(), write_unaligned, 0);
        mech_trace::dw!("1001afe0", b.add(0xf2), 0);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001afe0(param_1: *mut u8) -> u32 {
    mech_trace::enter!("1001afe0", param_1);
    // SAFETY: the callees and globals are the game's own; all pointers
    // are used where the original uses them.
    unsafe {
        FUN_1001af40();
        mech_trace::gw!("1001afe0", "100c49d0", DAT_100c49d0, 0xff);
        mech_trace::gw!("1001afe0", "100c48d0", DAT_100c48d0, 0xff);
        mech_trace::gw!("1001afe0", "100c47d0", DAT_100c47d0, 0xff);
        let a = &raw mut DAT_100c47d0;
        let b = &raw mut DAT_100c48d0;
        let c = &raw mut DAT_100c49d0;
        let mut i: i32 = 1;
        loop {
            mech_trace::dw!("1001afe0", a.add(i as usize), i as u8);
            mech_trace::dw!("1001afe0", b.add(i as usize), i as u8);
            mech_trace::dw!("1001afe0", c.add(i as usize), i as u8);
            i += 1;
            if i >= 0x100 {
                break;
            }
        }
        mech_trace::dw!("1001afe0", c.add(1), 0x16);
        mech_trace::dw!("1001afe0", b.add(1), 0x11);
        mech_trace::dw!("1001afe0", a.add(1), 0x9d);
        FUN_10012ac0(
            mech_trace::gr!("1001afe0", "100a672c", DAT_100a672c),
            param_1,
            mech_trace::gr!("1001afe0", "1008f900", DAT_1008f900),
        );

        let buf = &raw mut DAT_100c41c8;
        if mech_trace::rd!(
            "1001afe0",
            ((mech_trace::gr!("1001afe0", "10083508", DAT_10083508).wrapping_add(0x23c3))
                as *const i32),
            read_unaligned
        ) != 0xc031
        {
            let r = mech_trace::cr!("1001afe0", "1000cf60", FUN_1000cf60(buf));
            if (r as u8) == 0 {
                if mech_trace::rd!(
                    "1001afe0",
                    ((mech_trace::gr!("1001afe0", "10083508", DAT_10083508).wrapping_add(0x23c3))
                        as *const i32),
                    read_unaligned
                ) != 0xc02b
                {
                    FUN_1005b1b0(
                        (&raw mut s_Warning__Could_not_load_RMG_conf_1008fc98).cast::<i8>(),
                        0,
                    );
                }
                zero_buf(buf);
            } else {
                zero_buf(buf);
                copy_str((&raw const DAT_1008fcc0), buf.add(0xe));
            }
            if mech_trace::rd!(
                "1001afe0",
                ((mech_trace::gr!("1001afe0", "10083508", DAT_10083508).wrapping_add(0x23c3))
                    as *const i32),
                read_unaligned
            ) == 0xc02b
            {
                copy_str((&raw const DAT_1008fcc4), buf.add(0xe));
            }
        }
        let t = mech_trace::cr!("1001afe0", "_time", _time(core::ptr::null_mut()));
        FUN_100706c0(t as u32);
        mech_trace::wr!("1001afe0", buf.add(6).cast::<u32>(), write_unaligned, 0);
        mech_trace::wr!(
            "1001afe0",
            buf.add(10).cast::<u32>(),
            write_unaligned,
            0xffff_ffff
        );
        mech_trace::wr!("1001afe0", buf.add(2).cast::<u32>(), write_unaligned, 0);
        let u = mech_trace::cr!("1001afe0", "10055d70", FUN_10055d70(buf.cast::<i16>()));
        FUN_10037d80((u as u16 as i16) as i32 as u32);
        FUN_10037e50((&raw mut DAT_1008fcc8).cast::<i8>());
        FUN_100361e0(0, -1, 3, 0, -1);
        FUN_100361e0(1, -1, 3, 0, -1);
        FUN_10063ee0((&raw mut DAT_1008f988).cast::<i32>());
        mech_trace::wr!(
            "1001afe0",
            (mech_trace::gr!("1001afe0", "1008fb44", PTR_DAT_1008fb44).wrapping_add(0x2c)
                as *mut i32),
            write_unaligned,
            1
        );
        FUN_1001add0();

        let mut p = mech_trace::cr!("1001afe0", "1006f200", FUN_1006f200(0x2d));
        let mut pi: *mut i32 = core::ptr::null_mut();
        if !p.is_null() {
            pi = mech_trace::cr!(
                "1001afe0",
                "10010ae0",
                FUN_10010ae0(
                    p,
                    mech_trace::gr!("1001afe0", "100a6724", DAT_100a6724),
                    param_1,
                    0x36
                )
            );
        }
        mech_trace::gw!("1001afe0", "1008fb3c", DAT_1008fb3c, pi);
        FUN_10010e30(pi, 0x1e);
        FUN_10010d60(mech_trace::gr!("1001afe0", "1008fb3c", DAT_1008fb3c));
        FUN_10010d80(mech_trace::gr!("1001afe0", "1008fb3c", DAT_1008fb3c));

        p = mech_trace::cr!("1001afe0", "1006f200", FUN_1006f200(0x2d));
        pi = core::ptr::null_mut();
        if !p.is_null() {
            pi = mech_trace::cr!(
                "1001afe0",
                "10010ae0",
                FUN_10010ae0(
                    p,
                    mech_trace::gr!("1001afe0", "100a6724", DAT_100a6724),
                    mech_trace::gr!("1001afe0", "100a6734", DAT_100a6734),
                    0x34
                )
            );
        }
        mech_trace::gw!("1001afe0", "1008fb40", DAT_1008fb40, pi);
        FUN_10010e30(pi, 0x32);

        p = mech_trace::cr!("1001afe0", "1006f200", FUN_1006f200(0x2d));
        pi = core::ptr::null_mut();
        if !p.is_null() {
            pi = mech_trace::cr!(
                "1001afe0",
                "10010ae0",
                FUN_10010ae0(
                    p,
                    mech_trace::gr!("1001afe0", "100a6724", DAT_100a6724),
                    param_1,
                    0x38
                )
            );
        }
        mech_trace::gw!("1001afe0", "1008fb10", DAT_1008fb10, pi);
        FUN_10010e30(pi, 0x32);

        p = mech_trace::cr!("1001afe0", "1006f200", FUN_1006f200(0x144));
        let mut pc: *mut i8 = core::ptr::null_mut();
        if !p.is_null() {
            pc = mech_trace::cr!(
                "1001afe0",
                "1005ee60",
                FUN_1005ee60(
                    p,
                    (&raw mut s_iaaprev_1008fcd0).cast::<i8>(),
                    0x1b5,
                    0x129,
                    0x21,
                    0,
                    mech_trace::gr!("1001afe0", "100a6730", DAT_100a6730),
                )
            );
        }
        mech_trace::gw!("1001afe0", "1008fb14", DAT_1008fb14, pc);
        FUN_1005f810(pc);

        p = mech_trace::cr!("1001afe0", "1006f200", FUN_1006f200(0x144));
        pc = core::ptr::null_mut();
        if !p.is_null() {
            pc = mech_trace::cr!(
                "1001afe0",
                "1005ee60",
                FUN_1005ee60(
                    p,
                    (&raw mut s_iaanext_1008fcd8).cast::<i8>(),
                    0x210,
                    0x12c,
                    0x21,
                    0,
                    mech_trace::gr!("1001afe0", "100a6730", DAT_100a6730),
                )
            );
        }
        mech_trace::gw!("1001afe0", "1008fb18", DAT_1008fb18, pc);
        FUN_1005f810(pc);

        p = mech_trace::cr!("1001afe0", "1006f200", FUN_1006f200(0x144));
        pc = core::ptr::null_mut();
        if !p.is_null() {
            pc = mech_trace::cr!(
                "1001afe0",
                "1005ee60",
                FUN_1005ee60(
                    p,
                    (&raw mut s_iaaprev_1008fce0).cast::<i8>(),
                    0x1b5,
                    0xd0,
                    0x21,
                    0,
                    mech_trace::gr!("1001afe0", "100a6730", DAT_100a6730),
                )
            );
        }
        mech_trace::gw!("1001afe0", "1008fb1c", DAT_1008fb1c, pc);
        FUN_1005f810(pc);

        p = mech_trace::cr!("1001afe0", "1006f200", FUN_1006f200(0x144));
        pc = core::ptr::null_mut();
        if !p.is_null() {
            pc = mech_trace::cr!(
                "1001afe0",
                "1005ee60",
                FUN_1005ee60(
                    p,
                    (&raw mut s_iaanext_1008fce8).cast::<i8>(),
                    0x210,
                    0xd4,
                    0x21,
                    0,
                    mech_trace::gr!("1001afe0", "100a6730", DAT_100a6730),
                )
            );
        }
        mech_trace::gw!("1001afe0", "1008fb20", DAT_1008fb20, pc);
        FUN_1005f810(pc);

        p = mech_trace::cr!("1001afe0", "1006f200", FUN_1006f200(0x144));
        pc = core::ptr::null_mut();
        if !p.is_null() {
            pc = mech_trace::cr!(
                "1001afe0",
                "1005ee60",
                FUN_1005ee60(
                    p,
                    (&raw mut s_iaaexit_1008fcf0).cast::<i8>(),
                    0x1fe,
                    0x1b8,
                    0x21,
                    0,
                    mech_trace::gr!("1001afe0", "100a6730", DAT_100a6730),
                )
            );
        }
        mech_trace::gw!("1001afe0", "1008fb24", DAT_1008fb24, pc);
        FUN_1005f810(pc);

        p = mech_trace::cr!("1001afe0", "1006f200", FUN_1006f200(0x144));
        pc = core::ptr::null_mut();
        if !p.is_null() {
            pc = mech_trace::cr!(
                "1001afe0",
                "1005ee60",
                FUN_1005ee60(
                    p,
                    (&raw mut s_iaamisp_1008fcf8).cast::<i8>(),
                    0x9d,
                    0x1b8,
                    0x21,
                    0,
                    mech_trace::gr!("1001afe0", "100a6730", DAT_100a6730),
                )
            );
        }
        mech_trace::gw!("1001afe0", "1008fb2c", DAT_1008fb2c, pc);
        FUN_1005f810(pc);

        p = mech_trace::cr!("1001afe0", "1006f200", FUN_1006f200(0x144));
        pc = core::ptr::null_mut();
        if !p.is_null() {
            pc = mech_trace::cr!(
                "1001afe0",
                "1005ee60",
                FUN_1005ee60(
                    p,
                    (&raw mut s_iaalnch_1008fd00).cast::<i8>(),
                    0x1ac,
                    0x189,
                    0x21,
                    0,
                    mech_trace::gr!("1001afe0", "100a6730", DAT_100a6730),
                )
            );
        }
        mech_trace::gw!("1001afe0", "1008fb30", DAT_1008fb30, pc);
        FUN_1005f810(pc);

        p = mech_trace::cr!("1001afe0", "1006f200", FUN_1006f200(0x144));
        pc = core::ptr::null_mut();
        if !p.is_null() {
            pc = mech_trace::cr!(
                "1001afe0",
                "1005ee60",
                FUN_1005ee60(
                    p,
                    (&raw mut s_iaamlab_1008fd08).cast::<i8>(),
                    0x19,
                    0x1b8,
                    0x21,
                    0,
                    mech_trace::gr!("1001afe0", "100a6730", DAT_100a6730),
                )
            );
        }
        mech_trace::gw!("1001afe0", "1008fb34", DAT_1008fb34, pc);
        FUN_1005f810(pc);

        p = mech_trace::cr!("1001afe0", "1006f200", FUN_1006f200(0x111));
        let mut pu: *mut u32 = core::ptr::null_mut();
        if !p.is_null() {
            pu = mech_trace::cr!(
                "1001afe0",
                "10064610",
                FUN_10064610(
                    p,
                    mech_trace::gr!("1001afe0", "100a672c", DAT_100a672c),
                    mech_trace::gr!("1001afe0", "100a6760", DAT_100a6760),
                    0,
                    mech_trace::gr!("1001afe0", "1008f8f8", PTR_DAT_1008f8f8),
                    9,
                    (&raw mut DAT_100a683c) as u32,
                )
            );
        }
        mech_trace::gw!("1001afe0", "1008fb38", DAT_1008fb38, pu);
        FUN_10016b50((&raw mut LAB_1001bc10) as u32);
        FUN_10063d30((&raw mut DAT_1008f988).cast::<i32>());
        mech_trace::rt!("1001afe0", 1)
    }
}
