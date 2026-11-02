// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ad6e0: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100b0a14: i32;
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_1016a600: [u8; 1];
    static mut DAT_100b0de0: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _toupper(c: i32) -> i32;
    fn FUN_10052813(c: *mut i8);
    fn FUN_10012a14(c: i32, param_2: *mut i8) -> u32;
    fn _sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1005be24(c: *mut i8, param_2: i32, param_3: i32, param_4: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100528ee(c: u32) -> u32 {
    mech_trace::enter!("100528ee", c);
    let mut local_58: [i8; 4] = [0; 4];
    let mut local_54: [i8; 80] = [0; 80];
    // SAFETY: the globals and callees are the game's own memory; the buffer
    // DAT_1016a600 is the game's 40+ byte input buffer, indexed as the original does.
    unsafe {
        let buf = (&raw mut DAT_1016a600).cast::<u8>();
        let mut local_5c: i32 = if mech_trace::gr!("100528ee", "100adf38", DAT_100adf38)
            == mech_trace::gr!("100528ee", "100ad6e0", DAT_100ad6e0)
        {
            0
        } else {
            mech_trace::gr!("100528ee", "100ad6e0", DAT_100ad6e0)
        };
        if (c & 0x400) != 0 {
            return mech_trace::rt!("100528ee", 0);
        }
        local_58[0] = c as i8;
        if (c as i8) == 8 {
            if 0 < mech_trace::gr!("100528ee", "100b0a14", DAT_100b0a14) {
                mech_trace::gw!(
                    "100528ee",
                    "100b0a14",
                    DAT_100b0a14,
                    mech_trace::gr!("100528ee", "100b0a14", DAT_100b0a14).wrapping_sub(1)
                );
            }
            mech_trace::dw!(
                "100528ee",
                buf.offset(mech_trace::gr!("100528ee", "100b0a14", DAT_100b0a14) as isize),
                0
            );
            return mech_trace::rt!("100528ee", 1);
        }
        if (c as i8) == 13 {
            if mech_trace::gr!("100528ee", "100ad6e0", DAT_100ad6e0) < 1 {
                if mech_trace::gr!("100528ee", "100ad6e0", DAT_100ad6e0) == -1 {
                    local_5c = -1;
                    mech_trace::gw!("100528ee", "100ad6e0", DAT_100ad6e0, 0);
                }
            } else {
                mech_trace::gw!("100528ee", "100ad6e0", DAT_100ad6e0, 0);
            }
        } else if (c as i8) == 0x1b {
            mech_trace::gw!("100528ee", "100ad6e0", DAT_100ad6e0, 0);
            let mut p = buf.cast::<u32>();
            let mut n = 10;
            while n != 0 {
                mech_trace::wr!("100528ee", p, write_unaligned, 0);
                p = p.add(1);
                n -= 1;
            }
            mech_trace::gw!("100528ee", "100b0a14", DAT_100b0a14, 0);
            return mech_trace::rt!("100528ee", 1);
        }
        mech_trace::rt!(
            "100528ee",
            if mech_trace::gr!("100528ee", "100b0a14", DAT_100b0a14) < 0x28 {
                if (c & 0x100) != 0 {
                    if mech_trace::gr!("100528ee", "100ad6e0", DAT_100ad6e0) != -1 {
                        return mech_trace::rt!("100528ee", 0);
                    }
                    if (c as i8) == b'e' as i8 {
                        local_5c = -3;
                    } else {
                        if (c as i8) != b'f' as i8 {
                            return mech_trace::rt!("100528ee", 0);
                        }
                        local_5c = -2;
                    }
                    mech_trace::gw!("100528ee", "100ad6e0", DAT_100ad6e0, 0);
                }
                if (c & 0x200) != 0 {
                    let r = mech_trace::cr!("100528ee", "_toupper", _toupper((c as i8) as i32));
                    local_58[0] = r as i8;
                    FUN_10052813(local_58.as_mut_ptr());
                }
                if mech_trace::gr!("100528ee", "100ad6e0", DAT_100ad6e0) == 0 {
                    mech_trace::cr!(
                        "100528ee",
                        "10012a14",
                        FUN_10012a14(local_5c, buf.cast::<i8>())
                    );
                    let tbl = (&raw mut DAT_100eef40).cast::<u32>();
                    let v = mech_trace::rd!(
                        "100528ee",
                        tbl.offset(mech_trace::gr!("100528ee", "100adf38", DAT_100adf38) as isize),
                        read_unaligned
                    )
                    .wrapping_add(0x110);
                    mech_trace::cr!(
                        "100528ee",
                        "_sprintf",
                        _sprintf(
                            local_54.as_mut_ptr(),
                            (&raw const DAT_100b0de0).cast::<i8>(),
                            v,
                            buf,
                        )
                    );
                    mech_trace::cr!(
                        "100528ee",
                        "1005be24",
                        FUN_1005be24(local_54.as_mut_ptr(), 1, 0x2d4, 0x32)
                    );
                    let mut p = buf.cast::<u32>();
                    let mut n = 10;
                    while n != 0 {
                        mech_trace::wr!("100528ee", p, write_unaligned, 0);
                        p = p.add(1);
                        n -= 1;
                    }
                    mech_trace::gw!("100528ee", "100b0a14", DAT_100b0a14, 0);
                    1
                } else if local_58[0] < 0x20 || 0x7e < local_58[0] {
                    0
                } else {
                    mech_trace::dw!(
                        "100528ee",
                        buf.offset(mech_trace::gr!("100528ee", "100b0a14", DAT_100b0a14) as isize),
                        local_58[0] as u8
                    );
                    mech_trace::gw!(
                        "100528ee",
                        "100b0a14",
                        DAT_100b0a14,
                        mech_trace::gr!("100528ee", "100b0a14", DAT_100b0a14).wrapping_add(1)
                    );
                    1
                }
            } else {
                1
            }
        )
    }
}
