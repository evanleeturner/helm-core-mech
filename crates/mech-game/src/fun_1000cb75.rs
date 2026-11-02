// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7570: u32;
    static mut DAT_100a758c: i32;
    static mut DAT_100a7590: i32;
    static mut DAT_100a7594: i32;
    static mut DAT_100a7598: u8;
    static mut DAT_100a75c0: u8;
    static mut DAT_101f5518: u32;
    static mut DAT_101f5520: u8;
    static mut DAT_101f5524: u8;
    static mut DAT_101f5528: u8;
    static mut DAT_101f552c: u8;
    static mut DAT_101f5530: u8;
    static mut DAT_101f5534: u8;
    static mut DAT_101f5538: u8;
    static mut DAT_101f553c: u8;
    static mut DAT_101f5540: u8;
    static mut DAT_101f5544: u8;
    static mut DAT_101f5548: u8;
    static mut DAT_101f554c: u8;
    static mut DAT_101f5550: u8;
    static mut DAT_101f5554: u8;
    static mut DAT_101f5558: u8;
    static mut DAT_101f555c: u8;
    static mut DAT_101f5560: u8;
    static mut DAT_101f5564: u8;
    static mut DAT_101f5598: u8;
    static mut DAT_1021559c: u8;
    static mut DAT_102155a0: u8;
    static mut DAT_10215624: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008975d(param_1: *mut u32) -> u8;
    fn FUN_10089cc9(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u8;
    fn FUN_100895ca(param_1: *mut i32, param_2: *mut i32, param_3: *mut u32) -> u8;
}

unsafe fn g(base: *mut u8, off: i32) -> i32 {
    // SAFETY: caller supplies a game-owned address plus offset.
    unsafe {
        mech_trace::rd!(
            "1000cb75",
            base.wrapping_offset(off as isize).cast::<i32>(),
            read_unaligned
        )
    }
}

