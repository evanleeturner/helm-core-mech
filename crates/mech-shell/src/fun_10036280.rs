// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093a24: i32;
    static mut DAT_10093a2c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001a730(param_1: i32, param_2: i32, param_3: i32, param_4: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10036280() {
    mech_trace::enter!("10036280");
    // SAFETY: the callee and the globals are the game's own; the address of
    // DAT_10093a2c is passed through as the original does.
    unsafe {
        FUN_1001a730(
            0,
            0,
            mech_trace::gr!("10036280", "10093a24", DAT_10093a24),
            &raw mut DAT_10093a2c,
        );
    }
}
