// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aafbc: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1002cdb5() -> bool {
    mech_trace::enter!("1002cdb5");
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        mech_trace::rt!(
            "1002cdb5",
            mech_trace::gr!("1002cdb5", "100aafbc", DAT_100aafbc) == 0
        )
    }
}
