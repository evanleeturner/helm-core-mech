// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008edf4: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10016b50(callback_likely: u32) {
    mech_trace::enter!("10016b50", callback_likely);
    // SAFETY: the global is the game's own memory; written by value.
    unsafe {
        mech_trace::gw!("10016b50", "1008edf4", DAT_1008edf4, callback_likely);
    }
}
