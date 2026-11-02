// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_10160f48: i32;
    static mut _DAT_10160f44: i32;
    static mut _DAT_10160f4c: i32;
    static mut DAT_10160f38: i32;
    static mut DAT_10160f3c: i32;
    static mut DAT_10160f40: i32;
    static mut DAT_10160f54: i32;
    static mut DAT_10160f20: i32;
    static mut DAT_10160f24: i32;
    static mut DAT_10160f28: i32;
    static mut DAT_10160f2c: i32;
    static mut DAT_10160f30: i32;
    static mut DAT_10160f34: i32;
    static mut DAT_10115154: i32;
    static mut PTR_DAT_100aaf68: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10035980(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
    fn FUN_1003c230(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003bed0(param_1: *mut u16) -> u32 {
    mech_trace::enter!("1003bed0", param_1);
    // SAFETY: param_1 points at the caller's object; the globals and callees
    // are the game's own, and everything is read/written in the original order.
    unsafe {
        let i_var1 = mech_trace::rd!(
            "1003bed0",
            param_1.byte_add(0x40).cast::<i32>(),
            read_unaligned
        );
        let mut i_var2 = mech_trace::rd!(
            "1003bed0",
            param_1.byte_add(0x34).cast::<i32>(),
            read_unaligned
        )
        .wrapping_sub(mech_trace::gr!("1003bed0", "10160f48", _DAT_10160f48));
        let i_var3 = mech_trace::rd!(
            "1003bed0",
            param_1.byte_add(0x38).cast::<i32>(),
            read_unaligned
        )
        .wrapping_sub(mech_trace::gr!("1003bed0", "10160f44", _DAT_10160f44));
        let i_var4 = mech_trace::rd!(
            "1003bed0",
            param_1.byte_add(0x3c).cast::<i32>(),
            read_unaligned
        )
        .wrapping_sub(mech_trace::gr!("1003bed0", "10160f4c", _DAT_10160f4c));
        if (mech_trace::rd!("1003bed0", param_1, read_unaligned) & 0x1000) != 0 {
            return mech_trace::rt!("1003bed0", 1);
        }
        let i_var6 = mech_trace::cr!(
            "1003bed0",
            "10035980",
            FUN_10035980(
                i_var2,
                mech_trace::gr!("1003bed0", "10160f38", DAT_10160f38),
                i_var3,
                mech_trace::gr!("1003bed0", "10160f3c", DAT_10160f3c),
                i_var4,
                mech_trace::gr!("1003bed0", "10160f40", DAT_10160f40),
            )
        );
        mech_trace::gw!("1003bed0", "10115154", DAT_10115154, i_var6);
        if i_var6.wrapping_add(i_var1) < mech_trace::gr!("1003bed0", "10160f54", DAT_10160f54) {
            return mech_trace::rt!("1003bed0", 4);
        }
        let i_var7 = mech_trace::cr!(
            "1003bed0",
            "10035980",
            FUN_10035980(
                i_var2,
                mech_trace::gr!("1003bed0", "10160f20", DAT_10160f20),
                i_var3,
                mech_trace::gr!("1003bed0", "10160f24", DAT_10160f24),
                i_var4,
                mech_trace::gr!("1003bed0", "10160f28", DAT_10160f28),
            )
        );
        let base = mech_trace::gr!("1003bed0", "100aaf68", PTR_DAT_100aaf68);
        let mut local_14;
        if i_var7 < 1 {
            local_14 = mech_trace::cr!(
                "1003bed0",
                "1003c230",
                FUN_1003c230(
                    mech_trace::rd!(
                        "1003bed0",
                        base.byte_add(0x18).cast::<i32>(),
                        read_unaligned
                    ),
                    i_var7.wrapping_neg(),
                    i_var6.wrapping_neg(),
                    mech_trace::rd!(
                        "1003bed0",
                        base.byte_add(0xa8).cast::<i32>(),
                        read_unaligned
                    ),
                )
            );
        } else {
            local_14 = mech_trace::cr!(
                "1003bed0",
                "1003c230",
                FUN_1003c230(
                    mech_trace::rd!(
                        "1003bed0",
                        base.byte_add(0x18).cast::<i32>(),
                        read_unaligned
                    ),
                    i_var7,
                    i_var6.wrapping_neg(),
                    mech_trace::rd!(
                        "1003bed0",
                        base.byte_add(0xa8).cast::<i32>(),
                        read_unaligned
                    ),
                )
            );
        }
        if i_var1 < local_14 {
            return mech_trace::rt!("1003bed0", 6);
        }
        i_var2 = mech_trace::cr!(
            "1003bed0",
            "10035980",
            FUN_10035980(
                i_var2,
                mech_trace::gr!("1003bed0", "10160f2c", DAT_10160f2c),
                i_var3,
                mech_trace::gr!("1003bed0", "10160f30", DAT_10160f30),
                i_var4,
                mech_trace::gr!("1003bed0", "10160f34", DAT_10160f34),
            )
        );
        let base = mech_trace::gr!("1003bed0", "100aaf68", PTR_DAT_100aaf68);
        if i_var2 < 1 {
            local_14 = mech_trace::cr!(
                "1003bed0",
                "1003c230",
                FUN_1003c230(
                    mech_trace::rd!(
                        "1003bed0",
                        base.byte_add(0xb0).cast::<i32>(),
                        read_unaligned
                    ),
                    i_var2.wrapping_neg(),
                    i_var6.wrapping_neg(),
                    mech_trace::rd!(
                        "1003bed0",
                        base.byte_add(0xac).cast::<i32>(),
                        read_unaligned
                    ),
                )
            );
        } else {
            local_14 = mech_trace::cr!(
                "1003bed0",
                "1003c230",
                FUN_1003c230(
                    mech_trace::rd!(
                        "1003bed0",
                        base.byte_add(0xb0).cast::<i32>(),
                        read_unaligned
                    ),
                    i_var2,
                    i_var6.wrapping_neg(),
                    mech_trace::rd!(
                        "1003bed0",
                        base.byte_add(0xac).cast::<i32>(),
                        read_unaligned
                    ),
                )
            );
        }
        mech_trace::rt!("1003bed0", if i_var1 < local_14 { 7 } else { 0 })
    }
}
