// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9a2c: *mut u32;
    static mut DAT_100b9d30: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006fa34() -> u32 {
    mech_trace::enter!("1006fa34");
    // SAFETY: the globals are the game's own memory; DAT_100b9a2c holds a
    // pointer to a writable u32, written where the original does.
    unsafe {
        mech_trace::wr!(
            "1006fa34",
            mech_trace::gr!("1006fa34", "100b9a2c", DAT_100b9a2c),
            write_unaligned,
            mech_trace::gr!("1006fa34", "100b9d30", DAT_100b9d30)
        );
    }
    mech_trace::rt!("1006fa34", 0)
}
