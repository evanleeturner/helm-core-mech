// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7ce8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10046900() -> u32 {
    mech_trace::enter!("10046900");
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        mech_trace::rt!(
            "10046900",
            mech_trace::gr!("10046900", "100c7ce8", DAT_100c7ce8)
        )
    }
}
