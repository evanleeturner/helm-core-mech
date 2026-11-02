// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7888: i32;
    static mut _DAT_100a7920: i32;
    static mut _DAT_100a7958: i32;
    static mut _DAT_100a7990: i32;
    static mut _DAT_100a79c8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063de0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10060b90() {
    mech_trace::enter!("10060b90");
    // SAFETY: the globals and the callee are the game's own; the address of
    // DAT_100a7888 is passed on without creating a reference.
    unsafe {
        mech_trace::gw!("10060b90", "100a7920", _DAT_100a7920, 1);
        mech_trace::gw!("10060b90", "100a7958", _DAT_100a7958, 1);
        mech_trace::gw!("10060b90", "100a7990", _DAT_100a7990, 1);
        mech_trace::gw!("10060b90", "100a79c8", _DAT_100a79c8, 1);
        FUN_10063de0(&raw mut DAT_100a7888);
    }
}
