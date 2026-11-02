// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10026f0b(param_1: *mut i32) -> u8;
    fn FUN_10061d8b(param_1: *mut i32) -> u8;
    fn FUN_100832d4(param_1: i32) -> u32;
    fn FUN_100639ad(param_1: *mut i32) -> u8;
    fn FUN_1000465a(param_1: *mut i32) -> u8;
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_1006b2cc(param_1: *mut i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1004fed5() -> u8;
}

unsafe fn rd(a: i32) -> i32 {
    // SAFETY: the address is a game object field supplied by the caller's data.
    unsafe { mech_trace::rd!("1004c9d7", (a as usize as *const i32), read_unaligned) }
}

unsafe fn wr(a: i32, v: i32) {
    // SAFETY: the address is a game object field supplied by the caller's data.
    unsafe { mech_trace::wr!("1004c9d7", (a as usize as *mut i32), write_unaligned, v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004c9d7(param_1: *mut i32) {
    mech_trace::enter!("1004c9d7", param_1);
    // SAFETY: param_1 points at the game's object; all inner pointers are the
    // game's own structures, accessed at the same offsets and order as the original.
    unsafe {
        mech_trace::cr!(
            "1004c9d7",
            "10026f0b",
            FUN_10026f0b(mech_trace::rd!("1004c9d7", param_1, read_unaligned) as usize as *mut i32)
        );
        if mech_trace::rd!("1004c9d7", param_1.add(0xf5), read_unaligned) == 2 {
            mech_trace::cr!("1004c9d7", "10061d8b", FUN_10061d8b(param_1));
            let o = mech_trace::rd!("1004c9d7", param_1, read_unaligned);
            if rd(o.wrapping_add(0x10)) == 2 {
                wr(
                    mech_trace::rd!("1004c9d7", param_1, read_unaligned).wrapping_add(0xdc),
                    0,
                );
            } else {
                let o = mech_trace::rd!("1004c9d7", param_1, read_unaligned);
                if rd(o.wrapping_add(0xdc)) != 0
                    && (mech_trace::rd!(
                        "1004c9d7",
                        (o.wrapping_add(0xdd) as usize as *const u8),
                        read
                    ) & 0x10)
                        == 0
                    && mech_trace::cr!(
                        "1004c9d7",
                        "100832d4",
                        FUN_100832d4(mech_trace::rd!("1004c9d7", param_1, read_unaligned))
                    ) == 0
                {
                    wr(
                        mech_trace::rd!("1004c9d7", param_1, read_unaligned).wrapping_add(0xdc),
                        0,
                    );
                }
            }
            mech_trace::cr!("1004c9d7", "100639ad", FUN_100639ad(param_1));
            mech_trace::wr!("1004c9d7", param_1.add(0xfc), write_unaligned, 0);
            let o = mech_trace::rd!("1004c9d7", param_1, read_unaligned);
            mech_trace::wr!(
                "1004c9d7",
                (rd(o.wrapping_add(0x4c)).wrapping_add(0x43) as usize as *mut u8),
                write,
                0
            );
            let flags =
                mech_trace::rd!("1004c9d7", param_1.add(0x110).cast::<u16>(), read_unaligned);
            if (flags & 4) != 0 && (flags & 8) == 0 {
                let o = mech_trace::rd!("1004c9d7", param_1, read_unaligned);
                wr(rd(o.wrapping_add(0x4c)).wrapping_add(8), 0);
            }
            let o = mech_trace::rd!("1004c9d7", param_1, read_unaligned);
            let i_var1 = rd(rd(o.wrapping_add(0x4c)).wrapping_add(4));
            let p105 = mech_trace::rd!("1004c9d7", param_1.add(0x105), read_unaligned);
            if p105 < 0x1680000 {
                let p2 = mech_trace::rd!("1004c9d7", param_1.add(2), read_unaligned);
                if p105 < p2 {
                    mech_trace::wr!(
                        "1004c9d7",
                        param_1.add(2),
                        write_unaligned,
                        mech_trace::rd!("1004c9d7", param_1.add(0x105), read_unaligned)
                    );
                } else if p2.wrapping_neg() != p105 && p2 <= p105.wrapping_neg() {
                    mech_trace::wr!(
                        "1004c9d7",
                        param_1.add(2),
                        write_unaligned,
                        mech_trace::rd!("1004c9d7", param_1.add(0x105), read_unaligned)
                            .wrapping_neg()
                    );
                }
            } else {
                let mut local_10 = i_var1.wrapping_sub(mech_trace::rd!(
                    "1004c9d7",
                    param_1.add(3),
                    read_unaligned
                ));
                while 0xb40000 < local_10 {
                    local_10 = local_10.wrapping_sub(0x1680000);
                }
                while local_10 < -0xb40000 {
                    local_10 = local_10.wrapping_add(0x1680000);
                }
                mech_trace::wr!("1004c9d7", param_1.add(2), write_unaligned, i_var1);
                mech_trace::wr!(
                    "1004c9d7",
                    param_1.add(3),
                    write_unaligned,
                    i_var1.wrapping_sub(local_10)
                );
            }
            let o = mech_trace::rd!("1004c9d7", param_1, read_unaligned);
            mech_trace::wr!(
                "1004c9d7",
                param_1.add(6),
                write_unaligned,
                rd(rd(o.wrapping_add(0x4c)))
            );
        } else {
            mech_trace::wr!("1004c9d7", param_1.add(2), write_unaligned, 0);
            mech_trace::wr!("1004c9d7", param_1.add(6), write_unaligned, 0);
            let o = mech_trace::rd!("1004c9d7", param_1, read_unaligned);
            wr(rd(o.wrapping_add(0x4c)).wrapping_add(8), 0);
            let o = mech_trace::rd!("1004c9d7", param_1, read_unaligned);
            wr(rd(o.wrapping_add(0x4c)).wrapping_add(4), 0);
            let o = mech_trace::rd!("1004c9d7", param_1, read_unaligned);
            wr(rd(o.wrapping_add(0x4c)), 0);
        }
        mech_trace::cr!("1004c9d7", "1000465a", FUN_1000465a(param_1));
        match mech_trace::rd!("1004c9d7", param_1.add(0xf5), read_unaligned) {
            0 => {
                mech_trace::wr!("1004c9d7", param_1.add(0xf5), write_unaligned, 1);
                let i_var1 = mech_trace::cr!("1004c9d7", "10051f43", FUN_10051f43(0x16a));
                mech_trace::wr!(
                    "1004c9d7",
                    param_1.add(0xef),
                    write_unaligned,
                    (mech_trace::gr!("1004c9d7", "100b38fc", DAT_100b38fc) as i32)
                        .wrapping_add(i_var1)
                        .wrapping_add(0x43e)
                );
            }
            1 => {
                if mech_trace::rd!("1004c9d7", param_1.add(0xef), read_unaligned)
                    < mech_trace::gr!("1004c9d7", "100b38fc", DAT_100b38fc) as i32
                {
                    mech_trace::wr!("1004c9d7", param_1.add(0xf5), write_unaligned, 2);
                }
            }
            2 => {}
            3 => {
                mech_trace::wr!("1004c9d7", param_1.add(0xf5), write_unaligned, 0);
            }
            4 => {
                if mech_trace::rd!("1004c9d7", param_1.add(0xef), read_unaligned) == 0 {
                    mech_trace::wr!(
                        "1004c9d7",
                        param_1.add(0xef),
                        write_unaligned,
                        (mech_trace::gr!("1004c9d7", "100b38fc", DAT_100b38fc) as i32)
                            .wrapping_add(0x712)
                    );
                    mech_trace::cr!("1004c9d7", "1004fed5", FUN_1004fed5());
                }
                if (mech_trace::gr!("1004c9d7", "100b38fc", DAT_100b38fc) as i32)
                    < mech_trace::rd!("1004c9d7", param_1.add(0xef), read_unaligned)
                {
                    mech_trace::cr!("1004c9d7", "1006b2cc", FUN_1006b2cc(param_1));
                }
            }
            _ => {}
        }
    }
}
