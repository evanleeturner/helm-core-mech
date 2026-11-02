// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1003c7f5() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003618(
        param_1: i32,
        param_2: u32,
        param_3: u32,
        param_4: i32,
        param_5: u32,
        param_6: u16,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10003783(
    param_1: i32,
    param_2: u32,
    param_3: u32,
    param_4: i32,
    param_5: u32,
    param_6: u16,
) -> i32 {
    mech_trace::enter!(
        "10003783", param_1, param_2, param_3, param_4, param_5, param_6
    );
    // SAFETY: both callees are the game's own functions; arguments are passed
    // through unchanged.
    unsafe {
        let mut i_var1 = mech_trace::cr!("10003783", "1003c7f5", FUN_1003c7f5()) as i32;
        if i_var1 < 1 {
            i_var1 = mech_trace::cr!(
                "10003783",
                "10003618",
                FUN_10003618(param_1, param_2, param_3, param_4, param_5, param_6)
            ) as i32;
        }
        mech_trace::rt!("10003783", i_var1)
    }
}
