// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b4e64: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10072879() -> u32 {
    mech_trace::enter!("10072879");
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        mech_trace::rt!(
            "10072879",
            mech_trace::gr!("10072879", "100b4e64", DAT_100b4e64)
        )
    }
}
