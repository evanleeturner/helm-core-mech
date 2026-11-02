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
pub unsafe extern "C" fn FUN_10016b61(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) {
    mech_trace::enter!(
        "10016b61", param_1, param_2, param_3, param_4, param_5, param_6
    );
    let i_var1 = param_4 / 2;
    let i_var2 = param_3.wrapping_sub(param_5);
    if 0 < param_5 {
        let half = i_var1 / 2;
        let mut l: i32 = 0;
        while l < half {
            // SAFETY: calls the game's own function with the original's arguments.
            unsafe {
                mech_trace::cr!(
                    "10016b61",
                    "10056489",
                    FUN_10056489(
                        param_1,
                        l.wrapping_add(param_2),
                        param_3,
                        l.wrapping_add(param_2),
                        i_var2,
                        0,
                        param_6.wrapping_sub(1),
                    )
                );
            }
            l = l.wrapping_add(1);
        }
        l = half;
        while l < i_var1 {
            // SAFETY: calls the game's own function with the original's arguments.
            unsafe {
                mech_trace::cr!(
                    "10016b61",
                    "10056489",
                    FUN_10056489(
                        param_1,
                        l.wrapping_add(param_2),
                        param_3,
                        l.wrapping_add(param_2),
                        i_var2,
                        0,
                        param_6,
                    )
                );
            }
            l = l.wrapping_add(1);
        }
        let i_var3 = i_var1.wrapping_add(param_4.wrapping_sub(i_var1) / 2);
        l = i_var1;
        while l < i_var3 {
            // SAFETY: calls the game's own function with the original's arguments.
            unsafe {
                mech_trace::cr!(
                    "10016b61",
                    "10056489",
                    FUN_10056489(
                        param_1,
                        l.wrapping_add(param_2),
                        param_3,
                        l.wrapping_add(param_2),
                        i_var2,
                        0,
                        param_6.wrapping_sub(1),
                    )
                );
            }
            l = l.wrapping_add(1);
        }
        l = i_var3;
        while l < param_4 {
            // SAFETY: calls the game's own function with the original's arguments.
            unsafe {
                mech_trace::cr!(
                    "10016b61",
                    "10056489",
                    FUN_10056489(
                        param_1,
                        l.wrapping_add(param_2),
                        param_3,
                        l.wrapping_add(param_2),
                        i_var2,
                        0,
                        param_6.wrapping_sub(2),
                    )
                );
            }
            l = l.wrapping_add(1);
        }
    }
}
