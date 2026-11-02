// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38f8: [i8; 1];
    static mut DAT_100c7f9c: *mut u8;
}

unsafe extern "C" {
    fn fopen(filename: *mut i8, mode: *mut i8) -> *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10064c99(filename: *mut i8) -> bool {
    mech_trace::enter!("10064c99", filename);
    // SAFETY: fopen and the globals are the game's own; the mode string is
    // the game's array, passed by address, and filename is the caller's.
    unsafe {
        mech_trace::gw!(
            "10064c99",
            "100c7f9c",
            DAT_100c7f9c,
            mech_trace::cr!(
                "10064c99",
                "fopen",
                fopen(filename, (&raw mut DAT_100b38f8).cast::<i8>())
            )
        );
        mech_trace::rt!(
            "10064c99",
            mech_trace::gr!("10064c99", "100c7f9c", DAT_100c7f9c).is_null()
        )
    }
}
