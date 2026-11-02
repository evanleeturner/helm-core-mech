// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a67c0: i32;
    static mut DAT_100a67bc: i32;
    static mut DAT_100a67c4: i32;
    static mut DAT_100a67c8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017f30(param_1: i32, param_2: i32) -> u32;
    fn FUN_10017ec0(param_1: i32, param_2: i32, param_3: i8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000c6b0(param_1: i32, param_2: i32) -> u32 {
    mech_trace::enter!("1000c6b0", param_1, param_2);
    let mut param_2 = param_2;
    if param_2 == 0 {
        // SAFETY: the globals and callees are the game's own; values are
        // passed through unchanged in the original's order.
        unsafe {
            let u_var1 = mech_trace::cr!(
                "1000c6b0",
                "10017f30",
                FUN_10017f30(
                    mech_trace::gr!("1000c6b0", "100a67c0", DAT_100a67c0),
                    param_1
                )
            );
            if (u_var1 as i32) >= 0 {
                let u_var2 = mech_trace::cr!(
                    "1000c6b0",
                    "10017ec0",
                    FUN_10017ec0(
                        mech_trace::gr!("1000c6b0", "100a67c0", DAT_100a67c0),
                        param_1,
                        0
                    )
                );
                return mech_trace::rt!("1000c6b0", (u_var2 >> 8) | 1);
            }
            let u_var1 = mech_trace::cr!(
                "1000c6b0",
                "10017f30",
                FUN_10017f30(
                    mech_trace::gr!("1000c6b0", "100a67bc", DAT_100a67bc),
                    param_1
                )
            );
            if (u_var1 as i32) >= 0 {
                let u_var2 = mech_trace::cr!(
                    "1000c6b0",
                    "10017ec0",
                    FUN_10017ec0(
                        mech_trace::gr!("1000c6b0", "100a67bc", DAT_100a67bc),
                        param_1,
                        0
                    )
                );
                return mech_trace::rt!("1000c6b0", (u_var2 >> 8) | 1);
            }
            let u_var1 = mech_trace::cr!(
                "1000c6b0",
                "10017f30",
                FUN_10017f30(
                    mech_trace::gr!("1000c6b0", "100a67c4", DAT_100a67c4),
                    param_1
                )
            );
            if (u_var1 as i32) >= 0 {
                let u_var2 = mech_trace::cr!(
                    "1000c6b0",
                    "10017ec0",
                    FUN_10017ec0(
                        mech_trace::gr!("1000c6b0", "100a67c4", DAT_100a67c4),
                        param_1,
                        0
                    )
                );
                return mech_trace::rt!("1000c6b0", (u_var2 >> 8) | 1);
            }
            param_2 = mech_trace::cr!(
                "1000c6b0",
                "10017f30",
                FUN_10017f30(
                    mech_trace::gr!("1000c6b0", "100a67c8", DAT_100a67c8),
                    param_1
                )
            ) as i32;
            if param_2 >= 0 {
                let u_var2 = mech_trace::cr!(
                    "1000c6b0",
                    "10017ec0",
                    FUN_10017ec0(
                        mech_trace::gr!("1000c6b0", "100a67c8", DAT_100a67c8),
                        param_1,
                        0
                    )
                );
                return mech_trace::rt!("1000c6b0", (u_var2 >> 8) | 1);
            }
        }
    }
    mech_trace::rt!("1000c6b0", (param_2 as u32) & 0xFFFFFF00)
}
