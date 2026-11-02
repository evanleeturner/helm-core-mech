// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut LAB_1000bde0: [u8; 0];
    static mut DAT_10083508: u32;
    static mut DAT_10094514: [i8; 0];
    static mut DAT_1009451c: [i8; 0];
    static mut DAT_10094524: [i8; 0];
    static mut DAT_10098224: *mut u8;
    static mut DAT_100a678c: *mut i32;
    static mut DAT_100a6798: *mut i32;
    static mut DAT_100a67a4: *mut i32;
    static mut DAT_100a67a8: *mut i32;
    static mut DAT_100a67ac: *mut i32;
    static mut DAT_100a67b0: u32;
    static mut DAT_100a67b4: *mut i32;
    static mut DAT_100a67b8: *mut i32;
    static mut s_NOVICE_1009450c: [i8; 0];
    static mut s_SHELLMER_CFG_100944fc: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006e790(param_1: *mut u8) -> u8;
    fn FUN_1006f780(param_1: *const u8) -> u8;
    fn FUN_10017c90(
        param_1: *mut u32,
        param_2: i32,
        param_3: u32,
        param_4: u8,
        param_5: u32,
    ) -> u32;
    fn FUN_10017d70(param_1: *mut i32) -> u32;
    fn FUN_10038710(param_1: *mut i8) -> u32;
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_10017df0(param_1: *mut i32, param_2: u32) -> u32;
    fn FUN_100180f0(param_1: *mut i32, param_2: i8) -> u8;
    fn FUN_10017ec0(param_1: i32, param_2: i32, param_3: i8) -> u8;
    fn FUN_1000be60(param_1: i32, param_2: i32) -> u8;
    fn FUN_1000be30(param_1: *mut u32) -> u32;
    fn FUN_1000ae30(param_1: *mut u8) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(param_1: *mut u8, param_2: u32, param_3: u32) -> *mut u8;
    fn FUN_10037c90() -> u8;
    fn FUN_10037d30() -> u8;
    fn FUN_10037ce0() -> u8;
}

