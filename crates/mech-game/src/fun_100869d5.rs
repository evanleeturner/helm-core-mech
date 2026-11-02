// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(mem: *mut core::ffi::c_void, param_2: u32, param_3: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100869d5(mem: *mut core::ffi::c_void) {
    mech_trace::enter!("100869d5", mem);
    // SAFETY: the heap handle global and HeapFree are the game's own; mem is
    // passed through unchanged as the original does.
    unsafe {
        mech_trace::cr!(
            "100869d5",
            "HeapFree",
            HeapFree(
                mech_trace::gr!("100869d5", "100adf74", DAT_100adf74),
                1,
                mem
            )
        );
    }
}
