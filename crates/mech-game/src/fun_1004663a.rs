// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7d00: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10046563() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004663a() -> u32 {
    mech_trace::enter!("1004663a");
    // SAFETY: the callee and the global are the game's own memory; the
    // global is written with the callee's result and read back.
    unsafe {
        mech_trace::gw!(
            "1004663a",
            "100c7d00",
            DAT_100c7d00,
            mech_trace::cr!("1004663a", "10046563", FUN_10046563())
        );
        mech_trace::rt!(
            "1004663a",
            mech_trace::gr!("1004663a", "100c7d00", DAT_100c7d00)
        )
    }
}
