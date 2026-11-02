// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10218074: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10002415() -> u32 {
    mech_trace::enter!("10002415");
    // SAFETY: DAT_10218074 is the game's own global, read by value.
    unsafe {
        mech_trace::rt!(
            "10002415",
            mech_trace::gr!("10002415", "10218074", DAT_10218074)
        )
    }
}
