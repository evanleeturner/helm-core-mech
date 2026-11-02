// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aca54: i32;
    static mut DAT_100aca58: i32;
    static mut DAT_100aca5c: u32;
    static mut DAT_100ad578: u8;
    static mut DAT_100ad57c: u8;
    static mut DAT_10123120: u32;
    static mut DAT_10123240: u8;
    static mut DAT_10123244: u8;
    static mut DAT_10123248: u8;
    static mut DAT_1012324c: u8;
    static mut DAT_10123278: u8;
    static mut DAT_1012327a: u8;
    static mut DAT_1012327b: u8;
    static mut DAT_1012327c: u8;
    static mut s_Primary_100ad53c: u8;
    static mut s_Return_100ad560: u8;
    static mut s_Secondary_100ad548: u8;
    static mut s_Star__d_mission__Failed_at__d_se_100ad4f0: u8;
    static mut s_Star__d_mission__Out_of_time_at___100ad514: u8;
    static mut s_Star__d_mission__Successful_at___100ad4c8: u8;
    static mut s_Star__d_mission___d_sec_left_100ad488: u8;
    static mut s_Star__d_mission__in_progress_100ad4a8: u8;
    static mut s_Tertiary_100ad554: u8;
    static mut s_Tertiary_100ad56c: u8;
    static mut s__2_2s__2_2d__s___3_3s_100ad580: u8;
    static mut s__s__s__3_3d_sec__s_100ad5f0: u8;
    static mut s__s__s__s_100ad608: u8;
    static mut s__s_failed_at__3_3d_sec__s_100ad5b8: u8;
    static mut s__s_successful_at__3_3d_sec__s_100ad598: u8;
    static mut s_in_progress_100ad5d8: u8;
    static mut s_inactive_100ad5e4: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_100274a5(param_1: i16, param_2: i32, param_3: i16) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10027502();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100280a3(param_1: i32) {
    mech_trace::enter!("100280a3", param_1);
    let mut local_220 = [0i8; 256];
    let mut local_110 = [0i8; 256];
    // SAFETY: all globals, strings and callees are the game's own memory and
    // code; offsets mirror the original's indexing into its mission tables.
    unsafe {
        let i_var2 = param_1.wrapping_mul(0x3c0a);
        let pi_var1 = (&raw mut DAT_10123240)
            .byte_offset(i_var2 as isize)
            .cast::<i32>();
        match DAT_10123278_at(i_var2) {
            0 => {
                let v = mech_trace::rd!(
                    "100280a3",
                    (&raw mut DAT_1012324c)
                        .byte_offset(i_var2 as isize)
                        .cast::<i32>(),
                    read_unaligned
                );
                if v < 1 {
                    mech_trace::cr!(
                        "100280a3",
                        "_sprintf",
                        _sprintf(
                            local_110.as_mut_ptr(),
                            (&raw const s_Star__d_mission__in_progress_100ad4a8).cast::<i8>(),
                            param_1,
                        )
                    );
                } else {
                    let start = mech_trace::rd!(
                        "100280a3",
                        (&raw mut DAT_10123244)
                            .byte_offset(i_var2 as isize)
                            .cast::<i32>(),
                        read_unaligned
                    );
                    let local_114 = v.wrapping_sub(
                        mech_trace::gr!("100280a3", "10123120", DAT_10123120)
                            .wrapping_sub(start as u32) as i32,
                    );
                    mech_trace::cr!(
                        "100280a3",
                        "_sprintf",
                        _sprintf(
                            local_110.as_mut_ptr(),
                            (&raw const s_Star__d_mission___d_sec_left_100ad488).cast::<i8>(),
                            param_1,
                            local_114,
                        )
                    );
                }
            }
            2 => {
                let v = mech_trace::rd!(
                    "100280a3",
                    (&raw mut DAT_10123248)
                        .byte_offset(i_var2 as isize)
                        .cast::<u32>(),
                    read_unaligned
                );
                mech_trace::cr!(
                    "100280a3",
                    "_sprintf",
                    _sprintf(
                        local_110.as_mut_ptr(),
                        (&raw const s_Star__d_mission__Successful_at___100ad4c8).cast::<i8>(),
                        param_1,
                        v,
                    )
                );
            }
            3 => {
                let v = mech_trace::rd!(
                    "100280a3",
                    (&raw mut DAT_10123248)
                        .byte_offset(i_var2 as isize)
                        .cast::<u32>(),
                    read_unaligned
                );
                mech_trace::cr!(
                    "100280a3",
                    "_sprintf",
                    _sprintf(
                        local_110.as_mut_ptr(),
                        (&raw const s_Star__d_mission__Failed_at__d_se_100ad4f0).cast::<i8>(),
                        param_1,
                        v,
                    )
                );
            }
            4 => {
                let v = mech_trace::rd!(
                    "100280a3",
                    (&raw mut DAT_10123248)
                        .byte_offset(i_var2 as isize)
                        .cast::<u32>(),
                    read_unaligned
                );
                mech_trace::cr!(
                    "100280a3",
                    "_sprintf",
                    _sprintf(
                        local_110.as_mut_ptr(),
                        (&raw const s_Star__d_mission__Out_of_time_at___100ad514).cast::<i8>(),
                        param_1,
                        v,
                    )
                );
            }
            _ => {}
        }

        if mech_trace::gr!("100280a3", "100aca5c", DAT_100aca5c) == 0xFFFF_FFFF {
            mech_trace::gw!("100280a3", "100aca5c", DAT_100aca5c, 0);
            mech_trace::gw!(
                "100280a3",
                "100aca58",
                DAT_100aca58,
                mech_trace::rd!("100280a3", pi_var1, read_unaligned).wrapping_sub(1)
            );
            if 0x13 < mech_trace::gr!("100280a3", "100aca58", DAT_100aca58) {
                mech_trace::gw!("100280a3", "100aca58", DAT_100aca58, 0x14);
            }
        }
        if mech_trace::gr!("100280a3", "100aca54", DAT_100aca54)
            < mech_trace::gr!("100280a3", "100aca5c", DAT_100aca5c) as i32
        {
            if mech_trace::gr!("100280a3", "100aca54", DAT_100aca54) < 1 {
                mech_trace::gw!(
                    "100280a3",
                    "100aca54",
                    DAT_100aca54,
                    mech_trace::gr!("100280a3", "100aca5c", DAT_100aca5c) as i32
                );
            } else {
                mech_trace::gw!(
                    "100280a3",
                    "100aca5c",
                    DAT_100aca5c,
                    mech_trace::gr!("100280a3", "100aca54", DAT_100aca54) as u32
                );
                mech_trace::gw!(
                    "100280a3",
                    "100aca58",
                    DAT_100aca58,
                    mech_trace::rd!("100280a3", pi_var1, read_unaligned).wrapping_sub(1)
                );
                if mech_trace::gr!("100280a3", "100aca54", DAT_100aca54).wrapping_add(0x14)
                    <= mech_trace::rd!("100280a3", pi_var1, read_unaligned).wrapping_sub(1)
                {
                    mech_trace::gw!(
                        "100280a3",
                        "100aca58",
                        DAT_100aca58,
                        mech_trace::gr!("100280a3", "100aca54", DAT_100aca54).wrapping_add(0x14)
                    );
                }
                FUN_10027502();
            }
        } else if mech_trace::gr!("100280a3", "100aca58", DAT_100aca58)
            < mech_trace::gr!("100280a3", "100aca54", DAT_100aca54)
        {
            if mech_trace::gr!("100280a3", "100aca54", DAT_100aca54)
                < mech_trace::rd!("100280a3", pi_var1, read_unaligned)
            {
                mech_trace::gw!(
                    "100280a3",
                    "100aca58",
                    DAT_100aca58,
                    mech_trace::gr!("100280a3", "100aca54", DAT_100aca54)
                );
                mech_trace::gw!(
                    "100280a3",
                    "100aca5c",
                    DAT_100aca5c,
                    mech_trace::gr!("100280a3", "100aca54", DAT_100aca54).wrapping_sub(0x14) as u32
                );
                if (mech_trace::gr!("100280a3", "100aca5c", DAT_100aca5c) as i32) < 1 {
                    mech_trace::gw!("100280a3", "100aca5c", DAT_100aca5c, 0);
                }
                FUN_10027502();
            } else {
                mech_trace::gw!(
                    "100280a3",
                    "100aca54",
                    DAT_100aca54,
                    mech_trace::gr!("100280a3", "100aca58", DAT_100aca58)
                );
            }
        }

        let mut local_11c = mech_trace::gr!("100280a3", "100aca5c", DAT_100aca5c) as i32;
        while local_11c <= mech_trace::gr!("100280a3", "100aca58", DAT_100aca58) {
            let i_var2 = local_11c
                .wrapping_mul(0x13f)
                .wrapping_add(param_1.wrapping_mul(0x3c0a));
            let local_c = (&raw mut DAT_1012327a).byte_offset(i_var2 as isize);
            let sel = mech_trace::rd!(
                "100280a3",
                (&raw mut DAT_1012327b).byte_offset(i_var2 as isize),
                read
            );
            let local_8: *const i8 = match sel {
                1 => (&raw const s_Primary_100ad53c).cast::<i8>(),
                2 => (&raw const s_Secondary_100ad548).cast::<i8>(),
                4 => (&raw const s_Tertiary_100ad554).cast::<i8>(),
                8 => (&raw const s_Return_100ad560).cast::<i8>(),
                _ => (&raw const s_Tertiary_100ad56c).cast::<i8>(),
            };
            let arg = mech_trace::rd!(
                "100280a3",
                (&raw mut DAT_1012327c)
                    .byte_offset(i_var2 as isize)
                    .cast::<u32>(),
                read_unaligned
            );
            let local_118 = mech_trace::cr!(
                "100280a3",
                "100274a5",
                FUN_100274a5(arg as i16, 0x100accc0, 0xe)
            );
            let local_10: *const u8 =
                if mech_trace::gr!("100280a3", "100aca54", DAT_100aca54) == local_11c {
                    &raw const DAT_100ad578
                } else {
                    &raw const DAT_100ad57c
                };
            mech_trace::cr!(
                "100280a3",
                "_sprintf",
                _sprintf(
                    local_220.as_mut_ptr(),
                    (&raw const s__2_2s__2_2d__s___3_3s_100ad580).cast::<i8>(),
                    local_10,
                    local_11c,
                    local_8,
                    local_118,
                )
            );
            let kind = mech_trace::rd!("100280a3", local_c, read);
            if kind == 5 {
                mech_trace::cr!(
                    "100280a3",
                    "_sprintf",
                    _sprintf(
                        local_110.as_mut_ptr(),
                        (&raw const s__s_successful_at__3_3d_sec__s_100ad598).cast::<i8>(),
                        local_220.as_ptr(),
                        mech_trace::rd!(
                            "100280a3",
                            local_c.byte_add(0x6b).cast::<u32>(),
                            read_unaligned
                        ),
                        local_c.byte_add(0xad),
                    )
                );
            } else if kind == 6 {
                mech_trace::cr!(
                    "100280a3",
                    "_sprintf",
                    _sprintf(
                        local_110.as_mut_ptr(),
                        (&raw const s__s_failed_at__3_3d_sec__s_100ad5b8).cast::<i8>(),
                        local_220.as_ptr(),
                        mech_trace::rd!(
                            "100280a3",
                            local_c.byte_add(0x6b).cast::<u32>(),
                            read_unaligned
                        ),
                        local_c.byte_add(0xad),
                    )
                );
            } else {
                let local_228: *const i8 =
                    if mech_trace::rd!("100280a3", local_c.byte_add(0x73), read) == 0 {
                        (&raw const s_inactive_100ad5e4).cast::<i8>()
                    } else {
                        (&raw const s_in_progress_100ad5d8).cast::<i8>()
                    };
                let v = mech_trace::rd!(
                    "100280a3",
                    local_c.byte_add(0x6f).cast::<i32>(),
                    read_unaligned
                );
                if v < 1 || mech_trace::rd!("100280a3", local_c.byte_add(0x73), read) == 0 {
                    mech_trace::cr!(
                        "100280a3",
                        "_sprintf",
                        _sprintf(
                            local_110.as_mut_ptr(),
                            (&raw const s__s__s__s_100ad608).cast::<i8>(),
                            local_220.as_ptr(),
                            local_228,
                            local_c.byte_add(0xad),
                        )
                    );
                } else {
                    let start = mech_trace::rd!(
                        "100280a3",
                        local_c.byte_add(0x67).cast::<i32>(),
                        read_unaligned
                    );
                    let local_114 = v.wrapping_sub(
                        mech_trace::gr!("100280a3", "10123120", DAT_10123120)
                            .wrapping_sub(start as u32) as i32,
                    );
                    mech_trace::cr!(
                        "100280a3",
                        "_sprintf",
                        _sprintf(
                            local_110.as_mut_ptr(),
                            (&raw const s__s__s__3_3d_sec__s_100ad5f0).cast::<i8>(),
                            local_220.as_ptr(),
                            local_228,
                            local_114,
                            local_c.byte_add(0xad),
                        )
                    );
                }
            }
            local_11c = local_11c.wrapping_add(1);
        }
    }
}

#[allow(non_snake_case)]
unsafe fn DAT_10123278_at(i: i32) -> u8 {
    // SAFETY: indexes the game's own table at the original's offset.
    unsafe {
        mech_trace::rd!(
            "100280a3",
            (&raw mut DAT_10123278).byte_offset(i as isize),
            read
        )
    }
}
