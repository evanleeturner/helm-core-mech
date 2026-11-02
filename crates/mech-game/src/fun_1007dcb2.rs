// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b94cc: [i8; 1];
    static mut DAT_100b94c8: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FILE_read(filename: *mut i8, param2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007dcb2(param_1: u32) -> u32 {
    mech_trace::enter!("1007dcb2", param_1);
    let mut local_104: [i8; 256] = [0; 256];
    // SAFETY: the globals, sprintf and FILE_read are the game's own; param_1
    // is the caller's NUL-terminated string address, and local_104 is a
    // local buffer passed as the destination and filename.
    unsafe {
        mech_trace::cr!(
            "1007dcb2",
            "_sprintf",
            _sprintf(
                local_104.as_mut_ptr(),
                b"%s/%s.%s\0".as_ptr().cast::<i8>(),
                (&raw mut DAT_100b94cc).cast::<i8>(),
                param_1 as usize as *const i8,
                (&raw mut DAT_100b94c8).cast::<i8>(),
            )
        );
        mech_trace::rt!(
            "1007dcb2",
            mech_trace::cr!(
                "1007dcb2",
                "FILE_read",
                FILE_read(local_104.as_mut_ptr(), 0)
            )
        )
    }
}
