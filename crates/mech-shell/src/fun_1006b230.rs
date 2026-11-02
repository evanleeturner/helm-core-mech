// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006b300(param_1: i32, param_2: i32, param_3: i32, param_4: i8) -> u8;
    fn FUN_1006b380(param_1: i32, param_2: i32, param_3: i32, param_4: i8) -> u8;
    fn FUN_1006b500(param_1: i32, param_2: i32, param_3: i8, param_4: i32, param_5: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006b230(param_1: i32, param_2: i32, param_3: i8) -> i32 {
    mech_trace::enter!("1006b230", param_1, param_2, param_3);
    let mut local_4: i32 = 0;
    if param_2 == 0 {
        return mech_trace::rt!("1006b230", 0);
    }
    let mut i_var3: i32 = 0;
    let mut pi_var5 = param_2.wrapping_add(0x368) as *const i32;
    let mut pi_var6 = param_1.wrapping_add(0x368) as *const i32;
    // SAFETY: param_1 points at the game's own object; the byte is written as the original does.
    unsafe {
        mech_trace::wr!(
            "1006b230",
            (param_1.wrapping_add(0x84d) as *mut u8),
            write,
            1
        );
    }
    loop {
        // SAFETY: the callees are the game's own; the pointers walk the game's object arrays as the original does.
        unsafe {
            let i_var1 = mech_trace::cr!(
                "1006b230",
                "1006b300",
                FUN_1006b300(param_1, param_2, i_var3, param_3)
            ) as i32;
            let mut i_var4: i32 = 0;
            let i_var2 = mech_trace::cr!(
                "1006b230",
                "1006b380",
                FUN_1006b380(param_1, param_2, i_var3, param_3)
            ) as i32;
            local_4 = local_4.wrapping_add(i_var1).wrapping_add(i_var2);
            loop {
                if mech_trace::rd!("1006b230", pi_var5, read_unaligned)
                    != mech_trace::rd!("1006b230", pi_var6, read_unaligned)
                {
                    let r = mech_trace::cr!(
                        "1006b230",
                        "1006b500",
                        FUN_1006b500(i_var3, i_var4, param_3, param_1, param_2)
                    ) as i32;
                    if 0 < r {
                        local_4 = local_4.wrapping_add(r);
                    }
                }
                pi_var6 = pi_var6.wrapping_add(1);
                pi_var5 = pi_var5.wrapping_add(1);
                i_var4 = i_var4.wrapping_add(1);
                if i_var4 >= 0xc {
                    break;
                }
            }
        }
        i_var3 = i_var3.wrapping_add(1);
        if i_var3 >= 8 {
            break;
        }
    }
    mech_trace::rt!("1006b230", local_4)
}
