// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10215654: i32;
    static mut DAT_1021565c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000a942() {
    mech_trace::enter!("1000a942");
    // SAFETY: the globals are the game's own memory; written in the original's order.
    unsafe {
        mech_trace::gw!("1000a942", "10215654", DAT_10215654, 0);
        mech_trace::gw!("1000a942", "1021565c", DAT_1021565c, 0);
    }
}
