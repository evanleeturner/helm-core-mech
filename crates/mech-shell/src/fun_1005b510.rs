// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c6398: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006e790(param_1: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005b510() {
    mech_trace::enter!("1005b510");
    // SAFETY: the global and the callee are the game's own; the global is
    // passed by value to the callee as the original does.
    unsafe {
        FUN_1006e790(mech_trace::gr!("1005b510", "100c6398", DAT_100c6398));
    }
}
