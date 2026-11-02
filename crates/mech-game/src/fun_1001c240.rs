// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab0c0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001c240() -> i32 {
    mech_trace::enter!("1001c240");
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        mech_trace::rt!(
            "1001c240",
            mech_trace::gr!("1001c240", "100ab0c0", DAT_100ab0c0)
        )
    }
}
