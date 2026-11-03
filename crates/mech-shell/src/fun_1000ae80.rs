// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0330: i32;
    static mut DAT_100b0334: i32;
    static mut DAT_100b0338: i32;
    static mut DAT_100b033c: i32;
    static mut DAT_100b0340: i32;
    static mut DAT_100b0344: i32;
    static mut DAT_100b0348: u8;
    static mut DAT_100b0260: u32;
    static mut DAT_100b0264: i32;
    static mut DAT_100b0388: u32;
    static mut DAT_100b0418: u32;
    static mut DAT_100b041c: u32;
    static mut DAT_100b0420: u32;
    static mut DAT_100b0424: u16;
    static mut DAT_100b043c: i16;
    static mut DAT_100b043e: u16;
    static mut DAT_100b0440: u32;
    static mut DAT_100b0444: u32;
    static mut DAT_100b0448: u32;
    static mut DAT_100b0466: u16;
    static mut DAT_100b0468: u32;
    static mut DAT_100b046c: u32;
    static mut DAT_100b0470: u32;
    static mut DAT_100b048e: u16;
    static mut DAT_100b0490: u32;
    static mut DAT_100b0494: u32;
    static mut DAT_100b0498: u32;
    static mut DAT_100b04b6: u16;
    static mut DAT_100b04b8: u32;
    static mut DAT_100b04c0: u32;
    static mut DAT_100b04de: u16;
    static mut DAT_100b04e0: u32;
    static mut DAT_100b04e8: u32;
    static mut DAT_100b0506: u16;
    static mut DAT_100b0508: u32;
    static mut DAT_100b0510: u32;
    static mut DAT_100b052e: u16;
    static mut DAT_100b0530: u32;
    static mut DAT_100b0538: u32;
    static mut DAT_100b0556: u16;
    static mut DAT_10098224: u32;
    static mut DAT_10084a68: u32;
    static mut DAT_10084a6c: u32;
    static mut DAT_10084a70: u32;
    static mut DAT_10084a74: u32;
    static mut DAT_1007f038: f64;
    static mut DAT_1007f058: f64;
    static mut DAT_1007f060: f64;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10009e80(param_1: *mut i8) -> *mut u32;
    fn FUN_1000a030(param_1: *mut u32, param_2: *mut u32, param_3: u32) -> *mut u32;
    fn _strncpy(dest: *mut i8, source: *mut i8, count: u32) -> *mut i8;
    fn FUN_10009840(param_1: *mut i8) -> *mut i8;
    fn FUN_1000a060(param_1: *mut i8);
    fn FUN_1000b7f0(param_1: i32, param_2: i32) -> i32;
    fn FUN_1000b8f0(param_1: u32, param_2: u32, param_3: i32, param_4: *mut i32) -> u32;
    fn FUN_1000b940(param_1: u32, param_2: u32, param_3: *mut i32, param_4: *mut i32) -> u32;
    fn FUN_1000a160(param_1: i32) -> i32;
    fn FUN_1000a790(param_1: i32);
    fn FUN_1000a800(param_1: i32);
    fn FUN_1000a8c0(param_1: i32);
    fn FUN_1000aa80(param_1: i32);
    fn FUN_1000a890(param_1: i32);
    fn FUN_1000a6f0(param_1: i32);
    fn FUN_1000d650(param_1: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000be10() -> u32;
    fn HeapAlloc(h: u32, flags: u32, size: u32) -> *mut u8;
}

unsafe fn r(b: *mut u8, off: usize) -> i32 {
    // SAFETY: b points at the allocated 0x853-byte block; offset is in range.
    unsafe { mech_trace::rd!("1000ae80", b.byte_add(off).cast::<i32>(), read_unaligned) }
}

unsafe fn w(b: *mut u8, off: usize, v: i32) {
    // SAFETY: b points at the allocated 0x853-byte block; offset is in range.
    unsafe {
        mech_trace::wr!(
            "1000ae80",
            b.byte_add(off).cast::<i32>(),
            write_unaligned,
            v
        )
    }
}

unsafe fn w16(b: *mut u8, off: usize, v: u16) {
    // SAFETY: b points at the allocated 0x853-byte block; offset is in range.
    unsafe {
        mech_trace::wr!(
            "1000ae80",
            b.byte_add(off).cast::<u16>(),
            write_unaligned,
            v
        )
    }
}

unsafe fn rg(base: *const u8, off: isize) -> i32 {
    // SAFETY: base is a game global table; the offset is where the original reads.
    unsafe {
        mech_trace::rd!(
            "1000ae80",
            base.byte_offset(off).cast::<i32>(),
            read_unaligned
        )
    }
}

