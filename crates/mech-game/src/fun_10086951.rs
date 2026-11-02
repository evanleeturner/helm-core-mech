// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn HeapAlloc(
        bytes: *mut core::ffi::c_void,
        param_2: u32,
        param_3: u32,
    ) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10086951(bytes: u32) {
    mech_trace::enter!("10086951", bytes);
    // SAFETY: the heap handle global and HeapAlloc are the game's own; the
    // result is discarded as in the original.
    unsafe {
        mech_trace::cr!(
            "10086951",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10086951", "100adf74", DAT_100adf74),
                9,
                bytes
            )
        );
    }
}
