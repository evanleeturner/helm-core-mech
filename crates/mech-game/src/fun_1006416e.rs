// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10006d3e() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006bbb(param_1: u32) -> i32;
    fn FUN_1002f668(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: u32);
    fn FUN_100640ee(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32);
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_1000fef4(param_1: *mut u32, param_2: *mut u32, param_3: *mut u32);
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32);
    fn FUN_1002f3a7(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006416e(
    param_1: i32,
    param_2: i32,
    param_3: u32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    mech_trace::enter!(
        "1006416e", param_1, param_2, param_3, param_4, param_5, param_6, param_7
    );
    let mut local_8: i32 = 0;
    let mut local_c: i32 = 0;
    let mut local_10: i32 = 0;
    // SAFETY: all callees are the game's own; locals are passed by pointer
    // as the original does, and param_2/param_7 point at game structures.
    unsafe {
        let local_18 = mech_trace::cr!("1006416e", "10006d3e", FUN_10006d3e());
        let local_14 = mech_trace::cr!(
            "1006416e",
            "10006bbb",
            FUN_10006bbb((param_4 as u32).wrapping_shl(13))
        );
        FUN_1002f668(param_2, local_14.wrapping_neg(), local_18, 0, 0);
        FUN_100640ee(
            param_1,
            &raw mut local_8 as *mut u32,
            &raw mut local_c as *mut u32,
            &raw mut local_10 as *mut u32,
        );
        if param_6 != 0 {
            let mut local_24 = mech_trace::cr!("1006416e", "10051f43", FUN_10051f43(200));
            local_24 = local_24.wrapping_sub(100);
            let mut local_34: u32 = (param_5 as u32).wrapping_neg();
            let mut local_38: u32 = 0;
            let mut local_20: u32 = param_3;
            FUN_1000fef4(&raw mut local_34, &raw mut local_38, &raw mut local_20);
            let mut i_var1 = mech_trace::cr!(
                "1006416e",
                "10003d60",
                FUN_10003d60(local_24, local_34 as i32)
            );
            local_8 = local_8.wrapping_add(i_var1);
            i_var1 = mech_trace::cr!(
                "1006416e",
                "10003d60",
                FUN_10003d60(local_24, local_20 as i32)
            );
            local_10 = local_10.wrapping_add(i_var1);
            let local_28 = mech_trace::cr!(
                "1006416e",
                "10003d60",
                FUN_10003d60(param_4, local_20 as i32)
            );
            i_var1 = mech_trace::cr!(
                "1006416e",
                "10003d60",
                FUN_10003d60(local_20 as i32, param_3 as i32)
            );
            let mut local_2c = mech_trace::cr!(
                "1006416e",
                "10003d60",
                FUN_10003d60(param_5, local_34 as i32)
            );
            local_2c = i_var1.wrapping_add(local_2c);
            let mut local_30 = mech_trace::cr!(
                "1006416e",
                "10003d60",
                FUN_10003d60(local_34 as i32, param_4)
            );
            local_30 = local_30.wrapping_neg();
            i_var1 = mech_trace::cr!("1006416e", "10003d60", FUN_10003d60(local_24, local_28));
            local_8 = local_8.wrapping_add(i_var1);
            i_var1 = mech_trace::cr!("1006416e", "10003d60", FUN_10003d60(local_24, local_2c));
            local_c = local_c.wrapping_add(i_var1);
            i_var1 = mech_trace::cr!("1006416e", "10003d60", FUN_10003d60(local_24, local_30));
            local_10 = local_10.wrapping_add(i_var1);
        }
        FUN_1002f37a(param_2, local_8 as u32, local_c as u32, local_10 as u32);
        if (mech_trace::rd!(
            "1006416e",
            (param_7 as usize as *const u8).byte_add(8),
            read
        ) & 0x40)
            == 0
        {
            let inner = mech_trace::rd!(
                "1006416e",
                ((param_2 as usize + 0x6c) as *const i32),
                read_unaligned
            );
            let local_1c = mech_trace::rd!(
                "1006416e",
                ((inner as usize + 0x40) as *const i32),
                read_unaligned
            )
            .wrapping_mul(2);
            let i_var1 = mech_trace::cr!(
                "1006416e",
                "10003d60",
                FUN_10003d60(param_3 as i32, local_1c)
            );
            let i_var2 = mech_trace::cr!("1006416e", "10003d60", FUN_10003d60(param_4, local_1c));
            let i_var3 = mech_trace::cr!("1006416e", "10003d60", FUN_10003d60(param_5, local_1c));
            FUN_1002f3a7(param_2, i_var1, i_var2, i_var3);
        }
    }
}
