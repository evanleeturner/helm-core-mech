// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a67f4: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10010930() -> u32 {
    mech_trace::enter!("10010930");
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        mech_trace::rt!(
            "10010930",
            mech_trace::gr!("10010930", "100a67f4", DAT_100a67f4)
        )
    }
}
