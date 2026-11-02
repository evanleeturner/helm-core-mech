// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af0a4: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003e9fe() -> u32 {
    mech_trace::enter!("1003e9fe");
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        mech_trace::rt!(
            "1003e9fe",
            mech_trace::gr!("1003e9fe", "100af0a4", DAT_100af0a4)
        )
    }
}
