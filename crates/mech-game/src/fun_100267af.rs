// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10011e60(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_1002650e(param_1: *mut i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100267af(param_1: *mut i32, param_2: i32) {
    mech_trace::enter!("100267af", param_1, param_2);
    // SAFETY: param_1 points at the caller's int array; the callees are the
    // game's own functions, called in the original order.
    unsafe {
        if mech_trace::rd!("100267af", param_1.add(7), read_unaligned) != 0 {
            let i_var1 = mech_trace::cr!(
                "100267af",
                "10011e60",
                FUN_10011e60(
                    mech_trace::rd!("100267af", param_1.add(0xc), read_unaligned),
                    param_2.wrapping_sub(mech_trace::rd!(
                        "100267af",
                        param_1.add(1),
                        read_unaligned
                    )),
                    mech_trace::rd!("100267af", param_1.add(7), read_unaligned),
                )
            );
            mech_trace::cr!("100267af", "1002650e", FUN_1002650e(param_1, i_var1 as i32));
        }
    }
}
