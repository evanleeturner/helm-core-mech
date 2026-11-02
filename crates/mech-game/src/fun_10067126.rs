// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067126(
    param_1: *mut i32,
    param_2: u32,
    param_3: i32,
    param_4: *mut i32,
) -> *mut i32 {
    mech_trace::enter!("10067126", param_1, param_2, param_3, param_4);
    // SAFETY: the callees are the game's own; param_1 and param_4 point at
    // the caller's two-int arrays, read and written where the original does.
    unsafe {
        let p0 = mech_trace::rd!("10067126", param_1, read_unaligned);
        let p1 = mech_trace::rd!("10067126", param_1.add(1), read_unaligned);
        let mut local_1c: i32 = p0.wrapping_mul(2);
        let mut local_18: i32 = p1.wrapping_mul(2);
        let local_10: i32 = if p0 == 0 {
            0
        } else {
            mech_trace::cr!(
                "10067126",
                "10002570",
                FUN_10002570(
                    mech_trace::rd!("10067126", param_1.add(1), read_unaligned) as u32,
                    mech_trace::rd!("10067126", param_1, read_unaligned),
                )
            ) as i32
        };
        match param_2 {
            0 => {
                if local_10 < param_3 {
                    local_1c = mech_trace::cr!(
                        "10067126",
                        "10002570",
                        FUN_10002570(
                            mech_trace::rd!("10067126", param_1.add(1), read_unaligned) as u32,
                            param_3
                        )
                    ) as i32;
                    local_1c =
                        mech_trace::rd!("10067126", param_1, read_unaligned).wrapping_add(local_1c);
                } else {
                    local_18 = mech_trace::cr!(
                        "10067126",
                        "10003d60",
                        FUN_10003d60(
                            mech_trace::rd!("10067126", param_1, read_unaligned),
                            param_3
                        )
                    );
                    local_18 = mech_trace::rd!("10067126", param_1.add(1), read_unaligned)
                        .wrapping_add(local_18);
                }
            }
            1 => {
                if param_3 < local_10.wrapping_neg() {
                    local_1c = mech_trace::cr!(
                        "10067126",
                        "10002570",
                        FUN_10002570(
                            mech_trace::rd!("10067126", param_1.add(1), read_unaligned) as u32,
                            param_3
                        )
                    ) as i32;
                    local_1c =
                        mech_trace::rd!("10067126", param_1, read_unaligned).wrapping_add(local_1c);
                } else {
                    local_1c = 0;
                    local_18 = mech_trace::rd!("10067126", param_1.add(1), read_unaligned);
                    let i_var1 = mech_trace::cr!(
                        "10067126",
                        "10003d60",
                        FUN_10003d60(
                            mech_trace::rd!("10067126", param_1, read_unaligned),
                            param_3
                        )
                    );
                    local_18 = local_18.wrapping_sub(i_var1);
                }
            }
            2 => {
                if param_3 < local_10.wrapping_neg() {
                    local_18 = 0;
                    local_1c = mech_trace::rd!("10067126", param_1, read_unaligned);
                    let i_var1 = mech_trace::cr!(
                        "10067126",
                        "10002570",
                        FUN_10002570(
                            mech_trace::rd!("10067126", param_1.add(1), read_unaligned) as u32,
                            param_3
                        )
                    ) as i32;
                    local_1c = local_1c.wrapping_sub(i_var1);
                } else {
                    local_18 = mech_trace::cr!(
                        "10067126",
                        "10003d60",
                        FUN_10003d60(
                            mech_trace::rd!("10067126", param_1, read_unaligned),
                            param_3
                        )
                    );
                    local_18 = mech_trace::rd!("10067126", param_1.add(1), read_unaligned)
                        .wrapping_add(local_18);
                }
            }
            3 => {
                if local_10 < param_3 {
                    local_18 = 0;
                    local_1c = mech_trace::rd!("10067126", param_1, read_unaligned);
                    let i_var1 = mech_trace::cr!(
                        "10067126",
                        "10002570",
                        FUN_10002570(
                            mech_trace::rd!("10067126", param_1.add(1), read_unaligned) as u32,
                            param_3
                        )
                    ) as i32;
                    local_1c = local_1c.wrapping_sub(i_var1);
                } else {
                    local_1c = 0;
                    local_18 = mech_trace::rd!("10067126", param_1.add(1), read_unaligned);
                    let i_var1 = mech_trace::cr!(
                        "10067126",
                        "10003d60",
                        FUN_10003d60(
                            mech_trace::rd!("10067126", param_1, read_unaligned),
                            param_3
                        )
                    );
                    local_18 = local_18.wrapping_sub(i_var1);
                }
            }
            5 => {
                local_1c = mech_trace::rd!("10067126", param_1, read_unaligned);
            }
            7 => {
                local_18 = 0;
                local_1c = mech_trace::rd!("10067126", param_1, read_unaligned);
            }
            _ => {
                local_18 = 0;
                local_1c = 0;
            }
        }
        mech_trace::wr!("10067126", param_4, write_unaligned, local_1c);
        mech_trace::wr!("10067126", param_4.add(1), write_unaligned, local_18);
        mech_trace::rt!("10067126", param_4)
    }
}
