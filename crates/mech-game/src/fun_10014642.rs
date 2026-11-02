// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_10160c00: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003d2d3(param_1: i32, param_2: i32) -> u64;
    fn FUN_1001b863(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> u64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10014642(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> i32 {
    mech_trace::enter!("10014642", param_1, param_2, param_3, param_4);
    let mut local_10: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_8: i32 = 0x7fffffff;

    if param_1 == 0 {
        local_10 = 0;
    } else {
        // SAFETY: the callees and globals are the game's own; the pointers
        // read are the game's linked records and table, read as the original does.
        unsafe {
            let mut l_var3: u64 =
                mech_trace::cr!("10014642", "1003d2d3", FUN_1003d2d3(param_2, param_4));
            let mut u_var1: u32 = l_var3 as u32;
            let mut local_14: i32 = mech_trace::gr!("10014642", "10160c00", _DAT_10160c00);
            while local_14 != 0 {
                local_18 = local_18.wrapping_add(1);
                let mut local_1c: i32 = mech_trace::rd!(
                    "10014642",
                    ((param_1.wrapping_add(0x10)) as *const i32),
                    read_unaligned
                );
                while local_1c != 0 {
                    let u_var2: u32 = (l_var3 >> 32) as u32;
                    l_var3 = mech_trace::cr!(
                        "10014642",
                        "1001b863",
                        FUN_1001b863(
                            u_var1 as i32,
                            u_var2 as i32,
                            local_1c,
                            param_2,
                            param_3,
                            param_4,
                        )
                    );
                    if (l_var3 as u32 as i32) < local_8 {
                        local_10 = local_1c;
                        local_8 = l_var3 as u32 as i32;
                    }
                    u_var1 = l_var3 as u32;
                    local_1c = mech_trace::rd!(
                        "10014642",
                        ((local_1c.wrapping_add(0x10)) as *const i32),
                        read_unaligned
                    );
                }
                local_14 = mech_trace::rd!(
                    "10014642",
                    ((0x10160c00i32.wrapping_add(local_18.wrapping_mul(4))) as *const i32),
                    read_unaligned
                );
            }
        }
    }
    mech_trace::rt!("10014642", local_10)
}
