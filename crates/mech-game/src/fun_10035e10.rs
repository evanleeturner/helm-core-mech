// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adcf0: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100869d5(param_1: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10035e10() {
    mech_trace::enter!("10035e10");
    // SAFETY: the global and the callee are the game's own; the pointer is
    // passed to its releaser and then cleared, as the original does.
    unsafe {
        if !mech_trace::gr!("10035e10", "100adcf0", DAT_100adcf0).is_null() {
            FUN_100869d5(mech_trace::gr!("10035e10", "100adcf0", DAT_100adcf0));
            mech_trace::gw!("10035e10", "100adcf0", DAT_100adcf0, core::ptr::null_mut());
        }
    }
}
