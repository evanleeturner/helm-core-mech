// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10215658: i32;
    static mut DAT_10215660: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10009c90() {
    mech_trace::enter!("10009c90");
    // SAFETY: the globals are the game's own memory, read and written
    // where the original does.
    unsafe {
        if mech_trace::gr!("10009c90", "10215658", DAT_10215658) == 4
            && mech_trace::gr!("10009c90", "10215660", DAT_10215660) == 4
        {
            mech_trace::gw!("10009c90", "10215660", DAT_10215660, 5);
        } else if mech_trace::gr!("10009c90", "10215658", DAT_10215658) < 3
            && mech_trace::gr!("10009c90", "10215660", DAT_10215660) < 3
        {
            mech_trace::gw!("10009c90", "10215660", DAT_10215660, 3);
        }
    }
}
