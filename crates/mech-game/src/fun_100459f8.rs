// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100affd8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10045873(param_1: i32, param_2: i32, param_3: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100459f8(param_1: i32, param_2: i32, param_3: i32) -> u32 {
    mech_trace::enter!("100459f8", param_1, param_2, param_3);
    // SAFETY: the global and the callee are the game's own.
    unsafe {
        mech_trace::rt!(
            "100459f8",
            if mech_trace::gr!("100459f8", "100affd8", DAT_100affd8) == 0 {
                0xffffffff
            } else {
                let i_var2 = mech_trace::cr!(
                    "100459f8",
                    "10045873",
                    FUN_10045873(param_1, param_2, param_3)
                );
                if i_var2 == 0 { 0xffffffff } else { 0 }
            }
        )
    }
}
