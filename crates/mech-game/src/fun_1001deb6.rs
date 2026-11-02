// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab2cc: [i8; 0];
    static mut DAT_101e8690: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001ddf0(param_1: i32) -> *mut i8;
    fn FUN_1001de5c(param_1: *mut i8) -> u8;
    fn sprintf(dest: *mut i8, format: *mut i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001deb6(param_1: i32) {
    mech_trace::enter!("1001deb6", param_1);
    let mut local_108: [i8; 256] = [0; 256];
    // SAFETY: the callees and globals are the game's own; local_108 is a
    // 256-byte local buffer passed where the original passes it.
    unsafe {
        let _ = &raw mut DAT_101e8690;
        let local_8 = mech_trace::cr!("1001deb6", "1001ddf0", FUN_1001ddf0(param_1));
        mech_trace::cr!(
            "1001deb6",
            "sprintf",
            sprintf(
                local_108.as_mut_ptr(),
                (&raw mut DAT_100ab2cc).cast::<i8>(),
                local_8,
            )
        );
        mech_trace::cr!("1001deb6", "1001de5c", FUN_1001de5c(local_108.as_mut_ptr()));
    }
}
