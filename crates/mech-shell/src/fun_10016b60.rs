// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008edf4: u32;
    static mut DAT_1008ee2c: u32;
    static mut DAT_1008f110: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100640a0(screen_function_likely: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10016b60(param_1: i32) {
    mech_trace::enter!("10016b60", param_1);
    // SAFETY: the globals and the callee are the game's own memory and code;
    // the string is the game's array at its address.
    unsafe {
        let stashed = mech_trace::gr!("10016b60", "1008ee2c", DAT_1008ee2c);
        mech_trace::gw!("10016b60", "1008edf4", DAT_1008edf4, param_1 as u32);
        if param_1 == 0 {
            mech_trace::cr!(
                "10016b60",
                "100640a0",
                FUN_100640a0(&raw mut DAT_1008f110 as *mut i8)
            );
            return;
        }
        if mech_trace::gr!("10016b60", "1008ee2c", DAT_1008ee2c) != 0 {
            mech_trace::gw!("10016b60", "1008ee2c", DAT_1008ee2c, 0);
            mech_trace::gw!("10016b60", "1008edf4", DAT_1008edf4, stashed);
            return;
        }
        mech_trace::gw!("10016b60", "1008edf4", DAT_1008edf4, 0);
    }
}
