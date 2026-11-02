// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa558: u32;
    static mut DAT_100aa55c: u32;
    static mut DAT_100aa560: u32;
    static mut DAT_100b38d8: i32;
    static mut DAT_100eef40: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10080048(param_1: *mut i32) -> u8;
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

unsafe fn ri(a: u32) -> i32 {
    // SAFETY: caller supplies a valid game address.
    unsafe { mech_trace::rd!("1007f35b", (a as *const i32), read_unaligned) }
}

unsafe fn set_dir(u3: u32, u4: u32, u5: u32, len: i32) {
    // SAFETY: globals and callees are the game's own.
    unsafe {
        if len == 0 {
            mech_trace::gw!("1007f35b", "100aa55c", DAT_100aa55c, 0);
            mech_trace::gw!("1007f35b", "100aa558", DAT_100aa558, 0);
            mech_trace::gw!("1007f35b", "100aa560", DAT_100aa560, 0x10000);
        } else {
            mech_trace::gw!(
                "1007f35b",
                "100aa558",
                DAT_100aa558,
                mech_trace::cr!("1007f35b", "10002570", FUN_10002570(u3, len))
            );
            mech_trace::gw!(
                "1007f35b",
                "100aa55c",
                DAT_100aa55c,
                mech_trace::cr!("1007f35b", "10002570", FUN_10002570(u4, len))
            );
            mech_trace::gw!(
                "1007f35b",
                "100aa560",
                DAT_100aa560,
                mech_trace::cr!("1007f35b", "10002570", FUN_10002570(u5, len))
            );
        }
    }
}

