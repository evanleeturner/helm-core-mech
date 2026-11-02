// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_100ba7bc: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(
        param_1: *mut core::ffi::c_void,
        param_2: u32,
        param_3: u32,
    ) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1008605a() {
    mech_trace::enter!("1008605a");
    // SAFETY: HeapAlloc and the globals are the game's own; the heap handle
    // is read and the allocation result stored where the original does.
    unsafe {
        mech_trace::gw!(
            "1008605a",
            "100ba7bc",
            DAT_100ba7bc,
            mech_trace::cr!(
                "1008605a",
                "HeapAlloc",
                HeapAlloc(
                    mech_trace::gr!("1008605a", "100adf74", DAT_100adf74),
                    9,
                    0xfc4
                )
            )
        );
    }
}
