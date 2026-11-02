// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001bdc6(param_1: *mut u16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005fbab(
    param_1: *mut u16,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: *mut u32,
    param_6: *mut u32,
    param_7: *mut i32,
) {
    mech_trace::enter!(
        "1005fbab", param_1, param_2, param_3, param_4, param_5, param_6, param_7
    );
    // SAFETY: the callee is the game's own; param_1 points at the caller's
    // object, whose field at word offset 0x26 is a pointer to an int array;
    // the output pointers are the caller's and written where the original does.
    unsafe {
        mech_trace::cr!("1005fbab", "1001bdc6", FUN_1001bdc6(param_1));
        let pi_var1 = mech_trace::rd!(
            "1005fbab",
            param_1.add(0x26).cast::<*mut i32>(),
            read_unaligned
        );
        if pi_var1.is_null() {
            mech_trace::dw!("1005fbab", param_6, 0);
            mech_trace::dw!("1005fbab", param_5, 0);
        } else if param_2 < mech_trace::rd!("1005fbab", pi_var1, read_unaligned)
            || mech_trace::rd!("1005fbab", pi_var1.add(1), read_unaligned) < param_2
            || param_4 < mech_trace::rd!("1005fbab", pi_var1.add(4), read_unaligned)
            || mech_trace::rd!("1005fbab", pi_var1.add(5), read_unaligned) < param_4
        {
            mech_trace::dw!("1005fbab", param_6, 0);
            mech_trace::dw!("1005fbab", param_5, 0);
        } else {
            mech_trace::dw!("1005fbab", param_6, 1);
            mech_trace::dw!(
                "1005fbab",
                param_7,
                mech_trace::rd!("1005fbab", pi_var1.add(3), read_unaligned)
            );
            if param_3 < mech_trace::rd!("1005fbab", pi_var1.add(2), read_unaligned)
                || mech_trace::rd!("1005fbab", pi_var1.add(3), read_unaligned) < param_3
            {
                mech_trace::dw!("1005fbab", param_5, 0);
            } else {
                mech_trace::dw!("1005fbab", param_5, 1);
            }
        }
    }
}
