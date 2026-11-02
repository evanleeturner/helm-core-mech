// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3444: u32;
    static mut DAT_100b3448: *mut u8;
    static mut DAT_100b34a0: [i8; 1];
}

unsafe extern "C" {
    fn fprintf(file: *mut u8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005dd5a(param_1: u32) -> u32 {
    mech_trace::enter!("1005dd5a", param_1);
    // SAFETY: the globals and the CRT's fprintf are the game's own; the
    // format string and the file pointer are used as the original does.
    unsafe {
        mech_trace::rt!(
            "1005dd5a",
            if mech_trace::gr!("1005dd5a", "100b3444", DAT_100b3444) == 0
                || mech_trace::gr!("1005dd5a", "100b3448", DAT_100b3448).is_null()
            {
                0
            } else {
                mech_trace::cr!(
                    "1005dd5a",
                    "fprintf",
                    fprintf(
                        mech_trace::gr!("1005dd5a", "100b3448", DAT_100b3448),
                        (&raw mut DAT_100b34a0).cast::<i8>() as *const i8,
                        param_1,
                    )
                );
                1
            }
        )
    }
}
