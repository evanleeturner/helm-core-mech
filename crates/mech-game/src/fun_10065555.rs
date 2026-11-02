// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38e0: *mut core::ffi::c_void;
    static mut DAT_100b38e4: *mut core::ffi::c_void;
    static mut DAT_100b38e8: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100654db() -> u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _malloc(size: usize) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10065555() -> u32 {
    mech_trace::enter!("10065555");
    // SAFETY: the callees and globals are the game's own; the globals hold
    // the allocations made here.
    unsafe {
        mech_trace::cr!("10065555", "100654db", FUN_100654db());
        mech_trace::gw!(
            "10065555",
            "100b38e0",
            DAT_100b38e0,
            mech_trace::cr!("10065555", "_malloc", _malloc(0x1000))
        );
        if mech_trace::gr!("10065555", "100b38e0", DAT_100b38e0).is_null() {
            mech_trace::cr!("10065555", "100654db", FUN_100654db());
            return mech_trace::rt!("10065555", 3);
        }
        mech_trace::gw!(
            "10065555",
            "100b38e4",
            DAT_100b38e4,
            mech_trace::cr!("10065555", "_malloc", _malloc(0x4000))
        );
        if mech_trace::gr!("10065555", "100b38e4", DAT_100b38e4).is_null() {
            mech_trace::cr!("10065555", "100654db", FUN_100654db());
            return mech_trace::rt!("10065555", 3);
        }
        mech_trace::gw!(
            "10065555",
            "100b38e8",
            DAT_100b38e8,
            mech_trace::cr!("10065555", "_malloc", _malloc(0x9bd4))
        );
        if mech_trace::gr!("10065555", "100b38e8", DAT_100b38e8).is_null() {
            mech_trace::cr!("10065555", "100654db", FUN_100654db());
            return mech_trace::rt!("10065555", 3);
        }
        mech_trace::rt!("10065555", 0)
    }
}
