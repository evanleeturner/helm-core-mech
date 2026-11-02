// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a67b4: i32;
    static mut DAT_100a67b8: i32;
    static mut DAT_100aa6e0: i32;
    static mut DAT_100aa6e4: i32;
    static mut DAT_10082bf0: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000d230(param_1: i32) -> i32;
    fn FUN_1000d260(param_1: i32, param_2: i32) -> u32;
    fn FUN_10017df0(param_1: i32, param_2: *mut c_void) -> u32;
    fn FUN_10017ec0(param_1: i32, param_2: i32, param_3: i8);
    fn FUN_10018060(param_1: i32, param_2: i32) -> i32;
}

unsafe fn scan_kind2(val: i32, param_2: i8) -> u32 {
    // SAFETY: the globals and callees are the game's own; collection items
    // are valid records of at least 12 bytes.
    unsafe {
        let mut i: i32 = 0;
        if 0 < mech_trace::rd!(
            "1006b9c0",
            ((mech_trace::gr!("1006b9c0", "100a67b4", DAT_100a67b4).wrapping_add(4)) as *const i32),
            read_unaligned
        ) {
            loop {
                let item = mech_trace::cr!(
                    "1006b9c0",
                    "10018060",
                    FUN_10018060(mech_trace::gr!("1006b9c0", "100a67b4", DAT_100a67b4), i)
                );
                let p = item as *mut i32;
                if (mech_trace::rd!("1006b9c0", p.add(2), read_unaligned) as i8) == 2
                    && mech_trace::rd!("1006b9c0", p, read_unaligned) == val
                {
                    let mut u_var5: u32 = 0;
                    if param_2 == 1 {
                        FUN_10017ec0(
                            mech_trace::gr!("1006b9c0", "100a67b4", DAT_100a67b4),
                            item,
                            0,
                        );
                        u_var5 = mech_trace::cr!(
                            "1006b9c0",
                            "10017df0",
                            FUN_10017df0(
                                mech_trace::gr!("1006b9c0", "100aa6e0", DAT_100aa6e0),
                                p as *mut c_void
                            )
                        );
                    }
                    return (u_var5 & 0xffffff00) | 1;
                }
                i = i.wrapping_add(1);
                if !(i < mech_trace::rd!(
                    "1006b9c0",
                    ((mech_trace::gr!("1006b9c0", "100a67b4", DAT_100a67b4).wrapping_add(4))
                        as *const i32),
                    read_unaligned
                )) {
                    break;
                }
            }
        }
        (mech_trace::gr!("1006b9c0", "100a67b4", DAT_100a67b4) as u32) & 0xffffff00
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006b9c0(param_1: i32, param_2: i8, param_3: *mut u32) -> u32 {
    mech_trace::enter!("1006b9c0", param_1, param_2, param_3);
    // SAFETY: the globals and callees are the game's own; param_3 and the
    // collection items are valid memory supplied by the caller and the game.
    unsafe {
        mech_trace::wr!("1006b9c0", param_3, write_unaligned, 0);
        let mut i_var6 = mech_trace::cr!("1006b9c0", "1000d230", FUN_1000d230(param_1));
        let u_var2 = mech_trace::cr!("1006b9c0", "1000d260", FUN_1000d260(i_var6, param_1));
        if i_var6 == 0 {
            i_var6 = 0;
            if 0 < mech_trace::rd!(
                "1006b9c0",
                ((mech_trace::gr!("1006b9c0", "100a67b4", DAT_100a67b4).wrapping_add(4))
                    as *const i32),
                read_unaligned
            ) {
                loop {
                    let item = mech_trace::cr!(
                        "1006b9c0",
                        "10018060",
                        FUN_10018060(
                            mech_trace::gr!("1006b9c0", "100a67b4", DAT_100a67b4),
                            i_var6
                        )
                    );
                    let p = item as *mut u32;
                    if mech_trace::rd!("1006b9c0", p, read_unaligned) == u_var2
                        && (mech_trace::rd!("1006b9c0", p.add(2), read_unaligned) as i8) == 0
                    {
                        let mut u_var5 = ((item as u32) & 0xffffff00) | (param_2 as u8 as u32);
                        if param_2 == 1 {
                            FUN_10017ec0(
                                mech_trace::gr!("1006b9c0", "100a67b4", DAT_100a67b4),
                                item,
                                0,
                            );
                            u_var5 = mech_trace::cr!(
                                "1006b9c0",
                                "10017df0",
                                FUN_10017df0(
                                    mech_trace::gr!("1006b9c0", "100aa6e0", DAT_100aa6e0),
                                    p as *mut c_void
                                )
                            );
                        }
                        return mech_trace::rt!("1006b9c0", (u_var5 & 0xffffff00) | 1);
                    }
                    i_var6 = i_var6.wrapping_add(1);
                    if !(i_var6
                        < mech_trace::rd!(
                            "1006b9c0",
                            ((mech_trace::gr!("1006b9c0", "100a67b4", DAT_100a67b4).wrapping_add(4))
                                as *const i32),
                            read_unaligned
                        ))
                    {
                        break;
                    }
                }
            }
            return mech_trace::rt!(
                "1006b9c0",
                (mech_trace::gr!("1006b9c0", "100a67b4", DAT_100a67b4) as u32) & 0xffffff00
            );
        }
        if i_var6 == 1 {
            i_var6 = 0;
            if 0 < mech_trace::rd!(
                "1006b9c0",
                ((mech_trace::gr!("1006b9c0", "100a67b8", DAT_100a67b8).wrapping_add(4))
                    as *const i32),
                read_unaligned
            ) {
                loop {
                    let item = mech_trace::cr!(
                        "1006b9c0",
                        "10018060",
                        FUN_10018060(
                            mech_trace::gr!("1006b9c0", "100a67b8", DAT_100a67b8),
                            i_var6
                        )
                    );
                    let p = item as *mut u32;
                    if mech_trace::rd!("1006b9c0", p, read_unaligned) == u_var2
                        && (mech_trace::rd!("1006b9c0", p.add(2), read_unaligned) as i8) == 1
                    {
                        let mut u_var5 = ((item as u32) & 0xffffff00) | (param_2 as u8 as u32);
                        if param_2 == 1 {
                            FUN_10017ec0(
                                mech_trace::gr!("1006b9c0", "100a67b8", DAT_100a67b8),
                                item,
                                0,
                            );
                            u_var5 = mech_trace::cr!(
                                "1006b9c0",
                                "10017df0",
                                FUN_10017df0(
                                    mech_trace::gr!("1006b9c0", "100aa6e4", DAT_100aa6e4),
                                    p as *mut c_void
                                )
                            );
                        }
                        return mech_trace::rt!("1006b9c0", (u_var5 & 0xffffff00) | 1);
                    }
                    i_var6 = i_var6.wrapping_add(1);
                    if !(i_var6
                        < mech_trace::rd!(
                            "1006b9c0",
                            ((mech_trace::gr!("1006b9c0", "100a67b8", DAT_100a67b8).wrapping_add(4))
                                as *const i32),
                            read_unaligned
                        ))
                    {
                        break;
                    }
                }
            }
            return mech_trace::rt!(
                "1006b9c0",
                (mech_trace::gr!("1006b9c0", "100a67b8", DAT_100a67b8) as u32) & 0xffffff00
            );
        }
        if i_var6 != 2 || (u_var2 as i32) < 0 {
            return mech_trace::rt!("1006b9c0", u_var2 & 0xffffff00);
        }
        i_var6 = (param_1 / 100).wrapping_mul(100);
        if i_var6 == 8000 {
            return mech_trace::rt!("1006b9c0", scan_kind2(1, param_2));
        }
        if i_var6 == 0x2134 {
            return mech_trace::rt!("1006b9c0", scan_kind2(6, param_2));
        }
        if i_var6 != 0x254e {
            let base = (&raw mut DAT_10082bf0).cast::<u8>();
            mech_trace::wr!(
                "1006b9c0",
                param_3,
                write_unaligned,
                mech_trace::rd!(
                    "1006b9c0",
                    base.byte_add(u_var2.wrapping_mul(0xc) as usize)
                        .cast::<u32>(),
                    read_unaligned
                )
            );
            return mech_trace::rt!("1006b9c0", (u_var2.wrapping_mul(3) & 0xffffff00) | 1);
        }
        mech_trace::rt!("1006b9c0", scan_kind2(5, param_2))
    }
}
