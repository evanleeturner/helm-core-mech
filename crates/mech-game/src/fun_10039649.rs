// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ae750: u32;
    static mut DAT_100ae758: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f783(param_1: i32) -> u8;
    fn FUN_1002f727(param_1: i32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10039649(param_1: i32) {
    mech_trace::enter!("10039649", param_1);
    // SAFETY: the globals and callees are the game's own; the pointer in
    // DAT_100ae758 is checked for null before use.
    unsafe {
        if !mech_trace::gr!("10039649", "100ae758", DAT_100ae758).is_null() {
            if param_1 == 0 {
                mech_trace::cr!(
                    "10039649",
                    "1002f727",
                    FUN_1002f727(mech_trace::gr!("10039649", "100ae758", DAT_100ae758) as i32)
                );
            } else {
                mech_trace::cr!(
                    "10039649",
                    "1002f783",
                    FUN_1002f783(mech_trace::gr!("10039649", "100ae758", DAT_100ae758) as i32)
                );
            }
            mech_trace::cr!(
                "10039649",
                "1002fbc0",
                FUN_1002fbc0(mech_trace::gr!("10039649", "100ae758", DAT_100ae758))
            );
            mech_trace::gw!("10039649", "100ae750", DAT_100ae750, param_1 as u32);
        }
    }
}
