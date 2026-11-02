// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10023898(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10044933(param_1: i32) -> u32 {
    mech_trace::enter!("10044933", param_1);
    let mut b_var1 = false;
    // SAFETY: param_1 is the game's object address; fields are read at the
    // original offsets, and the callee is the game's own function.
    unsafe {
        if param_1 != 0 {
            let mut local_8: u16 = 0;
            while (local_8 as i32)
                < mech_trace::rd!(
                    "10044933",
                    ((param_1.wrapping_add(0x3dc)) as *const i32),
                    read_unaligned
                )
            {
                let mut i_var2 = (local_8 as i32)
                    .wrapping_mul(0x40)
                    .wrapping_add(mech_trace::rd!(
                        "10044933",
                        ((param_1.wrapping_add(0x54)) as *const i32),
                        read_unaligned
                    ));
                if mech_trace::rd!(
                    "10044933",
                    ((i_var2.wrapping_add(8)) as *const i32),
                    read_unaligned
                ) != -1
                {
                    i_var2 = mech_trace::cr!("10044933", "10023898", FUN_10023898(param_1, i_var2))
                        as i32;
                    if i_var2 != 0 {
                        b_var1 = true;
                        break;
                    }
                }
                local_8 = local_8.wrapping_add(1);
            }
        }
        mech_trace::rt!(
            "10044933",
            if b_var1
                || mech_trace::rd!(
                    "10044933",
                    ((param_1.wrapping_add(0x3dc)) as *const i32),
                    read_unaligned
                ) == 0
            {
                0
            } else {
                1
            }
        )
    }
}
