// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a75e4: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000e4fc() -> u32 {
    mech_trace::enter!("1000e4fc");
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        mech_trace::rt!(
            "1000e4fc",
            mech_trace::gr!("1000e4fc", "100a75e4", DAT_100a75e4)
        )
    }
}
