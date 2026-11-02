// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b32b8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005d8e0() -> i32 {
    mech_trace::enter!("1005d8e0");
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        mech_trace::rt!(
            "1005d8e0",
            mech_trace::gr!("1005d8e0", "100b32b8", DAT_100b32b8)
        )
    }
}