unsafe fn s(base: *mut u8, off: i32, v: i32) {
    // SAFETY: caller supplies a game-owned address plus offset.
    unsafe {
        mech_trace::wr!(
            "1000cb75",
            base.wrapping_offset(off as isize).cast::<i32>(),
            write_unaligned,
            v
        )
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000cb75(param_1: i32) {
    mech_trace::enter!("1000cb75", param_1);
    // SAFETY: all globals, arrays and callees are the game's own memory;
    // param_1 points at the caller's record, accessed as the original does.
    unsafe {
        let mut local_9c = [0i32; 12];
        let mut local_64 = [0i32; 12];
        let mut local_34 = [0i32; 12];

        let a20 = &raw mut DAT_101f5520 as *mut u8;
        let a24 = &raw mut DAT_101f5524 as *mut u8;
        let a28 = &raw mut DAT_101f5528 as *mut u8;
        let a2c = &raw mut DAT_101f552c as *mut u8;
        let a30 = &raw mut DAT_101f5530 as *mut u8;
        let a34 = &raw mut DAT_101f5534 as *mut u8;
        let a38 = &raw mut DAT_101f5538 as *mut u8;
        let a3c = &raw mut DAT_101f553c as *mut u8;
        let a40 = &raw mut DAT_101f5540 as *mut u8;
        let a44 = &raw mut DAT_101f5544 as *mut u8;
        let a48 = &raw mut DAT_101f5548 as *mut u8;
        let a4c = &raw mut DAT_101f554c as *mut u8;
        let a50 = &raw mut DAT_101f5550 as *mut u8;
        let a54 = &raw mut DAT_101f5554 as *mut u8;
        let a58 = &raw mut DAT_101f5558 as *mut u8;
        let a5c = &raw mut DAT_101f555c as *mut u8;
        let a60 = &raw mut DAT_101f5560 as *mut u8;
        let a64 = &raw mut DAT_101f5564 as *mut u8;
        let a98 = &raw mut DAT_101f5598 as *mut u8;
        let b9c = &raw mut DAT_1021559c as *mut u8;
        let ba0 = &raw mut DAT_102155a0 as *mut u8;

        if mech_trace::gr!("1000cb75", "100a7594", DAT_100a7594) == 0 {
            mech_trace::gw!("1000cb75", "101f5518", DAT_101f5518, 0);
            let mut local_6c: i32 = 0;
            while local_6c < mech_trace::gr!("1000cb75", "10215624", DAT_10215624) {
                let addr = mech_trace::gr!("1000cb75", "100a7570", DAT_100a7570)
                    .wrapping_add((local_6c as u32).wrapping_mul(4));
                mech_trace::wr!("1000cb75", (addr as *mut u32), write_unaligned, 0xffffffff);
                local_6c = local_6c.wrapping_add(1);
            }
            s(
                ba0,
                mech_trace::gr!("1000cb75", "100a7594", DAT_100a7594).wrapping_mul(4),
                -1,
            );
        }
        if mech_trace::gr!("1000cb75", "100a758c", DAT_100a758c) < 0x20
            && -1 < mech_trace::gr!("1000cb75", "100a758c", DAT_100a758c)
        {
            mech_trace::gw!(
                "1000cb75",
                "100a7594",
                DAT_100a7594,
                mech_trace::gr!("1000cb75", "100a7594", DAT_100a7594).wrapping_add(1)
            );
            s(
                ba0,
                mech_trace::gr!("1000cb75", "100a7594", DAT_100a7594).wrapping_mul(4),
                mech_trace::gr!("1000cb75", "100a758c", DAT_100a758c),
            );
            let i4 = mech_trace::gr!("1000cb75", "100a758c", DAT_100a758c).wrapping_mul(0x7c);
            let pi1 = a20.wrapping_offset(i4 as isize);
            let p1 = |o: i32| g(param_1 as u32 as usize as *mut u8, o);
            s(pi1, 0, p1(8));
            s(a24, i4, p1(0xc));
            s(a28, i4, p1(0x10));
            s(a2c, i4, p1(0x14));
            s(a30, i4, p1(0x18));
            s(a34, i4, p1(0x1c));
            s(
                a98,
                i4,
                g(
                    b9c,
                    mech_trace::gr!("1000cb75", "100a7594", DAT_100a7594).wrapping_mul(4),
                ),
            );

            let mut src = &raw mut DAT_100a7598 as *mut u32;
            let mut dst = a44.wrapping_offset(i4 as isize) as *mut u32;
            let mut n = 9;
            while n != 0 {
                mech_trace::wr!(
                    "1000cb75",
                    dst,
                    write_unaligned,
                    mech_trace::rd!("1000cb75", src, read_unaligned)
                );
                src = src.wrapping_add(1);
                dst = dst.wrapping_add(1);
                n -= 1;
            }

            if g(a2c, i4) < 0 {
                s(pi1, 0, g(pi1, 0).wrapping_add(g(a2c, i4)));
                s(a2c, i4, g(a2c, i4).wrapping_neg());
            }
            s(a2c, i4, g(a2c, i4).wrapping_add(g(pi1, 0)));
            if g(a30, i4) < 0 {
                s(a24, i4, g(a24, i4).wrapping_add(g(a30, i4)));
                s(a30, i4, g(a30, i4).wrapping_neg());
            }
            s(a30, i4, g(a30, i4).wrapping_add(g(a24, i4)));
            if g(a34, i4) < 0 {
                s(a28, i4, g(a28, i4).wrapping_add(g(a34, i4)));
                s(a34, i4, g(a34, i4).wrapping_neg());
            }
            s(a34, i4, g(a34, i4).wrapping_add(g(a28, i4)));
            s(a38, i4, g(a2c, i4).wrapping_add(g(pi1, 0)) / 2);
            s(a3c, i4, g(a24, i4).wrapping_add(g(a30, i4)) / 2);
            s(a40, i4, g(a34, i4).wrapping_add(g(a28, i4)) / 2);

            let pi2 = i4.wrapping_add(0x101f5568) as u32 as usize as *mut i32;
            mech_trace::cr!(
                "1000cb75",
                "1008975d",
                FUN_1008975d(local_34.as_mut_ptr().cast::<u32>())
            );
            local_34[0] = ((g(a44, i4) as u32).wrapping_shl(0x1d)) as i32;
            local_34[4] = ((g(a48, i4) as u32).wrapping_shl(0x1d)) as i32;
            local_34[8] = ((g(a4c, i4) as u32).wrapping_shl(0x1d)) as i32;

            mech_trace::cr!(
                "1000cb75",
                "10089cc9",
                FUN_10089cc9(
                    local_64.as_mut_ptr(),
                    0,
                    0,
                    0,
                    g(a38, i4).wrapping_neg(),
                    g(a3c, i4).wrapping_neg(),
                    g(a40, i4).wrapping_neg(),
                )
            );
            mech_trace::cr!(
                "1000cb75",
                "10089cc9",
                FUN_10089cc9(
                    local_9c.as_mut_ptr(),
                    g(a50, i4),
                    g(a54, i4),
                    g(a58, i4),
                    0,
                    0,
                    0,
                )
            );
            mech_trace::cr!(
                "1000cb75",
                "100895ca",
                FUN_100895ca(
                    local_9c.as_mut_ptr(),
                    local_64.as_mut_ptr(),
                    pi2.cast::<u32>(),
                )
            );
            mech_trace::cr!(
                "1000cb75",
                "10089cc9",
                FUN_10089cc9(
                    local_64.as_mut_ptr(),
                    0,
                    0,
                    0,
                    g(a5c, i4),
                    g(a60, i4),
                    g(a64, i4),
                )
            );
            mech_trace::cr!(
                "1000cb75",
                "100895ca",
                FUN_100895ca(local_64.as_mut_ptr(), pi2, pi2.cast::<u32>())
            );
            mech_trace::cr!(
                "1000cb75",
                "10089cc9",
                FUN_10089cc9(
                    local_64.as_mut_ptr(),
                    0,
                    0,
                    0,
                    g(a38, i4),
                    g(a3c, i4),
                    g(a40, i4),
                )
            );
            mech_trace::cr!(
                "1000cb75",
                "100895ca",
                FUN_100895ca(local_64.as_mut_ptr(), pi2, pi2.cast::<u32>())
            );

            let mut local_68 = pi1.cast::<i32>();
            while mech_trace::rd!("1000cb75", local_68.wrapping_add(0x1e), read_unaligned) != -1 {
                let pj = local_68.wrapping_add(0x1e);
                let idx = mech_trace::rd!("1000cb75", pj, read_unaligned);
                local_68 = a20
                    .wrapping_offset(idx.wrapping_mul(0x7c) as isize)
                    .cast::<i32>();
                let m = idx.wrapping_mul(0x7c).wrapping_add(0x101f5568) as u32 as usize as *mut i32;
                mech_trace::cr!(
                    "1000cb75",
                    "100895ca",
                    FUN_100895ca(m, pi2, pi2.cast::<u32>())
                );
            }

            let mut src = &raw mut DAT_100a75c0 as *mut u32;
            let mut dst = &raw mut DAT_100a7598 as *mut u32;
            let mut n = 9;
            while n != 0 {
                mech_trace::wr!(
                    "1000cb75",
                    dst,
                    write_unaligned,
                    mech_trace::rd!("1000cb75", src, read_unaligned)
                );
                src = src.wrapping_add(1);
                dst = dst.wrapping_add(1);
                n -= 1;
            }
            mech_trace::gw!(
                "1000cb75",
                "100a7590",
                DAT_100a7590,
                mech_trace::gr!("1000cb75", "100a758c", DAT_100a758c)
            );
            mech_trace::gw!(
                "1000cb75",
                "100a758c",
                DAT_100a758c,
                mech_trace::gr!("1000cb75", "100a758c", DAT_100a758c).wrapping_add(1)
            );
        }
    }
}