unsafe fn finish(
    param_1: *mut i32,
    pivar2: *mut i32,
    ivar1: u32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut i32,
    len: i32,
) {
    // SAFETY: all pointers are the game's own objects, accessed as the original does.
    unsafe {
        let mut i6 = mech_trace::cr!(
            "1007f35b",
            "10003d60",
            FUN_10003d60(
                mech_trace::gr!("1007f35b", "100aa558", DAT_100aa558) as i32,
                len
            )
        );
        mech_trace::wr!(
            "1007f35b",
            param_2,
            write_unaligned,
            ri(ivar1.wrapping_add(0x50)).wrapping_add(i6)
        );
        i6 = mech_trace::cr!(
            "1007f35b",
            "10003d60",
            FUN_10003d60(
                mech_trace::gr!("1007f35b", "100aa55c", DAT_100aa55c) as i32,
                len
            )
        );
        mech_trace::wr!(
            "1007f35b",
            param_3,
            write_unaligned,
            ri(ivar1.wrapping_add(0x54)).wrapping_add(i6)
        );
        i6 = mech_trace::cr!(
            "1007f35b",
            "10003d60",
            FUN_10003d60(
                mech_trace::gr!("1007f35b", "100aa560", DAT_100aa560) as i32,
                len
            )
        );
        mech_trace::wr!(
            "1007f35b",
            param_4,
            write_unaligned,
            ri(ivar1.wrapping_add(0x58)).wrapping_add(i6)
        );
        mech_trace::wr!(
            "1007f35b",
            param_1.add(0x10f),
            write_unaligned,
            mech_trace::rd!("1007f35b", pivar2.add(0x10f), read_unaligned)
        );
        let mut l38 = mech_trace::cr!(
            "1007f35b",
            "1000fc30",
            FUN_1000fc30(
                mech_trace::rd!("1007f35b", param_1.add(0x10d), read_unaligned).wrapping_sub(
                    mech_trace::rd!("1007f35b", pivar2.add(0x10d), read_unaligned)
                ),
                mech_trace::rd!("1007f35b", param_1.add(0x10e), read_unaligned).wrapping_sub(
                    mech_trace::rd!("1007f35b", pivar2.add(0x10e), read_unaligned)
                ),
                mech_trace::rd!("1007f35b", param_1.add(0x10f), read_unaligned),
            )
        );
        if l38 < 0x20001 {
            l38 = 0x10000;
        } else {
            l38 >>= 1;
        }
        i6 = mech_trace::cr!(
            "1007f35b",
            "10003d60",
            FUN_10003d60(
                mech_trace::gr!("1007f35b", "100aa558", DAT_100aa558) as i32,
                l38
            )
        );
        mech_trace::wr!("1007f35b", param_1.add(0x10a), write_unaligned, i6);
        i6 = mech_trace::cr!(
            "1007f35b",
            "10003d60",
            FUN_10003d60(
                mech_trace::gr!("1007f35b", "100aa55c", DAT_100aa55c) as i32,
                l38
            )
        );
        mech_trace::wr!("1007f35b", param_1.add(0x10b), write_unaligned, i6);
        i6 = mech_trace::cr!(
            "1007f35b",
            "10003d60",
            FUN_10003d60(
                mech_trace::gr!("1007f35b", "100aa560", DAT_100aa560) as i32,
                l38
            )
        );
        mech_trace::wr!("1007f35b", param_1.add(0x10c), write_unaligned, i6);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007f35b(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut i32,
    param_5: *mut i32,
) -> u32 {
    mech_trace::enter!("1007f35b", param_1, param_2, param_3, param_4, param_5);
    // SAFETY: all pointers and globals belong to the game; accessed as the original does.
    unsafe {
        let i6 = ri((mech_trace::rd!("1007f35b", param_1, read_unaligned) as u32).wrapping_add(4));
        let mut i7 = mech_trace::rd!("1007f35b", param_1.add(0x107), read_unaligned);
        let mut local_20: i32 = 0;
        loop {
            if mech_trace::gr!("1007f35b", "100b38d8", DAT_100b38d8) <= local_20 {
                return mech_trace::rt!("1007f35b", 0);
            }
            let ivar1 = mech_trace::rd!(
                "1007f35b",
                (&raw const DAT_100eef40)
                    .cast::<u32>()
                    .add(local_20 as usize),
                read
            );
            if ivar1 != 0 && ri(ivar1.wrapping_add(4)) != i6 {
                let piv2 = ri(ivar1.wrapping_add(0x20)) as u32 as *mut i32;
                if !piv2.is_null()
                    && (mech_trace::rd!("1007f35b", piv2.add(0x110).cast::<u16>(), read_unaligned)
                        & 0x100)
                        == 0
                    && ((ri(mech_trace::rd!("1007f35b", piv2, read_unaligned) as u32)
                        .wrapping_add(0) as u32
                        == 0
                        || true)
                        && (mech_trace::rd!(
                            "1007f35b",
                            (((mech_trace::rd!("1007f35b", piv2, read_unaligned) as u32)
                                .wrapping_add(0x14)) as *const u16),
                            read_unaligned
                        ) & 0x4000)
                            == 0)
                {
                    if ri(ivar1.wrapping_add(0x7c)) == i6 && local_20 < i6 {
                        if mech_trace::rd!("1007f35b", piv2.add(0xf5), read_unaligned) == 4 {
                            mech_trace::cr!("1007f35b", "10080048", FUN_10080048(piv2));
                            return mech_trace::rt!("1007f35b", 0);
                        }
                        mech_trace::wr!("1007f35b", param_5, write_unaligned, ivar1 as i32);
                        let u3 = mech_trace::rd!("1007f35b", param_2, read_unaligned)
                            .wrapping_sub(ri(ivar1.wrapping_add(0x50)))
                            as u32;
                        let u4 = mech_trace::rd!("1007f35b", param_3, read_unaligned)
                            .wrapping_sub(ri(ivar1.wrapping_add(0x54)))
                            as u32;
                        let u5 = mech_trace::rd!("1007f35b", param_4, read_unaligned)
                            .wrapping_sub(ri(ivar1.wrapping_add(0x58)))
                            as u32;
                        let len = mech_trace::cr!(
                            "1007f35b",
                            "1000fc30",
                            FUN_1000fc30(u3 as i32, u4 as i32, u5 as i32)
                        );
                        set_dir(u3, u4, u5, len);
                        i7 = mech_trace::rd!("1007f35b", piv2.add(0x107), read_unaligned)
                            .wrapping_add(i7);
                        finish(param_1, piv2, ivar1, param_2, param_3, param_4, i7);
                        return mech_trace::rt!("1007f35b", 1);
                    }
                    let u3 = mech_trace::rd!("1007f35b", param_2, read_unaligned)
                        .wrapping_sub(ri(ivar1.wrapping_add(0x50)))
                        as u32;
                    let u4 = mech_trace::rd!("1007f35b", param_3, read_unaligned)
                        .wrapping_sub(ri(ivar1.wrapping_add(0x54)))
                        as u32;
                    let u5 = mech_trace::rd!("1007f35b", param_4, read_unaligned)
                        .wrapping_sub(ri(ivar1.wrapping_add(0x58)))
                        as u32;
                    let i8 = mech_trace::rd!("1007f35b", piv2.add(0x107), read_unaligned)
                        .wrapping_add(i7);
                    let i9 = mech_trace::cr!(
                        "1007f35b",
                        "1000fc30",
                        FUN_1000fc30(u3 as i32, u4 as i32, u5 as i32)
                    );
                    if i9 < i8 {
                        if mech_trace::rd!("1007f35b", piv2.add(0xf5), read_unaligned) == 4 {
                            mech_trace::cr!("1007f35b", "10080048", FUN_10080048(piv2));
                            return mech_trace::rt!("1007f35b", 0);
                        }
                        mech_trace::wr!("1007f35b", param_5, write_unaligned, ivar1 as i32);
                        set_dir(u3, u4, u5, i9);
                        finish(param_1, piv2, ivar1, param_2, param_3, param_4, i8);
                        return mech_trace::rt!("1007f35b", 1);
                    }
                }
            }
            local_20 = local_20.wrapping_add(1);
        }
    }
}
