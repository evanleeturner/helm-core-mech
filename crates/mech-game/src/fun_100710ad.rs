// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10070f83(param_1: i32, param_2: *mut i8) -> i32;
    fn FUN_10071045(param_1: *mut u32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100710ad(param_1: *mut u32, param_2: *mut i8) -> u32 {
    mech_trace::enter!("100710ad", param_1, param_2);
    // SAFETY: the callees are the game's own; the pointers are passed through unchanged.
    unsafe {
        let i_var1 = mech_trace::cr!(
            "100710ad",
            "10070f83",
            FUN_10070f83(param_1 as i32, param_2)
        );
        mech_trace::rt!(
            "100710ad",
            if i_var1 == -1 {
                0xffffffff
            } else {
                mech_trace::cr!("100710ad", "10071045", FUN_10071045(param_1, i_var1))
            }
        )
    }
}
