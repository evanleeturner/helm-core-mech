// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10029e46(param_1: i32, param_2: u16, param_3: u16) -> u32;
    fn FUN_1002a1e0(param_1: u32, param_2: i32) -> i32;
    fn FUN_1002950f(param_1: i32, param_2: i16) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002883b(param_1: i32, param_2: u16, param_3: i16) -> i32 {
    mech_trace::enter!("1002883b", param_1, param_2, param_3);
    let mut local_18: i16 = -1;
    let mut local_10: i16 = 0;
    let mut local_8: u32 = 0xffff;

    let local_14: i32 = if param_3 == -3 {
        100000000
    } else {
        (param_3 as i32).wrapping_mul(100)
    };
    let mut local_c: i32 = local_14;

    loop {
        // SAFETY: the callee is the game's own function.
        let u_var3 = unsafe {
            mech_trace::cr!(
                "1002883b",
                "10029e46",
                FUN_10029e46(param_1, param_2, local_8 as u16)
            )
        };
        let s_var2 = u_var3 as i16;
        local_8 = (local_8 & 0xffff0000) | ((s_var2 as i32 & 0xffff) as u32);
        if s_var2 == -1 {
            break;
        }

        // SAFETY: the callee is the game's own function.
        let i_var4 = unsafe { mech_trace::cr!("1002883b", "1002a1e0", FUN_1002a1e0(local_8, 2)) };
        if i_var4 == 0 {
            // SAFETY: the callee is the game's own function.
            let b_var1 =
                unsafe { mech_trace::cr!("1002883b", "1002950f", FUN_1002950f(param_1, s_var2)) };
            if b_var1 != 0 {
                // SAFETY: param_1 is the caller's object pointer; offset 0xc0 holds an i32.
                let v = unsafe {
                    mech_trace::rd!(
                        "1002883b",
                        ((param_1 as usize).wrapping_add(0xc0) as *const i32),
                        read_unaligned
                    )
                };
                if v < local_c {
                    // SAFETY: same object and offset as the read above.
                    local_c = unsafe {
                        mech_trace::rd!(
                            "1002883b",
                            ((param_1 as usize).wrapping_add(0xc0) as *const i32),
                            read_unaligned
                        )
                    };
                    local_18 = s_var2;
                }
            }
        }
    }

    if local_c < local_14 && local_18 != -1 {
        local_10 = local_18;
    }

    mech_trace::rt!(
        "1002883b",
        (((local_c as u32) & 0xffff0000) | (((local_10 as i32) & 0xffff) as u32)) as i32
    )
}
