// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008edf4: u32;
    static mut DAT_1008edf8: u32;
    static mut DAT_1008ee00: u32;
    static mut DAT_100b0f40: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10016ba0(param_1: u32, param_2: u32) {
    mech_trace::enter!("10016ba0", param_1, param_2);
    // SAFETY: the globals are the game's own memory; each is read or
    // written exactly where the original does.
    unsafe {
        mech_trace::gw!(
            "10016ba0",
            "1008edf8",
            DAT_1008edf8,
            mech_trace::gr!("10016ba0", "1008edf4", DAT_1008edf4)
        );
        mech_trace::gw!("10016ba0", "1008ee00", DAT_1008ee00, param_1);
        mech_trace::gw!("10016ba0", "100b0f40", DAT_100b0f40, param_2);
        mech_trace::gw!("10016ba0", "1008edf4", DAT_1008edf4, 0);
    }
}
