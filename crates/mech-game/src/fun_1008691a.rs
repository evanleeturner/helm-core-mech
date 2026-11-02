// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c9054: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10086336(param_1: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1008691a() -> i32 {
    mech_trace::enter!("1008691a");
    // SAFETY: the global and the callee are the game's own; the pointer is
    // passed on only when non-null, as the original does.
    unsafe {
        let b_var1 = !mech_trace::gr!("1008691a", "100c9054", DAT_100c9054).is_null();
        if b_var1 {
            FUN_10086336(mech_trace::gr!("1008691a", "100c9054", DAT_100c9054));
        }
        mech_trace::rt!("1008691a", b_var1 as i32)
    }
}
