// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_10092214: i32;
    static mut DAT_100920a0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063de0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10035cf0(param_1: i32) {
    mech_trace::enter!("10035cf0", param_1);
    // SAFETY: the globals and the callee are the game's own; the address of
    // DAT_100920a0 is passed on without creating a reference.
    unsafe {
        mech_trace::gw!("10035cf0", "_DAT_10092214", _DAT_10092214, param_1);
        FUN_10063de0(&raw mut DAT_100920a0);
    }
}
