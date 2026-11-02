// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f52c4: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000ed93() -> u32 {
    mech_trace::enter!("1000ed93");
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        mech_trace::rt!(
            "1000ed93",
            mech_trace::gr!("1000ed93", "101f52c4", DAT_101f52c4)
        )
    }
}
