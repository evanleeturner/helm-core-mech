// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7310: *mut i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [i32; 1];
    static mut DAT_100b0710: i32;
    static mut DAT_10215670: [u8; 1];
    static mut DAT_100ef030: [u8; 1];
    static mut DAT_100af17c: i32;
    static mut DAT_101686f0: i32;
    static mut PTR_DAT_100af0b4: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100357e3(param_1: *mut i32) -> i32;
    fn FUN_100091ee(param_1: i32, param_2: u8) -> i32;
    fn FUN_1004d5e3(param_1: u32) -> u32;
    fn FUN_1004d60d(param_1: u32) -> u32;
    fn FUN_1008668b(param_1: i32, param_2: i32, param_3: *mut i32) -> *mut u16;
    fn FUN_10056fc4(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> i32;
    fn FUN_1008664f(param_1: i32, param_2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10009232(param_1: *mut i32) {
    mech_trace::enter!("10009232", param_1);
    // SAFETY: all globals, callees and pointers belong to the game; reads and
    // writes occur in the same order as the original.
    unsafe {
        let mut local_20: *mut i32 =
            mech_trace::rd!("10009232", param_1, read_unaligned) as usize as *mut i32;
        let mut local_14: i32 = -1;
        let mut local_30: i32 = 2;
        if !mech_trace::gr!("10009232", "100a7310", DAT_100a7310).is_null() {
            local_20 = mech_trace::gr!("10009232", "100a7310", DAT_100a7310);
        }
        let idx = mech_trace::gr!("10009232", "100adf38", DAT_100adf38);
        let local_c: i32 = mech_trace::rd!(
            "10009232",
            (&raw mut DAT_100eef40).cast::<i32>().offset(idx as isize),
            read_unaligned
        );
        let mut local_2c: [i32; 3] = [0; 3];
        local_2c[0] = mech_trace::rd!(
            "10009232",
            (local_c.wrapping_add(200) as usize as *const i32),
            read_unaligned
        );
        local_2c[1] = mech_trace::rd!(
            "10009232",
            (local_c.wrapping_add(0xcc) as usize as *const i32),
            read_unaligned
        );
        local_2c[2] = mech_trace::rd!(
            "10009232",
            (local_c.wrapping_add(0xd0) as usize as *const i32),
            read_unaligned
        );
        let mut local_10: i32 =
            mech_trace::cr!("10009232", "100357e3", FUN_100357e3(local_2c.as_mut_ptr()));
        if local_10 != 0 && mech_trace::rd!("10009232", param_1.add(0x20), read_unaligned) != 0 {
            let f = core::mem::transmute::<i32, unsafe extern "C" fn(*mut i32, i32, i32) -> i32>(
                mech_trace::rd!("10009232", param_1.add(0x20), read_unaligned),
            );
            local_10 = mech_trace::cr!("10009232", "ind", f(local_20, local_2c[0], local_2c[1]));
        }
        let mut local_38: u32 = mech_trace::rd!(
            "10009232",
            (local_c.wrapping_add(0xdc) as usize as *const u32),
            read_unaligned
        );
        loop {
            if local_30 == 0 {
                return;
            }
            if local_38 != 0 && (local_38 & 0x1000) == 0 {
                let local_1c: u32 = local_38 & 0xf00;
                let local_8: u32 = local_38 & 0xff;
                let local_18: u32;
                let mut local_34: u32;
                if local_1c == 0x100 {
                    if local_10 == 0 {
                        local_18 = 0;
                    } else {
                        let addr = (&raw mut DAT_10215670 as *mut u8 as i32)
                            .wrapping_add(local_8.wrapping_mul(0x54) as i32);
                        let r = mech_trace::cr!(
                            "10009232",
                            "100091ee",
                            FUN_100091ee(
                                addr,
                                mech_trace::gr!("10009232", "100b0710", DAT_100b0710) as u8
                            )
                        );
                        local_18 = (r != 0) as u32;
                    }
                    local_34 = 5;
                } else if local_1c == 0x200 {
                    local_18 = mech_trace::cr!("10009232", "1004d5e3", FUN_1004d5e3(local_8));
                    let e = mech_trace::rd!(
                        "10009232",
                        (&raw mut DAT_100eef40).cast::<i32>().add(local_8 as usize),
                        read_unaligned
                    );
                    let w = mech_trace::rd!(
                        "10009232",
                        (e.wrapping_add(0x14) as usize as *const u16),
                        read_unaligned
                    );
                    if (w & 0x1400) == 0 || (w & 0x800) != 0 {
                        local_34 = 3;
                    } else {
                        local_34 = 2;
                    }
                } else {
                    if local_1c != 0x400 {
                        return;
                    }
                    local_18 = mech_trace::cr!("10009232", "1004d60d", FUN_1004d60d(local_8));
                    let w = mech_trace::rd!(
                        "10009232",
                        (&raw mut DAT_100ef030)
                            .cast::<u8>()
                            .add(local_8.wrapping_mul(0x40) as usize)
                            .cast::<u16>(),
                        read_unaligned
                    );
                    if (w & 0x1400) == 0 {
                        local_34 = 3;
                    } else {
                        local_34 = 2;
                    }
                }
                if local_10 == 0 {
                    local_34 = 6;
                }
                let base = mech_trace::rd!("10009232", param_1.add(0x1c), read_unaligned);
                let a = (base as u32)
                    .wrapping_add(local_34.wrapping_mul(0xc))
                    .wrapping_add(local_18.wrapping_mul(4));
                local_14 = mech_trace::rd!("10009232", (a as usize as *const i32), read_unaligned);
                if local_14 == 0 {
                    return;
                }
                let puvar2 = mech_trace::cr!(
                    "10009232",
                    "1008668b",
                    FUN_1008668b(
                        mech_trace::gr!("10009232", "100af17c", DAT_100af17c),
                        mech_trace::gr!("10009232", "101686f0", DAT_101686f0)
                            .wrapping_add(local_14),
                        mech_trace::gr!("10009232", "PTR_DAT_100af0b4", PTR_DAT_100af0b4),
                    )
                );
                if !puvar2.is_null() {
                    if local_10 == 0 {
                        if mech_trace::rd!("10009232", param_1.add(0x21), read_unaligned) != 0 {
                            let g = core::mem::transmute::<
                                i32,
                                unsafe extern "C" fn(*mut i32, *mut i32, *mut i32),
                            >(mech_trace::rd!(
                                "10009232",
                                param_1.add(0x21),
                                read_unaligned
                            ));
                            mech_trace::cr!(
                                "10009232",
                                "ind",
                                g(local_20, local_2c.as_mut_ptr(), local_2c.as_mut_ptr())
                            );
                            mech_trace::cr!(
                                "10009232",
                                "10056fc4",
                                FUN_10056fc4(
                                    local_20,
                                    puvar2 as usize as i32,
                                    0,
                                    local_2c[0],
                                    local_2c[1],
                                )
                            );
                        }
                    } else {
                        mech_trace::cr!(
                            "10009232",
                            "10056fc4",
                            FUN_10056fc4(
                                local_20,
                                puvar2 as usize as i32,
                                0,
                                local_2c[0],
                                local_2c[1],
                            )
                        );
                    }
                    FUN_1008664f(
                        mech_trace::gr!("10009232", "101686f0", DAT_101686f0)
                            .wrapping_add(local_14),
                        mech_trace::gr!("10009232", "PTR_DAT_100af0b4", PTR_DAT_100af0b4),
                    );
                }
            }
            local_2c[0] = mech_trace::rd!(
                "10009232",
                (local_c.wrapping_add(0xf0) as usize as *const i32),
                read_unaligned
            );
            local_2c[1] = mech_trace::rd!(
                "10009232",
                (local_c.wrapping_add(0xf4) as usize as *const i32),
                read_unaligned
            );
            local_2c[2] = mech_trace::rd!(
                "10009232",
                (local_c.wrapping_add(0xf8) as usize as *const i32),
                read_unaligned
            );
            local_10 = mech_trace::cr!("10009232", "100357e3", FUN_100357e3(local_2c.as_mut_ptr()));
            if local_10 != 0 && mech_trace::rd!("10009232", param_1.add(0x20), read_unaligned) != 0
            {
                let f = core::mem::transmute::<i32, unsafe extern "C" fn(*mut i32, i32, i32) -> i32>(
                    mech_trace::rd!("10009232", param_1.add(0x20), read_unaligned),
                );
                local_10 =
                    mech_trace::cr!("10009232", "ind", f(local_20, local_2c[0], local_2c[1]));
            }
            local_38 = mech_trace::rd!(
                "10009232",
                (local_c.wrapping_add(0x104) as usize as *const u32),
                read_unaligned
            );
            local_30 = local_30.wrapping_sub(1);
        }
    }
}