unsafe fn copy_str(mut dst: *mut u8, mut src: *const u8) {
    // SAFETY: src is a NUL-terminated string and dst has room for it.
    unsafe {
        loop {
            let c = mech_trace::dr!("1000ae80", src);
            mech_trace::dw!("1000ae80", dst, c);
            if c == 0 {
                break;
            }
            src = src.add(1);
            dst = dst.add(1);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000ae80(param_1: *mut i8) -> *mut i8 {
    mech_trace::enter!("1000ae80", param_1);
    // SAFETY: the globals and callees are the game's own; the block from
    // HeapAlloc is 0x853 bytes and every offset written lies inside it.
    unsafe {
        let mut p = mech_trace::cr!("1000ae80", "10009e80", FUN_10009e80(param_1));
        if p.is_null() {
            return mech_trace::rt!("1000ae80", core::ptr::null_mut());
        }
        (&raw mut DAT_100b0330).cast::<u8>().write_bytes(0, 24);
        (&raw mut DAT_100b0418).cast::<u8>().write_bytes(0, 0x140);
        (&raw mut DAT_100b0388).cast::<u8>().write_bytes(0, 0x70);
        (&raw mut DAT_100b0260).cast::<u8>().write_bytes(0, 0xc8);
        (&raw mut DAT_100b0348).write_bytes(0, 58);

        p = mech_trace::cr!(
            "1000ae80",
            "1000a030",
            FUN_1000a030((&raw mut DAT_100b0330).cast::<u32>(), p, 0x18)
        );
        p = mech_trace::cr!(
            "1000ae80",
            "1000a030",
            FUN_1000a030((&raw mut DAT_100b0418).cast::<u32>(), p, 0x140)
        );
        let mut n = mech_trace::gr!("1000ae80", "100b0340", DAT_100b0340);
        if n >= 0xe {
            n = 0xe;
        }
        mech_trace::cr!(
            "1000ae80",
            "1000a030",
            FUN_1000a030(&raw mut DAT_100b0388, p, n.wrapping_shl(3) as u32)
        );
        p = p.byte_offset(
            mech_trace::gr!("1000ae80", "100b0340", DAT_100b0340).wrapping_mul(8) as isize,
        );
        let mut n = mech_trace::gr!("1000ae80", "100b0344", DAT_100b0344);
        if n >= 0x19 {
            n = 0x19;
        }
        mech_trace::cr!(
            "1000ae80",
            "1000a030",
            FUN_1000a030(&raw mut DAT_100b0260, p, n.wrapping_shl(3) as u32)
        );
        mech_trace::cr!(
            "1000ae80",
            "1000a030",
            FUN_1000a030(
                (&raw mut DAT_100b0348).cast::<u32>(),
                p.byte_offset(
                    mech_trace::gr!("1000ae80", "100b0344", DAT_100b0344).wrapping_mul(8) as isize
                ),
                0x3a,
            )
        );

        let b = mech_trace::cr!(
            "1000ae80",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("1000ae80", "10098224", DAT_10098224),
                1,
                0x853
            )
        );
        b.write_bytes(0, 0x853);
        copy_str(b.add(0x823), param_1.cast::<u8>());
        let s = b.add(0x81c).cast::<i8>();
        mech_trace::cr!("1000ae80", "_strncpy", _strncpy(s, param_1, 3));
        let t = mech_trace::cr!("1000ae80", "10009840", FUN_10009840(s));
        copy_str(b, t.cast::<u8>());
        FUN_1000a060(param_1);

        let mut i: usize = 0;
        while i < 0x4e {
            w(b, i * 8 + 0x4f0, -1);
            i += 1;
        }

        let t388 = (&raw mut DAT_100b0388).cast::<u8>();
        let mut i: i32 = 0;
        loop {
            if mech_trace::gr!("1000ae80", "100b0340", DAT_100b0340) > i {
                let o = (i as usize).wrapping_mul(8);
                w(b, o + 0x2f0, rg(t388, o as isize));
                w(b, o + 0x2f4, rg(t388, o as isize + 4));
            } else {
                w(b, (i as usize).wrapping_mul(8) + 0x2f0, -1);
            }
            i += 1;
            if !(i < 0xe) {
                break;
            }
        }

        let mut i: usize = 0;
        while i < 0x19 {
            w(b, i * 4 + 0x28c, -1);
            i += 1;
        }

        let t264 = (&raw mut DAT_100b0264).cast::<u8>();
        let mut i: i32 = 0;
        if mech_trace::gr!("1000ae80", "100b0344", DAT_100b0344) > 0 {
            loop {
                let v = rg(t264, (i as isize) * 8);
                mech_trace::cr!(
                    "1000ae80",
                    "1000b7f0",
                    FUN_1000b7f0(b as i32, v.wrapping_mul(100).wrapping_add(0x2711))
                );
                w(b, 0x268, r(b, 0x268).wrapping_add(100));
                i += 1;
                if !(mech_trace::gr!("1000ae80", "100b0344", DAT_100b0344) > i) {
                    break;
                }
            }
        }

        let t43c = (&raw mut DAT_100b043c).cast::<u8>();
        let t424 = (&raw mut DAT_100b0424).cast::<u8>();
        let t260 = (&raw mut DAT_100b0260).cast::<u8>();
        let mut ci: i32 = 0;
        loop {
            let mut cj: i32 = 0;
            loop {
                let cnt = mech_trace::rd!(
                    "1000ae80",
                    t43c.byte_offset(ci as isize * 40).cast::<i16>(),
                    read_unaligned
                ) as i32;
                let dst = ((cj + ci * 12) as usize).wrapping_mul(4) + 0x368;
                if cnt > cj {
                    let mut v = mech_trace::rd!(
                        "1000ae80",
                        t424.byte_offset((ci as isize * 20 + cj as isize) * 2)
                            .cast::<u16>(),
                        read_unaligned
                    ) as u32 as i32;
                    if v >= 0x2710 {
                        let mut k: usize = 0;
                        while k < 25 {
                            if rg(t260, k as isize * 8) == v {
                                v = r(b, k * 4 + 0x28c);
                                break;
                            }
                            k += 1;
                        }
                    }
                    w(b, dst, v);
                } else {
                    w(b, dst, -1);
                }
                cj += 1;
                if !(cj < 0xc) {
                    break;
                }
            }
            ci += 1;
            if !(ci < 8) {
                break;
            }
        }

        w(b, 0x814, 0);
        copy_str(b.add(0x100), (&raw const DAT_100b0348).cast::<u8>());
        w(
            b,
            0x200,
            mech_trace::gr!("1000ae80", "100b0330", DAT_100b0330).wrapping_mul(100),
        );
        w(
            b,
            0x280,
            mech_trace::gr!("1000ae80", "100b0334", DAT_100b0334),
        );
        w(
            b,
            0x288,
            mech_trace::gr!("1000ae80", "100b0338", DAT_100b0338),
        );
        w(
            b,
            0x22c,
            mech_trace::gr!("1000ae80", "100b033c", DAT_100b033c)
                .wrapping_mul(4)
                .wrapping_sub(0x28)
                .wrapping_mul(0x19),
        );
        let m = mech_trace::gr!("1000ae80", "100b0334", DAT_100b0334)
            .wrapping_mul(mech_trace::gr!("1000ae80", "100b0330", DAT_100b0330));
        let iv = m.wrapping_mul(5).wrapping_add(4) / 5;
        w(b, 0x210, iv);
        if iv < 10 {
            w(b, 0x210, 10);
        }
        if r(b, 0x210) > 0x190 {
            w(b, 0x210, 0x190);
        }
        w(b, 0x20c, r(b, 0x210).wrapping_sub(10) / 5);
        if mech_trace::cr!(
            "1000ae80",
            "1000b8f0",
            FUN_1000b8f0(b as u32, 0x1eaa, 1, core::ptr::null_mut())
        ) != 0
        {
            w(b, 0x20c, r(b, 0x20c).wrapping_add(10000));
        }
        if mech_trace::cr!(
            "1000ae80",
            "1000b940",
            FUN_1000b940(
                b as u32,
                0x2329,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        ) != 0
        {
            w(b, 0x240, 1);
        }
        if mech_trace::cr!(
            "1000ae80",
            "1000b940",
            FUN_1000b940(
                b as u32,
                0x251d,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        ) != 0
        {
            w(b, 0x24c, 1);
        }
        w(b, 0x258, 0);
        let t418 = (&raw mut DAT_100b0418).cast::<u8>();
        let mut i: isize = 0;
        while i < 8 {
            let s = rg(t418, i * 40 + 4).wrapping_add(rg(t418, i * 40));
            w(b, 0x258, r(b, 0x258).wrapping_add(s));
            i += 1;
        }
        let mut fv = r(b, 0x258) as f64;
        if r(b, 0x24c) == 1 {
            fv *= mech_trace::gr!("1000ae80", "1007f058", DAT_1007f058);
        } else {
            fv *= mech_trace::gr!("1000ae80", "1007f060", DAT_1007f060);
        }
        fv *= mech_trace::gr!("1000ae80", "1007f038", DAT_1007f038);
        w(b, 0x250, (fv as i64) as i32);
        let h = mech_trace::cr!(
            "1000ae80",
            "1000a160",
            FUN_1000a160(r(b, 0x250).wrapping_add(r(b, 0x250)))
        );
        w(b, 0x250, h / 2);
        w(b, 0x25c, r(b, 0x258));
        let lc = (r(b, 0x200) / 100).wrapping_sub(10) / 5;

        let g = |base: *const u32| -> i32 {
            mech_trace::rd!(
                "1000ae80",
                base.byte_offset(lc.wrapping_mul(16) as isize),
                read_unaligned
            ) as i32
        };
        w(b, 0x760, 3);
        w(b, 0x772, g(&raw const DAT_10084a6c));
        w(b, 0x784, g(&raw const DAT_10084a68));
        w(b, 0x796, g(&raw const DAT_10084a6c));
        w(b, 0x7a8, g(&raw const DAT_10084a70));
        w(b, 0x7ba, g(&raw const DAT_10084a70));
        w(b, 0x7cc, g(&raw const DAT_10084a74));
        w(b, 0x7de, g(&raw const DAT_10084a74));
        w(b, 0x764, 9);
        w(b, 0x776, r(b, 0x772).wrapping_add(r(b, 0x772)));
        w(b, 0x788, r(b, 0x784).wrapping_add(r(b, 0x784)));
        w(b, 0x79a, r(b, 0x796).wrapping_add(r(b, 0x796)));
        w(b, 0x7ac, r(b, 0x7a8).wrapping_add(r(b, 0x7a8)));
        w(b, 0x7be, r(b, 0x7ba).wrapping_add(r(b, 0x7ba)));
        w(b, 0x7d0, r(b, 0x7cc).wrapping_add(r(b, 0x7cc)));
        w(b, 0x7e2, r(b, 0x7de).wrapping_add(r(b, 0x7de)));
        w(
            b,
            0x760,
            mech_trace::gr!("1000ae80", "100b0420", DAT_100b0420) as i32,
        );
        w(
            b,
            0x772,
            mech_trace::gr!("1000ae80", "100b0448", DAT_100b0448) as i32,
        );
        w(
            b,
            0x784,
            mech_trace::gr!("1000ae80", "100b0470", DAT_100b0470) as i32,
        );
        w(
            b,
            0x796,
            mech_trace::gr!("1000ae80", "100b0498", DAT_100b0498) as i32,
        );
        w(
            b,
            0x7a8,
            mech_trace::gr!("1000ae80", "100b04c0", DAT_100b04c0) as i32,
        );
        w(
            b,
            0x7ba,
            mech_trace::gr!("1000ae80", "100b04e8", DAT_100b04e8) as i32,
        );
        w(
            b,
            0x7cc,
            mech_trace::gr!("1000ae80", "100b0510", DAT_100b0510) as i32,
        );
        w(
            b,
            0x7de,
            mech_trace::gr!("1000ae80", "100b0538", DAT_100b0538) as i32,
        );
        w(
            b,
            0x768,
            mech_trace::gr!("1000ae80", "100b0418", DAT_100b0418) as i32,
        );
        w(
            b,
            0x77a,
            mech_trace::gr!("1000ae80", "100b0440", DAT_100b0440) as i32,
        );
        w(
            b,
            0x78c,
            mech_trace::gr!("1000ae80", "100b0468", DAT_100b0468) as i32,
        );
        w(
            b,
            0x79e,
            mech_trace::gr!("1000ae80", "100b0490", DAT_100b0490) as i32,
        );
        w(
            b,
            0x7b0,
            mech_trace::gr!("1000ae80", "100b04b8", DAT_100b04b8) as i32,
        );
        w(
            b,
            0x7c2,
            mech_trace::gr!("1000ae80", "100b04e0", DAT_100b04e0) as i32,
        );
        w(
            b,
            0x7d4,
            mech_trace::gr!("1000ae80", "100b0508", DAT_100b0508) as i32,
        );
        w(
            b,
            0x7e6,
            mech_trace::gr!("1000ae80", "100b0530", DAT_100b0530) as i32,
        );
        w(b, 0x76c, -1);
        w(
            b,
            0x77e,
            mech_trace::gr!("1000ae80", "100b0444", DAT_100b0444) as i32,
        );
        w(
            b,
            0x790,
            mech_trace::gr!("1000ae80", "100b046c", DAT_100b046c) as i32,
        );
        w(
            b,
            0x7a2,
            mech_trace::gr!("1000ae80", "100b0494", DAT_100b0494) as i32,
        );
        w(b, 0x7b4, -1);
        w(b, 0x7c6, -1);
        w(b, 0x7d8, -1);
        w(b, 0x7ea, -1);
        w16(
            b,
            0x770,
            mech_trace::gr!("1000ae80", "100b043e", DAT_100b043e),
        );
        w16(
            b,
            0x782,
            mech_trace::gr!("1000ae80", "100b0466", DAT_100b0466),
        );
        w16(
            b,
            0x794,
            mech_trace::gr!("1000ae80", "100b048e", DAT_100b048e),
        );
        w16(
            b,
            0x7a6,
            mech_trace::gr!("1000ae80", "100b04b6", DAT_100b04b6),
        );
        w16(
            b,
            0x7b8,
            mech_trace::gr!("1000ae80", "100b04de", DAT_100b04de),
        );
        w16(
            b,
            0x7ca,
            mech_trace::gr!("1000ae80", "100b0506", DAT_100b0506),
        );
        w16(
            b,
            0x7dc,
            mech_trace::gr!("1000ae80", "100b052e", DAT_100b052e),
        );
        w16(
            b,
            0x7ee,
            mech_trace::gr!("1000ae80", "100b0556", DAT_100b0556),
        );
        w(b, 0x364, 0);
        w(b, 0x7f4, 0);
        w(b, 0x7f8, 0);
        w(b, 0x7fc, 0);
        w(b, 0x800, 0);
        w(b, 0x804, 0);
        w(b, 0x808, (r(b, 0x200) / 0x19).wrapping_add(0x32) / 100);
        if mech_trace::cr!(
            "1000ae80",
            "1000b940",
            FUN_1000b940(
                b as u32,
                0x1b59,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        ) != 0
        {
            w(b, 0x270, r(b, 0x808).wrapping_mul(100));
            w(b, 0x7f4, 1);
        }
        FUN_1000a790(b as i32);
        FUN_1000a800(b as i32);
        FUN_1000a8c0(b as i32);
        FUN_1000aa80(b as i32);
        FUN_1000a890(b as i32);
        w(b, 0x228, 0);
        let mut local_c: i32 = lc;
        let mut local_8: i32 = 0;
        if mech_trace::cr!(
            "1000ae80",
            "1000b940",
            FUN_1000b940(b as u32, 0x1f41, &mut local_c, &mut local_8)
        ) != 0
        {
            let idx = ((local_c.wrapping_mul(20)).wrapping_add(local_8) as isize) * 2;
            mech_trace::wr!(
                "1000ae80",
                t424.byte_offset(idx).cast::<u16>(),
                write_unaligned,
                0xffff
            );
            let rv = mech_trace::cr!(
                "1000ae80",
                "1000b940",
                FUN_1000b940(
                    b as u32,
                    0x1f41,
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                )
            );
            w(b, 0x228, 2);
            if rv == 0 {
                w(b, 0x228, 1);
            }
            mech_trace::wr!(
                "1000ae80",
                t424.byte_offset(idx).cast::<u16>(),
                write_unaligned,
                0x1f41
            );
        }
        if r(b, 0x228) == 0 {
            if r(b, 0x204) > r(b, 0x200)
                && mech_trace::gr!("1000ae80", "100b033c", DAT_100b033c) >= 0x14
                && (mech_trace::gr!("1000ae80", "100b033c", DAT_100b033c) & 1) == 0
            {
                w(b, 0x228, 2);
            } else {
                w(b, 0x228, 1);
            }
        }
        if r(b, 0x228) == 2 {
            w(b, 0x22c, (r(b, 0x22c) / 2).wrapping_sub(500));
            FUN_1000a890(b as i32);
        }
        w(b, 0x228, r(b, 0x228).wrapping_sub(1));
        w(b, 0x230, -1);
        FUN_1000a6f0(b as i32);
        w(b, 0x80c, -1);
        let v = mech_trace::cr!("1000ae80", "1000be10", FUN_1000be10());
        w(b, 0x810, v as i32);
        let v = mech_trace::cr!("1000ae80", "1000d650", FUN_1000d650(b as i32));
        w(b, 0x84f, v as i32);
        mech_trace::rt!("1000ae80", b.cast::<i8>())
    }
}
