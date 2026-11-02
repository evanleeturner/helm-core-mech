// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10056489(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10016cdd(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    mech_trace::enter!(
        "10016cdd", param_1, param_2, param_3, param_4, param_5, param_6
    );
    let i_var1 = param_5 / 2;
    let i_var2 = param_4.wrapping_add(param_2);
    if 0 < param_4 {
        let half = i_var1 / 2;
        let mut local: i32 = 0;
        while local < half {
            // SAFETY: the callee is the game's own function.
            unsafe {
                mech_trace::cr!(
                    "10016cdd",
                    "10056489",
                    FUN_10056489(
                        param_1,
                        param_2,
                        param_3.wrapping_add(local),
                        i_var2,
                        param_3.wrapping_add(local),
                        0,
                        param_6.wrapping_sub(1),
                    )
                );
            }
            local = local.wrapping_add(1);
        }
        local = half;
        while local < i_var1 {
            // SAFETY: the callee is the game's own function.
            unsafe {
                mech_trace::cr!(
                    "10016cdd",
                    "10056489",
                    FUN_10056489(
                        param_1,
                        param_2,
                        param_3.wrapping_add(local),
                        i_var2,
                        param_3.wrapping_add(local),
                        0,
                        param_6,
                    )
                );
            }
            local = local.wrapping_add(1);
        }
        let i_var3 = i_var1.wrapping_add(param_5.wrapping_sub(i_var1) / 2);
        local = i_var1;
        while local < i_var3 {
            // SAFETY: the callee is the game's own function.
            unsafe {
                mech_trace::cr!(
                    "10016cdd",
                    "10056489",
                    FUN_10056489(
                        param_1,
                        param_2,
                        param_3.wrapping_add(local),
                        i_var2,
                        param_3.wrapping_add(local),
                        0,
                        param_6.wrapping_sub(1),
                    )
                );
            }
            local = local.wrapping_add(1);
        }
        local = i_var3;
        while local < param_5 {
            // SAFETY: the callee is the game's own function.
            unsafe {
                mech_trace::cr!(
                    "10016cdd",
                    "10056489",
                    FUN_10056489(
                        param_1,
                        param_2,
                        param_3.wrapping_add(local),
                        i_var2,
                        param_3.wrapping_add(local),
                        0,
                        param_6.wrapping_sub(2),
                    )
                );
            }
            local = local.wrapping_add(1);
        }
    }
}
