// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3a68: u32;
    static mut DAT_100b39e8: [i8; 1];
    static mut DAT_100b3bf8: [i8; 1];
}

unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006a1f1() {
    mech_trace::enter!("1006a1f1");
    let mut local_24: [i8; 16] = [0; 16];
    let mut local_14: [i8; 16] = [0; 16];
    // SAFETY: the globals and sprintf are the game's own; the buffers are
    // local arrays passed by pointer, as the original does.
    unsafe {
        mech_trace::cr!(
            "1006a1f1",
            "sprintf",
            sprintf(
                local_14.as_mut_ptr(),
                (&raw mut DAT_100b3bf8).cast::<i8>(),
                0xc_i32,
                0xc_i32,
            )
        );
        mech_trace::cr!(
            "1006a1f1",
            "sprintf",
            sprintf(
                local_24.as_mut_ptr(),
                local_14.as_ptr(),
                (&raw mut DAT_100b39e8).cast::<i8>(),
            )
        );
        mech_trace::gw!("1006a1f1", "100b3a68", DAT_100b3a68, 0);
    }
}
