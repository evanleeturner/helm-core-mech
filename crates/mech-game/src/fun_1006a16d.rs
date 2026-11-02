// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3a38: u32;
    static mut DAT_100b3a3c: u32;
    static mut DAT_100b3a64: u32;
    static mut DAT_100b3a68: u32;
    static s__8_8ld_100b3bf0: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1005c0df(param_1: i32, param_2: i32, param_3: *mut i8, param_4: i32, param_5: i32)
    -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006a16d() {
    mech_trace::enter!("1006a16d");
    let mut local_2c: [i8; 40] = [0; 40];
    // SAFETY: the globals and callees are the game's own; local_2c is a
    // local buffer large enough for the formatted output.
    unsafe {
        if mech_trace::gr!("1006a16d", "100b3a68", DAT_100b3a68) == 0 {
            mech_trace::gw!("1006a16d", "100b3a68", DAT_100b3a68, 1);
        }
        mech_trace::cr!(
            "1006a16d",
            "_sprintf",
            _sprintf(
                local_2c.as_mut_ptr(),
                mech_trace::gr!("1006a16d", "s__8_8ld_100b3bf0", s__8_8ld_100b3bf0).as_ptr(),
                0i32
            )
        );
        if mech_trace::gr!("1006a16d", "100b3a64", DAT_100b3a64) != 0 {
            mech_trace::cr!(
                "1006a16d",
                "1005c0df",
                FUN_1005c0df(
                    0x37,
                    1,
                    local_2c.as_mut_ptr(),
                    mech_trace::gr!("1006a16d", "100b3a38", DAT_100b3a38) as i32,
                    mech_trace::gr!("1006a16d", "100b3a3c", DAT_100b3a3c) as i32,
                )
            );
        }
    }
}
