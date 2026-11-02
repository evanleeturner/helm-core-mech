// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b98cc: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10081830();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10081975() -> i32 {
    mech_trace::enter!("10081975");
    // SAFETY: the global and the callee are the game's own memory and code,
    // accessed in the same order as the original.
    unsafe {
        if mech_trace::gr!("10081975", "100b98cc", DAT_100b98cc) == 0 {
            FUN_10081830();
        }
        mech_trace::rt!(
            "10081975",
            mech_trace::gr!("10081975", "100b98cc", DAT_100b98cc)
        )
    }
}
