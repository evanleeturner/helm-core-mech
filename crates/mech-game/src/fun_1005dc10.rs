// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3314: *mut c_void;
    static mut DAT_100b3440: *mut c_void;
    static mut s_MW2_FM__Name__BETA_PATCH____Revi_100b32bc: [i8; 1];
    static mut s_Project_File__Name__A10_OP3E_ALP_100b331c: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _printf(param_1: *mut c_void, param_2: *mut c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005dc10() {
    mech_trace::enter!("1005dc10");
    // SAFETY: the globals and _printf are the game's own; only their
    // addresses are passed, in the original's order.
    unsafe {
        _printf(
            (&raw mut DAT_100b3314).cast::<c_void>(),
            (&raw mut s_MW2_FM__Name__BETA_PATCH____Revi_100b32bc).cast::<c_void>(),
        );
        _printf(
            (&raw mut DAT_100b3440).cast::<c_void>(),
            (&raw mut s_Project_File__Name__A10_OP3E_ALP_100b331c).cast::<c_void>(),
        );
    }
}
