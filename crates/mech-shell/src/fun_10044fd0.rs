// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_10099050: i32;
    static mut _DAT_10099088: i32;
    static mut _DAT_100990c0: i32;
    static mut _DAT_100990f8: i32;
    static mut DAT_10098fb8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063de0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10044fd0() {
    mech_trace::enter!("10044fd0");
    // SAFETY: the globals and the callee are the game's own; the address of
    // DAT_10098fb8 is passed on without creating a reference.
    unsafe {
        mech_trace::gw!("10044fd0", "10099050", _DAT_10099050, 0);
        mech_trace::gw!("10044fd0", "10099088", _DAT_10099088, 0);
        mech_trace::gw!("10044fd0", "100990c0", _DAT_100990c0, 0);
        mech_trace::gw!("10044fd0", "100990f8", _DAT_100990f8, 0);
        FUN_10063de0(&raw mut DAT_10098fb8);
    }
}
