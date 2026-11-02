// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0a18: i32;
    static mut DAT_100b0a28: *mut i8;
    static mut DAT_100b0a2c: *mut i8;
    static mut DAT_100b0a30: *mut i8;
    static mut DAT_100b0a34: *mut i8;
    static mut DAT_100b0a38: *mut i8;
    static mut DAT_100b0a3c: *mut i8;
    static mut DAT_100b0a40: *mut i8;
    static mut DAT_100b0a44: *mut i8;
    static mut DAT_100b0a48: *mut i8;
    static mut DAT_100b0a4c: *mut i8;
    static mut DAT_100b0a58: *mut i8;
    static mut DAT_100b0a64: *mut i8;
    static mut DAT_100b0a68: *mut i8;
    static mut DAT_100b0a70: *mut i8;
    static mut DAT_100b0a7c: *mut i8;
    static mut DAT_100b0a88: *mut i8;
    static mut DAT_100b0a8c: *mut i8;
    static mut DAT_100b0a90: *mut i8;
    static mut DAT_100b0a94: *mut i8;
    static mut DAT_100b0a98: *mut i8;
    static mut DAT_100b0aa0: *mut i8;
    static mut DAT_100b0aa4: *mut i8;
    static mut DAT_100b0aa8: *mut i8;
    static mut DAT_100b0aac: *mut i8;
    static mut DAT_100b0ab0: *mut i8;
    static mut DAT_100b0ab8: *mut i8;
    static mut DAT_100b0abc: *mut i8;
    static mut DAT_100b0ac0: *mut i8;
    static mut DAT_100b0ac4: *mut i8;
    static mut DAT_100b0ac8: *mut i8;
    static mut DAT_100b0acc: *mut i8;
    static mut DAT_100b0ad0: *mut i8;
    static mut DAT_100b0ad4: *mut i8;
    static mut DAT_100b0ad8: *mut i8;
    static mut DAT_100b0dbc: [i8; 1];
    static mut DAT_100adf34: *mut i8;
    static mut DAT_100adc08: u32;
    static mut DAT_100ad0a8: u32;
    static mut DAT_100b3918: u32;
    static mut DAT_100ab2f8: u32;
    static mut DAT_100a9f44: u32;
    static mut DAT_100a7b48: u32;
    static mut DAT_100b0a0c: u32;
    static mut DAT_100b07d8: u32;
    static mut DAT_10215664: u32;
    static mut DAT_100ba99d: i8;
    static mut DAT_100eef40: [i32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10052143();
    fn FUN_10052080(param_1: u16) -> u8;
    fn FUN_100520c7(param_1: *mut i8) -> i32;
    fn FUN_1005be24(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_10083715() -> u32;
    fn FUN_1004c173(param_1: i32);
    fn FUN_10005207(param_1: i32, param_2: *mut i32, param_3: i32);
    fn FUN_1001a5e1(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10052235(param_1: u16) {
    mech_trace::enter!("10052235", param_1);
    // SAFETY: all globals and callees are the game's own; the pointers in
    // the globals and the table DAT_100eef40 are valid in the original.
    unsafe {
        if mech_trace::gr!("10052235", "100b0a18", DAT_100b0a18) == 0 {
            mech_trace::gw!("10052235", "100b0a18", DAT_100b0a18, 1);
            FUN_10052143();
        }
        if mech_trace::cr!("10052235", "10052080", FUN_10052080(param_1)) == 0 {
            return;
        }
        if mech_trace::cr!(
            "10052235",
            "100520c7",
            FUN_100520c7(mech_trace::gr!("10052235", "100b0a28", DAT_100b0a28))
        ) != 0
        {
            if mech_trace::dr!(
                "10052235",
                mech_trace::gr!("10052235", "100adf34", DAT_100adf34).add(1)
            ) == 0
            {
                mech_trace::dw!(
                    "10052235",
                    mech_trace::gr!("10052235", "100adf34", DAT_100adf34).add(1),
                    1
                );
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0a2c", DAT_100b0a2c),
                    1,
                    0x16a,
                    0x32,
                );
            } else {
                mech_trace::dw!(
                    "10052235",
                    mech_trace::gr!("10052235", "100adf34", DAT_100adf34).add(1),
                    0
                );
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0a30", DAT_100b0a30),
                    1,
                    0x16a,
                    0x32,
                );
            }
            return;
        }
        if mech_trace::cr!(
            "10052235",
            "100520c7",
            FUN_100520c7(mech_trace::gr!("10052235", "100b0a34", DAT_100b0a34))
        ) != 0
        {
            if mech_trace::dr!(
                "10052235",
                mech_trace::gr!("10052235", "100adf34", DAT_100adf34)
            ) == 0
            {
                mech_trace::dw!(
                    "10052235",
                    mech_trace::gr!("10052235", "100adf34", DAT_100adf34),
                    1
                );
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0a38", DAT_100b0a38),
                    1,
                    0x16a,
                    0x32,
                );
            } else {
                mech_trace::dw!(
                    "10052235",
                    mech_trace::gr!("10052235", "100adf34", DAT_100adf34),
                    0
                );
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0a3c", DAT_100b0a3c),
                    1,
                    0x16a,
                    0x32,
                );
            }
            return;
        }
        if mech_trace::cr!(
            "10052235",
            "100520c7",
            FUN_100520c7(mech_trace::gr!("10052235", "100b0a40", DAT_100b0a40))
        ) != 0
        {
            if mech_trace::dr!(
                "10052235",
                mech_trace::gr!("10052235", "100adf34", DAT_100adf34).add(4)
            ) == 0
            {
                mech_trace::dw!(
                    "10052235",
                    mech_trace::gr!("10052235", "100adf34", DAT_100adf34).add(4),
                    1
                );
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0a44", DAT_100b0a44),
                    1,
                    0x16a,
                    0x32,
                );
            } else {
                mech_trace::dw!(
                    "10052235",
                    mech_trace::gr!("10052235", "100adf34", DAT_100adf34).add(4),
                    0
                );
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0a48", DAT_100b0a48),
                    1,
                    0x16a,
                    0x32,
                );
            }
            return;
        }
        if mech_trace::cr!(
            "10052235",
            "100520c7",
            FUN_100520c7(mech_trace::gr!("10052235", "100b0a4c", DAT_100b0a4c))
        ) != 0
        {
            FUN_1005be24((&raw mut DAT_100b0dbc).cast::<i8>(), 1, 0x16a, 0x32);
            return;
        }
        if mech_trace::cr!(
            "10052235",
            "100520c7",
            FUN_100520c7(mech_trace::gr!("10052235", "100b0a58", DAT_100b0a58))
        ) != 0
        {
            let u = mech_trace::cr!("10052235", "10083715", FUN_10083715());
            if -1 < (u as i32) {
                let e = mech_trace::rd!(
                    "10052235",
                    (&raw const DAT_100eef40).cast::<i32>().add(u as usize),
                    read
                );
                FUN_1004c173(e);
            }
            return;
        }
        if mech_trace::cr!(
            "10052235",
            "100520c7",
            FUN_100520c7(mech_trace::gr!("10052235", "100b0a64", DAT_100b0a64))
        ) != 0
        {
            mech_trace::gw!("10052235", "100ba99d", DAT_100ba99d, 1);
            FUN_1005be24(
                mech_trace::gr!("10052235", "100b0a68", DAT_100b0a68),
                1,
                0x16a,
                0x32,
            );
            return;
        }
        if mech_trace::cr!(
            "10052235",
            "100520c7",
            FUN_100520c7(mech_trace::gr!("10052235", "100b0a70", DAT_100b0a70))
        ) != 0
        {
            let u = mech_trace::cr!("10052235", "10083715", FUN_10083715());
            if -1 < (u as i32) {
                let e = mech_trace::rd!(
                    "10052235",
                    (&raw const DAT_100eef40).cast::<i32>().add(u as usize),
                    read
                );
                let p = mech_trace::rd!(
                    "10052235",
                    ((e as u32).wrapping_add(0x20) as *const *mut i32),
                    read_unaligned
                );
                FUN_10005207(-2, p, 3);
            }
            return;
        }
        if mech_trace::cr!(
            "10052235",
            "100520c7",
            FUN_100520c7(mech_trace::gr!("10052235", "100b0a7c", DAT_100b0a7c))
        ) != 0
        {
            mech_trace::gw!("10052235", "100b07d8", DAT_100b07d8, 1);
            return;
        }
        if mech_trace::cr!(
            "10052235",
            "100520c7",
            FUN_100520c7(mech_trace::gr!("10052235", "100b0a88", DAT_100b0a88))
        ) != 0
        {
            mech_trace::gw!(
                "10052235",
                "100b0a0c",
                DAT_100b0a0c,
                (mech_trace::gr!("10052235", "100b0a0c", DAT_100b0a0c) == 0) as u32
            );
            if mech_trace::gr!("10052235", "100b0a0c", DAT_100b0a0c) == 0 {
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0a90", DAT_100b0a90),
                    1,
                    0xb5,
                    0x32,
                );
            } else {
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0a8c", DAT_100b0a8c),
                    1,
                    0xb5,
                    0x32,
                );
            }
            return;
        }
        if mech_trace::cr!(
            "10052235",
            "100520c7",
            FUN_100520c7(mech_trace::gr!("10052235", "100b0a94", DAT_100b0a94))
        ) != 0
        {
            mech_trace::gw!(
                "10052235",
                "100a9f44",
                DAT_100a9f44,
                (mech_trace::gr!("10052235", "100a9f44", DAT_100a9f44) == 0) as u32
            );
            mech_trace::gw!(
                "10052235",
                "100a7b48",
                DAT_100a7b48,
                (mech_trace::gr!("10052235", "100a7b48", DAT_100a7b48) == 0) as u32
            );
            FUN_1005be24(
                mech_trace::gr!("10052235", "100b0a98", DAT_100b0a98),
                1,
                0x16a,
                0x32,
            );
            return;
        }
        if mech_trace::cr!(
            "10052235",
            "100520c7",
            FUN_100520c7(mech_trace::gr!("10052235", "100b0aa0", DAT_100b0aa0))
        ) != 0
        {
            if mech_trace::gr!("10052235", "100ab2f8", DAT_100ab2f8) == 0 {
                mech_trace::gw!("10052235", "100ab2f8", DAT_100ab2f8, 1);
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0aa4", DAT_100b0aa4),
                    1,
                    0x16a,
                    0x32,
                );
            } else {
                mech_trace::gw!("10052235", "100ab2f8", DAT_100ab2f8, 0);
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0aa8", DAT_100b0aa8),
                    1,
                    0x16a,
                    0x32,
                );
            }
            return;
        }
        if mech_trace::cr!(
            "10052235",
            "100520c7",
            FUN_100520c7(mech_trace::gr!("10052235", "100b0aac", DAT_100b0aac))
        ) != 0
        {
            FUN_1001a5e1(2);
            mech_trace::gw!("10052235", "10215664", DAT_10215664, 1);
            FUN_1005be24(
                mech_trace::gr!("10052235", "100b0ab0", DAT_100b0ab0),
                1,
                0x16a,
                0x32,
            );
            return;
        }
        if mech_trace::cr!(
            "10052235",
            "100520c7",
            FUN_100520c7(mech_trace::gr!("10052235", "100b0ab8", DAT_100b0ab8))
        ) != 0
        {
            mech_trace::gw!(
                "10052235",
                "100b3918",
                DAT_100b3918,
                (mech_trace::gr!("10052235", "100b3918", DAT_100b3918) == 0) as u32
            );
            if mech_trace::gr!("10052235", "100b3918", DAT_100b3918) == 0 {
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0ac0", DAT_100b0ac0),
                    1,
                    0x16a,
                    0x32,
                );
            } else {
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0abc", DAT_100b0abc),
                    1,
                    0x16a,
                    0x32,
                );
            }
            return;
        }
        if mech_trace::cr!(
            "10052235",
            "100520c7",
            FUN_100520c7(mech_trace::gr!("10052235", "100b0ac4", DAT_100b0ac4))
        ) != 0
        {
            mech_trace::gw!(
                "10052235",
                "100ad0a8",
                DAT_100ad0a8,
                (mech_trace::gr!("10052235", "100ad0a8", DAT_100ad0a8) == 0) as u32
            );
            if mech_trace::gr!("10052235", "100ad0a8", DAT_100ad0a8) == 0 {
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0acc", DAT_100b0acc),
                    1,
                    0x16a,
                    0x32,
                );
            } else {
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0ac8", DAT_100b0ac8),
                    1,
                    0x16a,
                    0x32,
                );
            }
            return;
        }
        if mech_trace::cr!(
            "10052235",
            "100520c7",
            FUN_100520c7(mech_trace::gr!("10052235", "100b0ad0", DAT_100b0ad0))
        ) != 0
        {
            mech_trace::gw!(
                "10052235",
                "100adc08",
                DAT_100adc08,
                (mech_trace::gr!("10052235", "100adc08", DAT_100adc08) == 0) as u32
            );
            if mech_trace::gr!("10052235", "100adc08", DAT_100adc08) == 0 {
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0ad8", DAT_100b0ad8),
                    1,
                    0x16a,
                    0x32,
                );
            } else {
                FUN_1005be24(
                    mech_trace::gr!("10052235", "100b0ad4", DAT_100b0ad4),
                    1,
                    0x16a,
                    0x32,
                );
            }
        }
    }
}
