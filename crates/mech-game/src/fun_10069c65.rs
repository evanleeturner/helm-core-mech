// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3a20: u32;
    static mut DAT_100b39e8: [i8; 1];
    static mut DAT_100b3acc: [i8; 1];
}

unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10069c65() {
    mech_trace::enter!("10069c65");
    let mut local_24: [i8; 16] = [0; 16];
    let mut local_14: [i8; 16] = [0; 16];
    // SAFETY: the globals and sprintf are the game's own; the buffers are
    // local and the format strings are the game's NUL-terminated data.
    unsafe {
        mech_trace::cr!(
            "10069c65",
            "sprintf",
            sprintf(
                local_14.as_mut_ptr(),
                (&raw mut DAT_100b3acc).cast::<i8>(),
                0xei32,
                0xei32,
            )
        );
        mech_trace::cr!(
            "10069c65",
            "sprintf",
            sprintf(
                local_24.as_mut_ptr(),
                local_14.as_ptr(),
                (&raw mut DAT_100b39e8).cast::<i8>(),
            )
        );
        mech_trace::gw!("10069c65", "100b3a20", DAT_100b3a20, 0);
    }
}
