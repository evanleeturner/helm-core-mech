// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008d104: *mut core::ffi::c_void;
    static mut DAT_100b0558: *mut core::ffi::c_void;
    static mut DAT_100b055c: *mut core::ffi::c_void;
    static mut DAT_100b0560: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006e790(param_1: *mut core::ffi::c_void) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000d9b0() {
    mech_trace::enter!("1000d9b0");
    // SAFETY: the globals and the callee are the game's own; each global is
    // read by value in the original's order and passed to the callee.
    unsafe {
        mech_trace::cr!(
            "1000d9b0",
            "1006e790",
            FUN_1006e790(mech_trace::gr!("1000d9b0", "1008d104", DAT_1008d104))
        );
        mech_trace::cr!(
            "1000d9b0",
            "1006e790",
            FUN_1006e790(mech_trace::gr!("1000d9b0", "100b055c", DAT_100b055c))
        );
        mech_trace::cr!(
            "1000d9b0",
            "1006e790",
            FUN_1006e790(mech_trace::gr!("1000d9b0", "100b0558", DAT_100b0558))
        );
        mech_trace::cr!(
            "1000d9b0",
            "1006e790",
            FUN_1006e790(mech_trace::gr!("1000d9b0", "100b0560", DAT_100b0560))
        );
    }
}
