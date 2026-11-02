// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa55c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10011e60(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_100065c4(param_1: i32, param_2: *mut i32, param_3: i32, param_4: u32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10006d3e() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007fcc3(param_1: *mut i32, param_2: *mut i32, param_3: i32) {
    mech_trace::enter!("1007fcc3", param_1, param_2, param_3);
    // SAFETY: the pointers, globals and callees are the game's own memory;
    // reads and calls happen in the original's order.
    unsafe {
        let local_8: i32;
        if param_2.is_null() {
            local_8 = -2;
        } else {
            local_8 = mech_trace::rd!(
                "1007fcc3",
                ((mech_trace::rd!("1007fcc3", param_2, read_unaligned) as u32).wrapping_add(4)
                    as *const i32),
                read_unaligned
            );
        }
        if (mech_trace::gr!("1007fcc3", "100aa55c", DAT_100aa55c) as i32) < -0xddb4 {
            let mut local_c: i32;
            if param_2.is_null() {
                local_c = param_3;
            } else {
                let v = mech_trace::rd!("1007fcc3", param_1.add(0x106), read_unaligned);
                let local_14 = if v < 1 { 1 } else { v };
                local_c = mech_trace::cr!(
                    "1007fcc3",
                    "10011e60",
                    FUN_10011e60(
                        param_3.wrapping_shl(2),
                        mech_trace::rd!("1007fcc3", param_2.add(0x106), read_unaligned),
                        local_14,
                    )
                ) as i32;
                if local_c < param_3 {
                    local_c = param_3;
                }
            }
            mech_trace::cr!(
                "1007fcc3",
                "100065c4",
                FUN_100065c4(local_8, param_1, local_c, 1)
            );
        } else if mech_trace::gr!("1007fcc3", "100aa55c", DAT_100aa55c) < 0xb506 {
            let iVar1 = mech_trace::cr!("1007fcc3", "10006d3e", FUN_10006d3e());
            let base = mech_trace::rd!("1007fcc3", param_1, read_unaligned);
            let a = mech_trace::rd!(
                "1007fcc3",
                (base.wrapping_add(0x60) as *const i32),
                read_unaligned
            );
            let b = mech_trace::rd!(
                "1007fcc3",
                (base.wrapping_add(0x6c) as *const i32),
                read_unaligned
            );
            let mut local_10 = iVar1.wrapping_sub(a).wrapping_sub(b);
            if local_10 < -0xb40000 {
                local_10 = local_10.wrapping_add(0x1680000);
            } else if 0xb40000 < local_10 {
                local_10 = local_10.wrapping_add(-0x1680000);
            }
            if local_10 < 1 {
                if local_10 < -0x2cffff && -0x870001 < local_10 {
                    mech_trace::cr!(
                        "1007fcc3",
                        "100065c4",
                        FUN_100065c4(local_8, param_1, param_3, 6)
                    );
                } else {
                    mech_trace::cr!(
                        "1007fcc3",
                        "100065c4",
                        FUN_100065c4(local_8, param_1, param_3, 4)
                    );
                }
            } else if local_10 < 0x2d0000 || 0x870000 < local_10 {
                mech_trace::cr!(
                    "1007fcc3",
                    "100065c4",
                    FUN_100065c4(local_8, param_1, param_3, 2)
                );
            } else {
                mech_trace::cr!(
                    "1007fcc3",
                    "100065c4",
                    FUN_100065c4(local_8, param_1, param_3, 5)
                );
            }
        } else {
            mech_trace::cr!(
                "1007fcc3",
                "100065c4",
                FUN_100065c4(local_8, param_1, param_3 >> 1, 7)
            );
            mech_trace::cr!(
                "1007fcc3",
                "100065c4",
                FUN_100065c4(local_8, param_1, param_3 >> 1, 8)
            );
        }
    }
}
