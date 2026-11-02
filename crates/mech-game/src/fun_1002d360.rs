// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_100a1130: f64;
    static mut DAT_100c6b18: u8;
    static mut DAT_100ad6e8: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(buf: *mut u8, fmt: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002d360(param_1: i32) -> *mut u8 {
    mech_trace::enter!("1002d360", param_1);
    // SAFETY: the globals and sprintf are the game's own; the buffer is the
    // game's static output buffer and the format is its own string.
    unsafe {
        let r = param_1 % 0x9f150;
        let i_var2 = r / 0x2a6c;
        let d_var1 = (r.wrapping_add(i_var2.wrapping_mul(-0x2a6c)) as f64)
            / mech_trace::gr!("1002d360", "_DAT_100a1130", _DAT_100a1130);
        mech_trace::cr!(
            "1002d360",
            "sprintf",
            sprintf(
                &raw mut DAT_100c6b18,
                (&raw const DAT_100ad6e8).cast::<i8>(),
                param_1 / 0x9f150,
                i_var2,
                d_var1,
            )
        );
        mech_trace::rt!("1002d360", &raw mut DAT_100c6b18)
    }
}
