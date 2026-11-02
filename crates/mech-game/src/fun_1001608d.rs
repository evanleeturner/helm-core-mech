// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006b20(param_1: i32) -> i32;
    fn FUN_10006b9a(param_1: i32) -> i32;
    fn FUN_10015b30(param_1: i32, param_2: i32) -> i32;
    fn FUN_100676f7(param_1: i32, param_2: *mut i32, param_3: *mut i32);
    fn FUN_10066f36(param_1: i32, param_2: *mut i32, param_3: *mut i32);
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
pub unsafe extern "C" fn FUN_1001608d(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32) {
    mech_trace::enter!("1001608d", param_1, param_2, param_3, param_4);
    let mut local_28: [i32; 2] = [0; 2];
    let mut local_1c: [i32; 2] = [0; 2];
    let mut local_14: [i32; 2] = [0; 2];
    // SAFETY: param_1 and param_4 point at the caller's rectangle structs;
    // the callees and their data are the game's own, called in the original order.
    unsafe {
        let p1 = param_1 as *const i32;
        let mut local_8: i32 = (mech_trace::rd!("1001608d", p1.add(4), read_unaligned))
            .wrapping_sub(mech_trace::rd!("1001608d", p1.add(2), read_unaligned))
            .wrapping_add(1)
            .wrapping_mul(0x10000);
        let local_20 = mech_trace::cr!("1001608d", "10006b20", FUN_10006b20(param_2));
        let local_c = mech_trace::cr!("1001608d", "10006b9a", FUN_10006b9a(param_2));
        let mut i_var1 = mech_trace::rd!("1001608d", p1.add(3), read_unaligned);
        let mut i_var2 = mech_trace::rd!("1001608d", p1.add(1), read_unaligned);
        let mut i_var3 = mech_trace::cr!("1001608d", "10015b30", FUN_10015b30(local_8, local_c));
        local_1c[0] =
            (i_var1.wrapping_sub(i_var2).wrapping_add(1) >> 1).wrapping_add(i_var3 >> 0x10);
        i_var1 = mech_trace::rd!("1001608d", p1.add(4), read_unaligned);
        i_var2 = mech_trace::rd!("1001608d", p1.add(2), read_unaligned);
        i_var3 = mech_trace::cr!("1001608d", "10015b30", FUN_10015b30(local_8, local_20));
        local_1c[1] =
            (i_var1.wrapping_sub(i_var2).wrapping_add(1) >> 1).wrapping_add(i_var3 >> 0x10);
        FUN_100676f7(param_1 as i32, local_1c.as_mut_ptr(), local_28.as_mut_ptr());
        if param_4 == 0 {
            mech_trace::cr!(
                "1001608d",
                "10056489",
                FUN_10056489(
                    param_1,
                    local_28[0],
                    local_28[1],
                    mech_trace::rd!("1001608d", p1.add(3), read_unaligned)
                        .wrapping_sub(mech_trace::rd!("1001608d", p1.add(1), read_unaligned))
                        .wrapping_add(1)
                        >> 1,
                    mech_trace::rd!("1001608d", p1.add(4), read_unaligned)
                        .wrapping_sub(mech_trace::rd!("1001608d", p1.add(2), read_unaligned))
                        .wrapping_add(1)
                        >> 1,
                    0,
                    param_3,
                )
            );
        } else {
            let p4 = param_4 as *const i32;
            local_8 = mech_trace::rd!("1001608d", p4.add(3), read_unaligned)
                .wrapping_sub(mech_trace::rd!("1001608d", p4.add(1), read_unaligned))
                .wrapping_add(1)
                .wrapping_mul(0x10000);
            i_var1 = mech_trace::rd!("1001608d", p4.add(3), read_unaligned);
            i_var2 = mech_trace::rd!("1001608d", p4.add(1), read_unaligned);
            i_var3 = mech_trace::cr!("1001608d", "10015b30", FUN_10015b30(local_8, local_c));
            local_1c[0] =
                (i_var1.wrapping_sub(i_var2).wrapping_add(1) >> 1).wrapping_add(i_var3 >> 0x10);
            i_var1 = mech_trace::rd!("1001608d", p4.add(4), read_unaligned);
            i_var2 = mech_trace::rd!("1001608d", p4.add(2), read_unaligned);
            i_var3 = mech_trace::cr!("1001608d", "10015b30", FUN_10015b30(local_8, local_20));
            local_1c[1] =
                (i_var1.wrapping_sub(i_var2).wrapping_add(1) >> 1).wrapping_add(i_var3 >> 0x10);
            FUN_10066f36(param_4, local_1c.as_mut_ptr(), local_14.as_mut_ptr());
            let a = mech_trace::rd!("1001608d", p1.add(3), read_unaligned)
                .wrapping_sub(mech_trace::rd!("1001608d", p1.add(1), read_unaligned))
                .wrapping_add(1)
                >> 1;
            let b = mech_trace::rd!("1001608d", p4.add(3), read_unaligned)
                .wrapping_sub(mech_trace::rd!("1001608d", p4.add(1), read_unaligned))
                .wrapping_add(1)
                >> 1;
            local_14[0] = local_14[0].wrapping_add(a.wrapping_sub(b));
            let c = mech_trace::rd!("1001608d", p1.add(4), read_unaligned)
                .wrapping_sub(mech_trace::rd!("1001608d", p1.add(2), read_unaligned))
                .wrapping_add(1)
                >> 1;
            let d = mech_trace::rd!("1001608d", p4.add(4), read_unaligned)
                .wrapping_sub(mech_trace::rd!("1001608d", p4.add(2), read_unaligned))
                .wrapping_add(1)
                >> 1;
            local_14[1] = local_14[1].wrapping_add(c.wrapping_sub(d));
            mech_trace::cr!(
                "1001608d",
                "10056489",
                FUN_10056489(
                    param_1,
                    local_28[0],
                    local_28[1],
                    local_14[0],
                    local_14[1],
                    0,
                    param_3,
                )
            );
        }
    }
}