#[allow(non_snake_case)]
unsafe fn drain_queue() {
    // SAFETY: the globals and callees are the game's own; the queue object
    // pointed to by DAT_100a67b0 is read where the original reads it.
    unsafe {
        let mut i_var5 = mech_trace::rd!(
            "10039270",
            ((mech_trace::gr!("10039270", "100a67b0", DAT_100a67b0).wrapping_add(4)) as *const i32),
            read_unaligned
        );
        while 0 < i_var5 {
            let pu_var3 = mech_trace::cr!(
                "10039270",
                "10018060",
                FUN_10018060(
                    mech_trace::gr!("10039270", "100a67b0", DAT_100a67b0) as i32,
                    0
                )
            ) as *mut u32;
            mech_trace::cr!(
                "10039270",
                "10017ec0",
                FUN_10017ec0(
                    mech_trace::gr!("10039270", "100a67b0", DAT_100a67b0) as i32,
                    pu_var3 as i32,
                    0
                )
            );
            mech_trace::cr!(
                "10039270",
                "1000be60",
                FUN_1000be60(
                    pu_var3 as i32,
                    mech_trace::gr!("10039270", "100a6798", DAT_100a6798) as i32
                )
            );
            mech_trace::cr!(
                "10039270",
                "10017df0",
                FUN_10017df0(
                    mech_trace::gr!("10039270", "100a6798", DAT_100a6798),
                    pu_var3 as u32
                )
            );
            let u_var4 = mech_trace::cr!("10039270", "1000be30", FUN_1000be30(pu_var3));
            mech_trace::cr!(
                "10039270",
                "10017df0",
                FUN_10017df0(
                    mech_trace::gr!("10039270", "100a678c", DAT_100a678c),
                    u_var4
                )
            );
            i_var5 = mech_trace::rd!(
                "10039270",
                ((mech_trace::gr!("10039270", "100a67b0", DAT_100a67b0).wrapping_add(4))
                    as *const i32),
                read_unaligned
            );
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10039270(param_1: u32) {
    mech_trace::enter!("10039270", param_1);
    // SAFETY: the globals and callees are the game's own; the allocated
    // block is 0x2dfb bytes and all offsets written stay inside it.
    unsafe {
        if mech_trace::gr!("10039270", "10083508", DAT_10083508) != 0 {
            mech_trace::cr!(
                "10039270",
                "1006e790",
                FUN_1006e790(mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u8)
            );
        }
        mech_trace::gw!(
            "10039270",
            "10083508",
            DAT_10083508,
            mech_trace::cr!(
                "10039270",
                "HeapAlloc",
                HeapAlloc(
                    mech_trace::gr!("10039270", "10098224", DAT_10098224),
                    1,
                    0x2dfb
                )
            ) as u32
        );
        let mut p = mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u32;
        let mut i_var5: i32 = 0xb7e;
        while i_var5 != 0 {
            mech_trace::wr!("10039270", p, write_unaligned, 0);
            p = p.add(1);
            i_var5 -= 1;
        }
        mech_trace::wr!("10039270", (p as *mut u16), write_unaligned, 0);
        mech_trace::wr!("10039270", (p as *mut u8).add(2), write, 0);

        let b = mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u8;
        mech_trace::wr!("10039270", b, write, 0);
        mech_trace::wr!("10039270", b.add(0x8e7), write, 1);
        mech_trace::wr!(
            "10039270",
            b.add(0x23bb).cast::<u32>(),
            write_unaligned,
            param_1
        );
        mech_trace::wr!("10039270", b.add(0x15e).cast::<u32>(), write_unaligned, 0);
        mech_trace::wr!("10039270", b.add(1), write, 0);
        mech_trace::wr!("10039270", b.add(0x23b9), write, 0);
        mech_trace::wr!("10039270", b.add(0x239b), write, 0);
        mech_trace::wr!("10039270", b.add(0x239a), write, 0);
        mech_trace::wr!("10039270", b.add(0x8ee), write, 0);
        mech_trace::wr!("10039270", b.add(0x2df7).cast::<u32>(), write_unaligned, 3);
        i_var5 = 0;
        loop {
            let i_var6 = i_var5.wrapping_add(4);
            mech_trace::wr!(
                "10039270",
                (mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u8)
                    .offset(i_var5 as isize)
                    .add(0x2342)
                    .cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            i_var5 = i_var6;
            if i_var6 >= 0x50 {
                break;
            }
        }
        mech_trace::cr!("10039270", "10037c90", FUN_10037c90());
        mech_trace::cr!("10039270", "10037d30", FUN_10037d30());
        mech_trace::cr!("10039270", "10037ce0", FUN_10037ce0());
        mech_trace::cr!(
            "10039270",
            "1006f780",
            FUN_1006f780((&raw mut s_SHELLMER_CFG_100944fc) as *const u8)
        );
        mech_trace::cr!(
            "10039270",
            "10017c90",
            FUN_10017c90(
                (&raw mut DAT_100a678c) as *mut u32,
                10,
                FUN_1000ae30 as usize as u32,
                4,
                0,
            )
        );
        mech_trace::cr!(
            "10039270",
            "10017c90",
            FUN_10017c90(
                (&raw mut DAT_100a6798) as *mut u32,
                10,
                FUN_1000ae30 as usize as u32,
                4,
                0,
            )
        );
        mech_trace::cr!(
            "10039270",
            "10017d70",
            FUN_10017d70(mech_trace::gr!("10039270", "100a67a4", DAT_100a67a4))
        );
        mech_trace::cr!(
            "10039270",
            "10017c90",
            FUN_10017c90((&raw mut DAT_100a67a4) as *mut u32, 10, 0, 4, 0)
        );
        mech_trace::cr!(
            "10039270",
            "10017d70",
            FUN_10017d70(mech_trace::gr!("10039270", "100a67a8", DAT_100a67a8))
        );
        mech_trace::cr!(
            "10039270",
            "10017c90",
            FUN_10017c90((&raw mut DAT_100a67a8) as *mut u32, 10, 0, 4, 0)
        );
        mech_trace::cr!(
            "10039270",
            "10017d70",
            FUN_10017d70(mech_trace::gr!("10039270", "100a67b4", DAT_100a67b4))
        );
        mech_trace::cr!(
            "10039270",
            "10017c90",
            FUN_10017c90(
                (&raw mut DAT_100a67b4) as *mut u32,
                10,
                0,
                1,
                (&raw mut LAB_1000bde0) as usize as u32,
            )
        );
        mech_trace::cr!(
            "10039270",
            "10017d70",
            FUN_10017d70(mech_trace::gr!("10039270", "100a67b8", DAT_100a67b8))
        );
        mech_trace::cr!(
            "10039270",
            "10017c90",
            FUN_10017c90(
                (&raw mut DAT_100a67b8) as *mut u32,
                10,
                0,
                1,
                (&raw mut LAB_1000bde0) as usize as u32,
            )
        );
        mech_trace::cr!(
            "10039270",
            "10017d70",
            FUN_10017d70(mech_trace::gr!("10039270", "100a67ac", DAT_100a67ac))
        );
        mech_trace::cr!(
            "10039270",
            "10017c90",
            FUN_10017c90(
                (&raw mut DAT_100a67ac) as *mut u32,
                10,
                0,
                1,
                (&raw mut LAB_1000bde0) as usize as u32,
            )
        );
        mech_trace::cr!(
            "10039270",
            "10017c90",
            FUN_10017c90(
                &raw mut DAT_100a67b0,
                10,
                FUN_1000ae30 as usize as u32,
                4,
                0,
            )
        );

        let mode = mech_trace::rd!(
            "10039270",
            (mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u8)
                .add(0x23bb)
                .cast::<i32>(),
            read_unaligned
        );
        if mode == 0 {
            mech_trace::cr!(
                "10039270",
                "10038710",
                FUN_10038710((&raw mut s_NOVICE_1009450c) as *mut i8)
            );
            mech_trace::wr!(
                "10039270",
                (mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u8)
                    .add(0x12a)
                    .cast::<u32>(),
                write_unaligned,
                0
            );
            return;
        }
        if mode == 1 {
            mech_trace::cr!(
                "10039270",
                "10038710",
                FUN_10038710((&raw mut DAT_10094514) as *mut i8)
            );
            let v = mech_trace::rd!(
                "10039270",
                (mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u8)
                    .add(0x1e6)
                    .cast::<u32>(),
                read_unaligned
            );
            mech_trace::wr!(
                "10039270",
                (mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u8)
                    .add(0x12a)
                    .cast::<u32>(),
                write_unaligned,
                v
            );
            mech_trace::wr!(
                "10039270",
                (mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u8)
                    .add(0x23a3)
                    .cast::<u32>(),
                write_unaligned,
                10000
            );
            mech_trace::wr!(
                "10039270",
                (mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u8)
                    .add(0x23a7)
                    .cast::<u32>(),
                write_unaligned,
                2000
            );
            i_var5 = 0;
            mech_trace::wr!(
                "10039270",
                (mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u8)
                    .add(0x23ab)
                    .cast::<u32>(),
                write_unaligned,
                1000
            );
            mech_trace::wr!(
                "10039270",
                (mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u8)
                    .add(0x23af)
                    .cast::<u32>(),
                write_unaligned,
                500
            );
            mech_trace::wr!(
                "10039270",
                (mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u8)
                    .add(0x23b3)
                    .cast::<u32>(),
                write_unaligned,
                0x2d0
            );
            if 0 < mech_trace::rd!(
                "10039270",
                mech_trace::gr!("10039270", "100a67ac", DAT_100a67ac).add(1),
                read_unaligned
            ) {
                loop {
                    let i_var6 = mech_trace::cr!(
                        "10039270",
                        "10018060",
                        FUN_10018060(
                            mech_trace::gr!("10039270", "100a67ac", DAT_100a67ac) as i32,
                            i_var5
                        )
                    );
                    let mut pi_var2 = mech_trace::gr!("10039270", "100a67b4", DAT_100a67b4);
                    if mech_trace::rd!("10039270", (i_var6.wrapping_add(8) as *const i8), read) == 1
                    {
                        pi_var2 = mech_trace::gr!("10039270", "100a67b8", DAT_100a67b8);
                    }
                    i_var5 = i_var5.wrapping_add(1);
                    mech_trace::cr!("10039270", "10017df0", FUN_10017df0(pi_var2, i_var6));
                    if i_var5
                        >= mech_trace::rd!(
                            "10039270",
                            mech_trace::gr!("10039270", "100a67ac", DAT_100a67ac).add(1),
                            read_unaligned
                        )
                    {
                        break;
                    }
                }
            }
            mech_trace::cr!(
                "10039270",
                "100180f0",
                FUN_100180f0(mech_trace::gr!("10039270", "100a67ac", DAT_100a67ac), 0)
            );
            drain_queue();
            mech_trace::wr!(
                "10039270",
                (mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u8).add(0x239d),
                write,
                1
            );
            mech_trace::wr!(
                "10039270",
                (mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u8).add(0x23a2),
                write,
                0
            );
            return;
        }
        if mode != 2 {
            return;
        }
        mech_trace::cr!(
            "10039270",
            "10038710",
            FUN_10038710((&raw mut DAT_1009451c) as *mut i8)
        );
        let src = (&raw mut DAT_10094524) as *const u8;
        let mut len: usize = 0;
        while mech_trace::rd!("10039270", src.add(len), read) != 0 {
            len += 1;
        }
        let total = len + 1;
        let mut s = src;
        let mut d = (mech_trace::gr!("10039270", "10083508", DAT_10083508) as *mut u8).add(0x166);
        let mut n = total >> 2;
        while n != 0 {
            mech_trace::wr!(
                "10039270",
                d.cast::<u32>(),
                write_unaligned,
                mech_trace::rd!("10039270", s.cast::<u32>(), read_unaligned)
            );
            s = s.add(4);
            d = d.add(4);
            n -= 1;
        }
        let mut r = total & 3;
        while r != 0 {
            mech_trace::wr!("10039270", d, write, mech_trace::rd!("10039270", s, read));
            s = s.add(1);
            d = d.add(1);
            r -= 1;
        }
        drain_queue();
    }
}
