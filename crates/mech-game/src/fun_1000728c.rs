// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a41e0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000728c(param_1: i32) -> u32 {
    mech_trace::enter!("1000728c", param_1);
    if !(0..=4).contains(&param_1) {
        return mech_trace::rt!("1000728c", 2);
    }
    // SAFETY: the global is the game's own memory.
    unsafe {
        mech_trace::gw!("1000728c", "100a41e0", DAT_100a41e0, param_1);
    }
    mech_trace::rt!("1000728c", 0)
}
