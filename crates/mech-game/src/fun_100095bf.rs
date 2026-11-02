// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7310: *mut i32;
    static mut DAT_100af17c: u32;
    static mut DAT_100b0710: u32;
    static mut DAT_100b9f60: i32;
    static mut DAT_101686f0: u32;
    static mut DAT_10215670: u8;
    static mut DAT_10215674: u8;
    static mut DAT_1021567c: u8;
    static mut DAT_10215688: u8;
    static mut DAT_1021568c: u8;
    static mut DAT_10215690: u8;
    static mut DAT_10215694: u8;
    static mut DAT_100af0b4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_100357e3(param_1: *mut i32) -> u32;
    fn FUN_100091ee(param_1: i32, param_2: u8) -> u32;
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_10056fc4(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> u32;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100095bf(param_1: *mut i32) {
    mech_trace::enter!("100095bf", param_1);
    // SAFETY: the globals, tables and callees are the game's own memory;
    // param_1 points at the caller's structure, accessed as the original does.
    unsafe {
        let mut local_20: i32 = 0;
        let mut local_1c: i32 = 0;
        let mut local_18: u32 = 0;
        let mut local_10: *mut i32 =
            mech_trace::rd!("100095bf", param_1, read_unaligned) as usize as *mut i32;
        if !mech_trace::gr!("100095bf", "100a7310", DAT_100a7310).is_null() {
            local_10 = mech_trace::gr!("100095bf", "100a7310", DAT_100a7310);
        }
        let mut local_14: i32 = 0;
        while local_14 < mech_trace::gr!("100095bf", "100b9f60", DAT_100b9f60) {
            let i_var1: i32 = local_14.wrapping_mul(0x54);
            let b70 = (&raw mut DAT_10215670) as usize as i32;
            let b74 = (&raw mut DAT_10215674) as usize as i32;
            let b7c = (&raw mut DAT_1021567c) as usize as i32;
            let b88 = (&raw mut DAT_10215688) as usize as i32;
            let b8c = (&raw mut DAT_1021568c) as usize as i32;
            let b90 = (&raw mut DAT_10215690) as usize as i32;
            let b94 = (&raw mut DAT_10215694) as usize as i32;
            if mech_trace::rd!(
                "100095bf",
                (b7c.wrapping_add(i_var1) as usize as *const i32),
                read_unaligned
            ) as u32
                == mech_trace::gr!("100095bf", "100b0710", DAT_100b0710)
                && mech_trace::rd!(
                    "100095bf",
                    (b70.wrapping_add(i_var1) as usize as *const i32),
                    read_unaligned
                ) != 0
                && (mech_trace::rd!(
                    "100095bf",
                    (b94.wrapping_add(i_var1) as usize as *const u16),
                    read_unaligned
                ) & 1)
                    == 0
            {
                let v74 = mech_trace::rd!(
                    "100095bf",
                    (b74.wrapping_add(i_var1) as usize as *const i32),
                    read_unaligned
                );
                if v74 == 0 {
                    local_20 = mech_trace::rd!(
                        "100095bf",
                        (b88.wrapping_add(i_var1) as usize as *const i32),
                        read_unaligned
                    );
                    local_1c = mech_trace::rd!(
                        "100095bf",
                        (b8c.wrapping_add(i_var1) as usize as *const i32),
                        read_unaligned
                    );
                    local_18 = mech_trace::rd!(
                        "100095bf",
                        (b90.wrapping_add(i_var1) as usize as *const u32),
                        read_unaligned
                    );
                } else {
                    mech_trace::cr!(
                        "100095bf",
                        "1002f322",
                        FUN_1002f322(
                            v74,
                            (&raw mut local_20).cast::<u32>(),
                            (&raw mut local_1c).cast::<u32>(),
                            &raw mut local_18,
                        )
                    );
                }
                let mut local_8: i32 =
                    mech_trace::cr!("100095bf", "100357e3", FUN_100357e3(&raw mut local_20)) as i32;
                if local_8 != 0
                    && mech_trace::rd!("100095bf", param_1.add(0x20), read_unaligned) != 0
                {
                    let f: unsafe extern "C" fn(*mut i32, i32, i32) -> i32 = core::mem::transmute(
                        mech_trace::rd!("100095bf", param_1.add(0x20), read_unaligned) as u32
                            as usize,
                    );
                    local_8 = mech_trace::cr!("100095bf", "ind", f(local_10, local_20, local_1c));
                }
                if local_8 != 0 {
                    let r = mech_trace::cr!(
                        "100095bf",
                        "100091ee",
                        FUN_100091ee(
                            b70.wrapping_add(i_var1),
                            mech_trace::gr!("100095bf", "100b0710", DAT_100b0710) as u8
                        )
                    );
                    let p1c =
                        mech_trace::rd!("100095bf", param_1.add(0x1c), read_unaligned) as usize;
                    let local_c: i32 = if r == 0 {
                        mech_trace::rd!("100095bf", ((p1c + 0x30) as *const i32), read_unaligned)
                    } else {
                        mech_trace::rd!("100095bf", ((p1c + 0x34) as *const i32), read_unaligned)
                    };
                    if local_c != -1 {
                        let puv2 = mech_trace::cr!(
                            "100095bf",
                            "1008668b",
                            FUN_1008668b(
                                mech_trace::gr!("100095bf", "100af17c", DAT_100af17c) as i32,
                                mech_trace::gr!("100095bf", "101686f0", DAT_101686f0)
                                    .wrapping_add(local_c as u32),
                                mech_trace::gr!("100095bf", "100af0b4", DAT_100af0b4) as usize
                                    as *mut i32,
                            )
                        );
                        if !puv2.is_null() {
                            mech_trace::cr!(
                                "100095bf",
                                "10056fc4",
                                FUN_10056fc4(local_10, puv2 as usize as i32, 0, local_20, local_1c)
                            );
                            mech_trace::cr!(
                                "100095bf",
                                "1008664f",
                                FUN_1008664f(
                                    mech_trace::gr!("100095bf", "101686f0", DAT_101686f0)
                                        .wrapping_add(local_c as u32),
                                    mech_trace::gr!("100095bf", "100af0b4", DAT_100af0b4) as usize
                                        as *mut i32,
                                )
                            );
                        }
                    }
                }
            }
            local_14 = local_14.wrapping_add(1);
        }
    }
}
