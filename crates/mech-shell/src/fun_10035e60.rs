// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093a24: i32;
    static mut DAT_10093914: i32;
    static mut DAT_1009390c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10035e60(param_1: i32) -> u32 {
    mech_trace::enter!("10035e60", param_1);
    // SAFETY: the globals are the game's own memory, read by value.
    unsafe {
        if param_1 != 0 {
            return mech_trace::rt!(
                "10035e60",
                mech_trace::gr!("10035e60", "10093a24", DAT_10093a24) as u32
            );
        }
        if mech_trace::gr!("10035e60", "10093914", DAT_10093914) < 0 {
            return mech_trace::rt!("10035e60", 0xffffffff);
        }
        mech_trace::rt!(
            "10035e60",
            mech_trace::gr!("10035e60", "1009390c", DAT_1009390c) as u32
        )
    }
}
