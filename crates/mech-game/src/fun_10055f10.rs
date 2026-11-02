// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf80: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10055f9e(
        param_1: i32,
        param_2: i32,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10055f10(param_1: u32, param_2: i32, param_3: *mut i32, param_4: i32) {
    mech_trace::enter!("10055f10", param_1, param_2, param_3, param_4);
    let u_var1 = param_1 & 0xf;
    if u_var1 == 0 || (u_var1 != 1 && u_var1 == 3) {
        // SAFETY: the global and the callee are the game's own; param_3 is
        // passed through untouched.
        unsafe {
            if (mech_trace::gr!("10055f10", "100aaf80", DAT_100aaf80) & 1) != 0 {
                mech_trace::cr!(
                    "10055f10",
                    "10055f9e",
                    FUN_10055f9e(
                        ((param_1 & 0xff0) >> 4) as i32,
                        param_2,
                        param_3,
                        param_4,
                        0,
                    )
                );
            }
        }
    }
}
