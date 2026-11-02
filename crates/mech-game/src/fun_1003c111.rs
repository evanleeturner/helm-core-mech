// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefa8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003c111() -> u8 {
    mech_trace::enter!("1003c111");
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        mech_trace::rt!(
            "1003c111",
            (mech_trace::gr!("1003c111", "100aefa8", DAT_100aefa8) == 1) as u8
        )
    }
}
