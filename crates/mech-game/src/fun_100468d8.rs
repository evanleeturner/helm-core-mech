// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7cec: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100468d8() -> u32 {
    mech_trace::enter!("100468d8");
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        mech_trace::rt!(
            "100468d8",
            mech_trace::gr!("100468d8", "100c7cec", DAT_100c7cec)
        )
    }
}
