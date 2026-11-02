// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f5300: i32;
    static mut DAT_100b3a30: i32;
    static mut DAT_100b3a40: i32;
    static mut DAT_100b3a38: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10066004(
        param_1: *mut core::ffi::c_void,
        param_2: *mut core::ffi::c_void,
        param_3: *mut core::ffi::c_void,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006a46b() {
    mech_trace::enter!("1006a46b");
    // SAFETY: the callee and the globals are the game's own; only their
    // addresses are passed, in the original's order.
    unsafe {
        FUN_10066004(
            (&raw mut DAT_101f5300).cast(),
            (&raw mut DAT_100b3a30).cast(),
            (&raw mut DAT_100b3a30).cast(),
        );
        FUN_10066004(
            (&raw mut DAT_101f5300).cast(),
            (&raw mut DAT_100b3a40).cast(),
            (&raw mut DAT_100b3a40).cast(),
        );
        FUN_10066004(
            (&raw mut DAT_101f5300).cast(),
            (&raw mut DAT_100b3a38).cast(),
            (&raw mut DAT_100b3a38).cast(),
        );
    }
}
