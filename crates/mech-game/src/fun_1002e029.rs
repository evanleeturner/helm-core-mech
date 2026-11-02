// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0a20: i32;
    static mut DAT_100ad6e0: i32;
    static mut DAT_100af17c: i32;
    static mut DAT_101686f0: i32;
    static mut PTR_DAT_100af0b8: *mut i32;
    static mut DAT_100ab0a4: i32;
    static mut DAT_100adf34: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [i32; 0];
    static mut DAT_100eebb0: [u8; 0];
    static mut DAT_100eebbe: u8;
    static mut DAT_1016a600: [u8; 0];
    static mut DAT_100ad814: [i8; 0];
    static mut DAT_100ad9a4: [i8; 0];
    static mut DAT_100ad818: [i8; 0];
    static mut DAT_100ad820: [i8; 0];
    static mut DAT_100ad830: [i8; 0];
    static mut DAT_100ad864: [i8; 0];
    static mut DAT_100ad88c: [i8; 0];
    static mut DAT_100ad89c: [i8; 0];
    static mut DAT_100ad8ac: [i8; 0];
    static mut DAT_100ad8c4: [i8; 0];
    static mut DAT_100ad8e4: [i8; 0];
    static mut DAT_100ad900: [i8; 0];
    static mut DAT_100ad910: [i8; 0];
    static mut DAT_100ad93c: [i8; 0];
    static mut DAT_100ad950: [i8; 0];
    static mut DAT_100ad978: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_1005aae4(param_1: i32) -> i32;
    fn FUN_1004d5e3(param_1: i32) -> i32;
    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn FUN_1005acaa(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut u8,
        param_6: i32,
    );
    fn FUN_1006670a(
        param_1: *mut i32,
        param_2: *mut i8,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
    fn FUN_100667a2(
        param_1: *mut i32,
        param_2: *mut i8,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
    fn FUN_1002d49d(param_1: *mut i8, param_2: i32) -> i32;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002e029(param_1: i32) {
    mech_trace::enter!("1002e029", param_1);
    // SAFETY: all globals and callees are the game's own; param_1 points at the
    // caller's structure, read at the same offsets as the original.
    unsafe {
        let mut local_3c: u8 = 0xe;
        let mut local_4c: *mut i8 = (&raw mut DAT_100ad814).cast::<i8>();
        let mut local_2c = [0u8; 40];
        let arr = (&raw mut DAT_100eef40).cast::<i32>();
        let buf = (&raw mut DAT_100eebb0) as usize as i32;
        let buf_p = (&raw mut DAT_100eebb0).cast::<u8>();
        let dat_16a600 = (&raw mut DAT_1016a600).cast::<u8>();

        if mech_trace::rd!(
            "1002e029",
            (param_1.wrapping_add(4) as *const i16),
            read_unaligned
        ) == 0
        {
            return;
        }
        if !(mech_trace::gr!("1002e029", "100b0a20", DAT_100b0a20) != 0
            || mech_trace::gr!("1002e029", "100ad6e0", DAT_100ad6e0) != 0)
        {
            return;
        }
        let puVar1 = mech_trace::cr!(
            "1002e029",
            "1008668b",
            FUN_1008668b(
                mech_trace::gr!("1002e029", "100af17c", DAT_100af17c),
                (mech_trace::gr!("1002e029", "101686f0", DAT_101686f0).wrapping_add(1)) as u32,
                mech_trace::gr!("1002e029", "100af0b8", PTR_DAT_100af0b8),
            )
        );
        if puVar1.is_null() {
            return;
        }
        let pv = puVar1 as usize as i32;
        let i_var2 = mech_trace::cr!("1002e029", "1005aae4", FUN_1005aae4(pv));
        let i_var3 = i_var2 / 2;
        let a8 = mech_trace::rd!(
            "1002e029",
            (param_1.wrapping_add(0xa8) as *const i32),
            read_unaligned
        ) as *const i32;
        let mut i_var5 = mech_trace::rd!("1002e029", a8, read_unaligned);
        let mut local_40 = mech_trace::rd!("1002e029", a8.byte_add(4), read_unaligned);
        let p4 = mech_trace::rd!(
            "1002e029",
            (param_1.wrapping_add(0xa4) as *const i32),
            read_unaligned
        ) as *mut i32;

        if mech_trace::gr!("1002e029", "100b0a20", DAT_100b0a20) == 1 {
            let i_var4 = mech_trace::cr!(
                "1002e029",
                "1004d5e3",
                FUN_1004d5e3(mech_trace::gr!("1002e029", "100ab0a4", DAT_100ab0a4))
            );
            if i_var4 != 0 {
                local_3c = 0xb;
            }
            let m = mech_trace::rd!(
                "1002e029",
                arr.offset(mech_trace::gr!("1002e029", "100ab0a4", DAT_100ab0a4) as isize),
                read_unaligned
            );
            if (mech_trace::rd!(
                "1002e029",
                (m.wrapping_add(0x14) as *const u16),
                read_unaligned
            ) & 6)
                != 0
            {
                local_4c = (&raw mut DAT_100ad818).cast::<i8>();
            }
            mech_trace::cr!(
                "1002e029",
                "_sprintf",
                _sprintf(
                    local_2c.as_mut_ptr().cast::<i8>(),
                    (&raw mut DAT_100ad820).cast::<i8>(),
                    m.wrapping_add(0x110),
                    local_4c,
                )
            );
            mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, local_3c);
            FUN_1005acaa(p4, i_var5, local_40, pv, local_2c.as_mut_ptr(), buf);
            mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
        } else if mech_trace::gr!("1002e029", "100b0a20", DAT_100b0a20) == 2 {
            mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 6);
            FUN_1005acaa(
                p4,
                i_var5,
                local_40,
                pv,
                (&raw mut DAT_100ad830).cast::<u8>(),
                buf,
            );
            mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
        } else if mech_trace::gr!("1002e029", "100b0a20", DAT_100b0a20) == 3 {
            mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
            FUN_1005acaa(
                p4,
                i_var5,
                local_40,
                pv,
                (&raw mut DAT_100ad864).cast::<u8>(),
                buf,
            );
            mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
        }
        let _ = buf_p;
        local_40 = local_40.wrapping_add(i_var3).wrapping_add(i_var2);
        if mech_trace::gr!("1002e029", "100ad6e0", DAT_100ad6e0) != 0 {
            if mech_trace::gr!("1002e029", "100ad6e0", DAT_100ad6e0) == -1 {
                mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
                FUN_1005acaa(
                    p4,
                    i_var5,
                    local_40,
                    pv,
                    (&raw mut DAT_100ad88c).cast::<u8>(),
                    buf,
                );
                mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
                FUN_1006670a(
                    p4,
                    (&raw mut DAT_100ad89c).cast::<i8>(),
                    i_var5,
                    local_40,
                    pv,
                    2,
                );
                local_40 = local_40.wrapping_add(i_var3).wrapping_add(i_var2);
                if mech_trace::rd!(
                    "1002e029",
                    ((mech_trace::gr!("1002e029", "100adf34", DAT_100adf34).wrapping_add(10))
                        as *const i8),
                    read
                ) == 0
                {
                    mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
                    FUN_1005acaa(
                        p4,
                        i_var5,
                        local_40,
                        pv,
                        (&raw mut DAT_100ad910).cast::<u8>(),
                        buf,
                    );
                } else {
                    mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
                    FUN_1005acaa(
                        p4,
                        i_var5,
                        local_40,
                        pv,
                        (&raw mut DAT_100ad8ac).cast::<u8>(),
                        buf,
                    );
                    mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
                    FUN_1005acaa(
                        p4,
                        i_var5,
                        local_40.wrapping_add(i_var2),
                        pv,
                        (&raw mut DAT_100ad8c4).cast::<u8>(),
                        buf,
                    );
                    local_40 = local_40.wrapping_add(i_var2).wrapping_add(i_var2);
                    mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
                    FUN_1005acaa(
                        p4,
                        i_var5,
                        local_40,
                        pv,
                        (&raw mut DAT_100ad8e4).cast::<u8>(),
                        buf,
                    );
                    local_40 = local_40.wrapping_add(i_var2);
                    mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
                    FUN_1005acaa(
                        p4,
                        i_var5,
                        local_40,
                        pv,
                        (&raw mut DAT_100ad900).cast::<u8>(),
                        buf,
                    );
                }
                local_40 = local_40.wrapping_add(i_var3).wrapping_add(i_var2);
            } else if 0 < mech_trace::gr!("1002e029", "100ad6e0", DAT_100ad6e0)
                && mech_trace::gr!("1002e029", "100ad6e0", DAT_100ad6e0) < 8
            {
                let local_34 = if mech_trace::gr!("1002e029", "100adf38", DAT_100adf38)
                    == mech_trace::gr!("1002e029", "100ad6e0", DAT_100ad6e0)
                {
                    0
                } else {
                    mech_trace::gr!("1002e029", "100ad6e0", DAT_100ad6e0)
                };
                let m = mech_trace::rd!("1002e029", arr.offset(local_34 as isize), read_unaligned);
                if m == 0 {
                    return;
                }
                if mech_trace::rd!("1002e029", (m as *const i32), read_unaligned) != 1 {
                    return;
                }
                if (mech_trace::rd!(
                    "1002e029",
                    (m.wrapping_add(0x14) as *const u16),
                    read_unaligned
                ) & 0x4800)
                    != 0
                {
                    return;
                }
                mech_trace::cr!(
                    "1002e029",
                    "_sprintf",
                    _sprintf(
                        local_2c.as_mut_ptr().cast::<i8>(),
                        (&raw mut DAT_100ad93c).cast::<i8>(),
                        m.wrapping_add(0x110),
                    )
                );
                mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
                FUN_1005acaa(p4, i_var5, local_40, pv, local_2c.as_mut_ptr(), buf);
                mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
                FUN_1006670a(
                    p4,
                    local_2c.as_mut_ptr().cast::<i8>(),
                    i_var5,
                    local_40,
                    pv,
                    2,
                );
                local_40 = local_40.wrapping_add(i_var3).wrapping_add(i_var2);
                mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
                FUN_1005acaa(
                    p4,
                    i_var5,
                    local_40,
                    pv,
                    (&raw mut DAT_100ad950).cast::<u8>(),
                    buf,
                );
                local_40 = local_40.wrapping_add(i_var3).wrapping_add(i_var2);
            }
            mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
            FUN_1005acaa(p4, i_var5, local_40, pv, dat_16a600, buf);
            mech_trace::gw!("1002e029", "100eebbe", DAT_100eebbe, 0xe);
            FUN_100667a2(
                p4,
                (&raw mut DAT_100ad978).cast::<i8>(),
                i_var5,
                local_40,
                pv,
                6,
            );
            i_var5 = mech_trace::cr!(
                "1002e029",
                "1002d49d",
                FUN_1002d49d(dat_16a600.cast::<i8>(), pv)
            );
            FUN_1006670a(
                p4,
                (&raw mut DAT_100ad9a4).cast::<i8>(),
                i_var5,
                local_40,
                pv,
                0xe,
            );
        }
        FUN_1008664f(
            (mech_trace::gr!("1002e029", "101686f0", DAT_101686f0).wrapping_add(1)) as u32,
            mech_trace::gr!("1002e029", "100af0b8", PTR_DAT_100af0b8),
        );
    }
}
