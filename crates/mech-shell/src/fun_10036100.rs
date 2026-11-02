// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093b38: [u8; 0];
    static mut DAT_10093b2c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036100(param_1: i32) -> *mut u8 {
    mech_trace::enter!("10036100", param_1);
    let mut param_1 = param_1;
    // SAFETY: the globals are the game's own; DAT_10093b2c holds the address
    // of a table whose header fields are read where the original reads them.
    unsafe {
        if param_1 < 0 {
            param_1 = mech_trace::rd!(
                "10036100",
                ((mech_trace::gr!("10036100", "10093b2c", DAT_10093b2c).wrapping_add(4))
                    as *const i32),
                read_unaligned
            );
        }
        if param_1
            < mech_trace::rd!(
                "10036100",
                ((mech_trace::gr!("10036100", "10093b2c", DAT_10093b2c).wrapping_add(8))
                    as *const i32),
                read_unaligned
            )
            && param_1
                < mech_trace::rd!(
                    "10036100",
                    ((mech_trace::gr!("10036100", "10093b2c", DAT_10093b2c).wrapping_add(0xc))
                        as *const i32),
                    read_unaligned
                )
        {
            return mech_trace::rt!(
                "10036100",
                mech_trace::gr!("10036100", "10093b2c", DAT_10093b2c)
                    .wrapping_add((param_1 as u32).wrapping_mul(0x40))
                    .wrapping_add(0x18) as *mut u8
            );
        }
        mech_trace::rt!("10036100", &raw mut DAT_10093b38 as *mut u8)
    }
}
