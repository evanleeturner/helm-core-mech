// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10215650: i32;
    static mut DAT_10215658: i32;
    static mut DAT_10215660: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10009c1b(param_1: i32) {
    mech_trace::enter!("10009c1b", param_1);
    // SAFETY: the globals are the game's own memory, read and written where
    // the original does.
    unsafe {
        if param_1 != mech_trace::gr!("10009c1b", "10215658", DAT_10215658) {
            if mech_trace::gr!("10009c1b", "10215658", DAT_10215658) < 3 && param_1 < 3 {
                mech_trace::gw!("10009c1b", "10215660", DAT_10215660, param_1);
            } else if mech_trace::gr!("10009c1b", "10215658", DAT_10215658) < 3 {
                mech_trace::gw!("10009c1b", "10215660", DAT_10215660, 3);
            } else {
                mech_trace::gw!("10009c1b", "10215660", DAT_10215660, 5);
                mech_trace::gw!("10009c1b", "10215650", DAT_10215650, param_1);
            }
        }
    }
}
