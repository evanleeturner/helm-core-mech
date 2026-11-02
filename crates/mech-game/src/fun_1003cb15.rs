// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003cb15() {
    mech_trace::enter!("1003cb15");
    // SAFETY: the global is the game's own memory.
    unsafe {
        mech_trace::gw!("1003cb15", "100aefc0", DAT_100aefc0, 0);
    }
}
