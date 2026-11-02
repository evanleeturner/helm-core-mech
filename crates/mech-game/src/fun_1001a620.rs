// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab088: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001a620() -> i32 {
    mech_trace::enter!("1001a620");
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        mech_trace::rt!(
            "1001a620",
            mech_trace::gr!("1001a620", "100ab088", DAT_100ab088)
        )
    }
}
