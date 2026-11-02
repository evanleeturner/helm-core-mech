// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002954c(param_1: u16) -> i32;
    fn FUN_10029e46(param_1: i32, param_2: u16, param_3: u16) -> u32;
    fn FUN_1002a1e0(param_1: u32, param_2: i32) -> i32;
    fn FUN_1002950f(param_1: i32, param_2: i16) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10028932(param_1: i32, param_2: u16, param_3: i16) -> i16 {
    mech_trace::enter!("10028932", param_1, param_2, param_3);
    let mut local_10: i16 = 0;
    let mut local_18: i16 = -1;
    let mut local_8: u32 = 0xFFFF;
    let local_14: i32;

    // SAFETY: param_1 is the caller's object address; the offsets are read
    // where the original reads them, and the callees are the game's own.
    unsafe {
        if param_3 == -3 {
            local_14 = 0;
        } else if param_3 == -4 {
            local_14 = mech_trace::rd!(
                "10028932",
                (param_1.wrapping_add(0x19e) as *const i32),
                read_unaligned
            );
        } else if param_3 == -2 {
            local_14 = mech_trace::rd!(
                "10028932",
                (param_1.wrapping_add(0x19a) as *const i32),
                read_unaligned
            );
        } else if param_3 == 0 {
            local_14 = mech_trace::cr!("10028932", "1002954c", FUN_1002954c(param_2));
        } else {
            local_14 = i32::from(param_3).wrapping_mul(100);
        }

        let mut local_c = local_14;

        loop {
            let u_var3 = mech_trace::cr!(
                "10028932",
                "10029e46",
                FUN_10029e46(param_1, param_2, local_8 as u16)
            );
            let s_var2 = u_var3 as i16;
            local_8 = (local_8 & 0xFFFF0000) | u32::from(s_var2 as u16);
            if s_var2 == -1 {
                break;
            }
            let i_var4 = mech_trace::cr!("10028932", "1002a1e0", FUN_1002a1e0(local_8, 2));
            if i_var4 == 0
                && mech_trace::cr!("10028932", "1002950f", FUN_1002950f(param_1, s_var2)) != 0
            {
                let v = mech_trace::rd!(
                    "10028932",
                    (param_1.wrapping_add(0xc0) as *const i32),
                    read_unaligned
                );
                if local_c < v {
                    local_c = mech_trace::rd!(
                        "10028932",
                        (param_1.wrapping_add(0xc0) as *const i32),
                        read_unaligned
                    );
                    local_18 = s_var2;
                }
            }
        }

        if local_14 < local_c && local_18 != -1 {
            local_10 = local_18;
        }
    }

    mech_trace::rt!("10028932", local_10)
}
