// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38e0: *mut core::ffi::c_void;
    static mut DAT_100b38e4: *mut core::ffi::c_void;
    static mut DAT_100b38e8: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008d9b0(param_1: *mut core::ffi::c_void) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100654db() {
    mech_trace::enter!("100654db");
    // SAFETY: the globals and the callee are the game's own; each pointer is
    // freed through the game's routine and then cleared, in the original order.
    unsafe {
        if !mech_trace::gr!("100654db", "100b38e8", DAT_100b38e8).is_null() {
            mech_trace::cr!(
                "100654db",
                "1008d9b0",
                FUN_1008d9b0(mech_trace::gr!("100654db", "100b38e8", DAT_100b38e8))
            );
            mech_trace::gw!("100654db", "100b38e8", DAT_100b38e8, core::ptr::null_mut());
        }
        if !mech_trace::gr!("100654db", "100b38e4", DAT_100b38e4).is_null() {
            mech_trace::cr!(
                "100654db",
                "1008d9b0",
                FUN_1008d9b0(mech_trace::gr!("100654db", "100b38e4", DAT_100b38e4))
            );
            mech_trace::gw!("100654db", "100b38e4", DAT_100b38e4, core::ptr::null_mut());
        }
        if !mech_trace::gr!("100654db", "100b38e0", DAT_100b38e0).is_null() {
            mech_trace::cr!(
                "100654db",
                "1008d9b0",
                FUN_1008d9b0(mech_trace::gr!("100654db", "100b38e0", DAT_100b38e0))
            );
            mech_trace::gw!("100654db", "100b38e0", DAT_100b38e0, core::ptr::null_mut());
        }
    }
}
