// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c4c: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000387d() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002ba7(
        param_1: i32,
        param_2: u32,
        param_3: u32,
        param_4: *mut core::ffi::c_void,
        param_5: i32,
        param_6: i32,
        param_7: u32,
        param_8: u32,
        param_9: i32,
        param_10: u16,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10003618(
    param_1: i32,
    param_2: u32,
    param_3: u32,
    param_4: i32,
    param_5: u32,
    param_6: u16,
) {
    mech_trace::enter!(
        "10003618", param_1, param_2, param_3, param_4, param_5, param_6
    );
    // SAFETY: the callees and the global are the game's own; arguments are
    // passed through in the original's order.
    unsafe {
        let i_var2: i32 = -1;
        let u_var1 = mech_trace::cr!("10003618", "1000387d", FUN_1000387d());
        mech_trace::cr!(
            "10003618",
            "10002ba7",
            FUN_10002ba7(
                param_1,
                param_2,
                param_3,
                core::ptr::null_mut(),
                param_4,
                mech_trace::gr!("10003618", "100b3c4c", DAT_100b3c4c) as i32,
                param_5,
                u_var1,
                i_var2,
                param_6,
            )
        );
    }
}
