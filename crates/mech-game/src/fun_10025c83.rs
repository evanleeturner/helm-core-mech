// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_10026815(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut i32,
        param_6: *mut i32,
    ) -> i32;
    fn FUN_1002634e(param_1: i32) -> i32;
    fn FUN_10025e99(param_1: i32, param_2: i32, param_3: *mut i32) -> u8;
    fn FUN_10014db6(param_1: i32, param_2: i32, param_3: *mut i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10025c83(param_1: *mut i32, param_2: i32, param_3: *mut i32) -> u32 {
    mech_trace::enter!("10025c83", param_1, param_2, param_3);
    let mut local_20: i32 = 0;
    let mut local_1c: i32 = 0;
    let mut local_10: i32;
    let mut local_8: i32 = 0;

    if param_1.is_null() {
        return mech_trace::rt!("10025c83", 0);
    }
    // SAFETY: the callees are the game's own; param_1 and param_3 point at
    // the caller's int arrays, read at the same offsets as the original.
    unsafe {
        let mut iVar3 = mech_trace::cr!(
            "10025c83",
            "10026815",
            FUN_10026815(
                mech_trace::dr!("10025c83", param_3),
                mech_trace::dr!("10025c83", param_3.add(9)),
                mech_trace::dr!("10025c83", param_1),
                mech_trace::dr!("10025c83", param_1.add(1)),
                &raw mut local_8,
                &raw mut local_20,
            )
        );
        if iVar3 != 0 {
            return mech_trace::rt!("10025c83", 0);
        }
        local_10 = 0;
        iVar3 = mech_trace::cr!(
            "10025c83",
            "10026815",
            FUN_10026815(
                mech_trace::dr!("10025c83", param_3.add(1)),
                mech_trace::dr!("10025c83", param_3.add(10)),
                mech_trace::dr!("10025c83", param_1.add(2)),
                mech_trace::dr!("10025c83", param_1.add(3)),
                &raw mut local_10,
                &raw mut local_1c,
            )
        );
        if iVar3 != 0 {
            return mech_trace::rt!("10025c83", 0);
        }
        if local_8 < local_10 {
            local_8 = local_10;
        }
        if local_1c < local_20 {
            local_20 = local_1c;
        }
        iVar3 = mech_trace::cr!(
            "10025c83",
            "10026815",
            FUN_10026815(
                mech_trace::dr!("10025c83", param_3.add(2)),
                mech_trace::dr!("10025c83", param_3.add(11)),
                mech_trace::dr!("10025c83", param_1.add(4)),
                mech_trace::dr!("10025c83", param_1.add(5)),
                &raw mut local_10,
                &raw mut local_1c,
            )
        );
        if iVar3 != 0 {
            return mech_trace::rt!("10025c83", 0);
        }
        if local_8 < local_10 {
            local_8 = local_10;
        }
        if local_1c < local_20 {
            local_20 = local_1c;
        }
        if local_20 < local_8 {
            return mech_trace::rt!("10025c83", 0);
        }
        if local_8 < 0 {
            local_8 = 0;
        }
        local_10 = mech_trace::cr!("10025c83", "1002634e", FUN_1002634e(param_3 as i32));
        if local_10 < local_20 {
            local_20 = local_10;
        }
        if local_20 < local_8 {
            return mech_trace::rt!("10025c83", 0);
        }
        if mech_trace::dr!("10025c83", param_1.add(6)) == 0 {
            let b_var1 = mech_trace::cr!(
                "10025c83",
                "10025e99",
                FUN_10025e99(param_1 as i32, param_2, param_3)
            );
            if b_var1 == 0 {
                return mech_trace::rt!("10025c83", 0);
            }
            return mech_trace::rt!("10025c83", 1);
        }
        let local_c = (param_2 as u32).wrapping_add(0x18) as i32;
        let mut local_14 = param_1.add(0xb);
        let mut local_18: i32 = 0;
        while local_18 < mech_trace::dr!("10025c83", param_1.add(6)) {
            let i_var3 = mech_trace::dr!("10025c83", local_14);
            local_14 = local_14.add(1);
            if mech_trace::cr!(
                "10025c83",
                "10014db6",
                FUN_10014db6(i_var3, local_c, param_3)
            ) != 0
            {
                return mech_trace::rt!("10025c83", 1);
            }
            local_18 = local_18.wrapping_add(1);
        }
        mech_trace::rt!("10025c83", 0)
    }
}
