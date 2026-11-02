// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9a2c: *mut u32;
    static mut DAT_100b9d30: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10045d6f() -> u32 {
    mech_trace::enter!("10045d6f");
    // SAFETY: the globals are the game's own memory; DAT_100b9a2c holds a
    // valid pointer to a u32 slot that the original writes through.
    unsafe {
        mech_trace::wr!(
            "10045d6f",
            mech_trace::gr!("10045d6f", "100b9a2c", DAT_100b9a2c),
            write_unaligned,
            mech_trace::gr!("10045d6f", "100b9d30", DAT_100b9d30)
        );
    }
    mech_trace::rt!("10045d6f", 0)
}
