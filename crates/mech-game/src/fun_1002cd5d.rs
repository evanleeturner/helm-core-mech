// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aafc0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002cd5d(mask: u32) -> bool {
    mech_trace::enter!("1002cd5d", mask);
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        mech_trace::rt!(
            "1002cd5d",
            (mask & mech_trace::gr!("1002cd5d", "100aafc0", DAT_100aafc0)) == 0
        )
    }
}
