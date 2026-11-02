// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af0a0: u32;
    static mut DAT_100c7bc0: u32;
    static mut DAT_100c7bc4: u32;
    static mut DAT_100c7bc8: u32;
    static mut DAT_100c7bd0: u32;
    static mut DAT_100c7bcc: u32;
    static mut DAT_100c7bd4: u32;
    static mut DAT_100c7bd8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __ftol() -> i64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003e979(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u32,
) {
    mech_trace::enter!(
        "1003e979", param_1, param_2, param_3, param_4, param_5, param_6
    );
    // SAFETY: the globals and __ftol are the game's own; the writes land at
    // the same addresses and in the same order as the original.
    unsafe {
        let i_var2 = mech_trace::gr!("1003e979", "100af0a0", DAT_100af0a0) as i32;
        if i_var2 < 10 {
            let i_var1 = i_var2.wrapping_mul(0x1c);
            let idx7 = i_var2.wrapping_mul(7) as isize;
            mech_trace::wr!(
                "1003e979",
                (&raw mut DAT_100c7bc0).wrapping_offset(idx7),
                write_unaligned,
                param_1
            );
            mech_trace::wr!(
                "1003e979",
                (&raw mut DAT_100c7bc4).wrapping_offset(idx7),
                write_unaligned,
                param_2
            );
            mech_trace::wr!(
                "1003e979",
                (&raw mut DAT_100c7bc8).wrapping_offset(i_var1 as isize),
                write_unaligned,
                param_3
            );
            mech_trace::wr!(
                "1003e979",
                (&raw mut DAT_100c7bd0).wrapping_offset(i_var1 as isize),
                write_unaligned,
                param_4
            );
            mech_trace::wr!(
                "1003e979",
                (&raw mut DAT_100c7bcc).wrapping_offset(i_var1 as isize),
                write_unaligned,
                param_5
            );
            mech_trace::wr!(
                "1003e979",
                (&raw mut DAT_100c7bd4).wrapping_offset(i_var1 as isize),
                write_unaligned,
                param_6
            );
            let l_var3 = mech_trace::cr!("1003e979", "__ftol", __ftol());
            mech_trace::wr!(
                "1003e979",
                (&raw mut DAT_100c7bd8).wrapping_offset(i_var1 as isize),
                write_unaligned,
                l_var3 as i32 as u32
            );
            mech_trace::gw!(
                "1003e979",
                "100af0a0",
                DAT_100af0a0,
                mech_trace::gr!("1003e979", "100af0a0", DAT_100af0a0).wrapping_add(1)
            );
        }
    }
}
