// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa6b8: u32;
    static mut DAT_100aa6bc: u32;
    static mut DAT_100aa6c0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006b230(param_1: i32, param_2: i32, param_3: i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006cf90() -> i32 {
    mech_trace::enter!("1006cf90");
    // SAFETY: the callee and the globals are the game's own memory.
    unsafe {
        let i_var1 = mech_trace::cr!(
            "1006cf90",
            "1006b230",
            FUN_1006b230(
                mech_trace::gr!("1006cf90", "100aa6c0", DAT_100aa6c0) as i32,
                mech_trace::gr!("1006cf90", "100aa6bc", DAT_100aa6bc) as i32,
                1
            )
        );
        let i_var2 = mech_trace::cr!(
            "1006cf90",
            "1006b230",
            FUN_1006b230(
                mech_trace::gr!("1006cf90", "100aa6b8", DAT_100aa6b8) as i32,
                mech_trace::gr!("1006cf90", "100aa6bc", DAT_100aa6bc) as i32,
                1
            )
        );
        mech_trace::rt!("1006cf90", i_var1.wrapping_sub(i_var2))
    }
}
