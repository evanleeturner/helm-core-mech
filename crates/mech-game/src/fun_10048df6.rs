// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100486a4(param_1: i32);
    fn FUN_10058e74(param_1: i32, param_2: i32);
    fn FUN_10056fc4(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_10066609(param_1: i32, param_2: i32);
    fn FUN_10049007(param_1: *mut i32);
    fn FUN_10049844(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10048df6(param_1: *mut i32) -> i32 {
    mech_trace::enter!("10048df6", param_1);
    // SAFETY: param_1 points at the caller's object; the callees are the
    // game's own functions, called in the original order.
    unsafe {
        let i_var1 = mech_trace::rd!("10048df6", param_1, read_unaligned);
        let i_var2 = mech_trace::rd!("10048df6", param_1.add(6), read_unaligned);

        if i_var1 == 0 {
            return mech_trace::rt!("10048df6", 0);
        }
        if i_var2 == 0 {
            return mech_trace::rt!("10048df6", 0);
        }

        FUN_100486a4(param_1 as i32);

        if (mech_trace::rd!("10048df6", param_1.add(1).cast::<u8>(), read) & 0x20) != 0 {
            FUN_10058e74(i_var1, 0);
        }

        let i_var3 = mech_trace::rd!("10048df6", param_1.add(5), read_unaligned);
        if i_var3 != 0 {
            FUN_10056fc4(
                i_var2,
                mech_trace::rd!("10048df6", param_1.add(5), read_unaligned),
                0,
                0,
                0,
            );
        }

        if (mech_trace::rd!("10048df6", param_1.add(1).cast::<u8>(), read) & 4) != 0 {
            FUN_10066609(i_var1, 1);
        }

        FUN_10049007(param_1);

        mech_trace::rt!(
            "10048df6",
            (mech_trace::cr!("10048df6", "10049844", FUN_10049844(param_1 as i32)) == 0) as i32
        )
    }
}
