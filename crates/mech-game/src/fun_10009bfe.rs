// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10215658: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10009c90();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10009bfe() {
    mech_trace::enter!("10009bfe");
    // SAFETY: the global and the callee are the game's own memory and code.
    unsafe {
        if mech_trace::gr!("10009bfe", "10215658", DAT_10215658) == 4 {
            FUN_10009c90();
        }
    }
}
