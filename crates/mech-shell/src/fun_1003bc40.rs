// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10094e50: u32;
    static mut DAT_10094e54: u32;
    static mut DAT_10094e58: u32;
    static mut DAT_10094e5c: u32;
    static mut DAT_10094e60: u32;
    static mut DAT_100b9d50: u32;
    static mut DAT_100b9d54: u8;
    static mut DAT_100b9d58: u32;
    static mut DAT_100b9d5c: u32;
    static mut DAT_100b9d64: u32;
    static mut DAT_100b9d68: u32;
    static mut DAT_100b9d6c: u32;
    static mut DAT_100b9d70: u32;
    static mut DAT_100b9d74: u32;
    static mut DAT_100b9d78: u32;
    static mut DAT_100b9d80: u32;
    static mut DAT_100b9d84: u32;
    static mut DAT_100b9d88: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003bc40() {
    mech_trace::enter!("1003bc40");
    // SAFETY: the globals are the game's own memory; written in the original order.
    unsafe {
        mech_trace::gw!("1003bc40", "10094e50", DAT_10094e50, 0);
        mech_trace::gw!("1003bc40", "10094e54", DAT_10094e54, 0);
        mech_trace::gw!("1003bc40", "10094e58", DAT_10094e58, 0);
        mech_trace::gw!("1003bc40", "10094e5c", DAT_10094e5c, 0);
        mech_trace::gw!("1003bc40", "100b9d6c", DAT_100b9d6c, 0);
        mech_trace::gw!("1003bc40", "10094e60", DAT_10094e60, 0);
        mech_trace::gw!("1003bc40", "100b9d68", DAT_100b9d68, 0);
        mech_trace::gw!("1003bc40", "100b9d58", DAT_100b9d58, 0);
        mech_trace::gw!("1003bc40", "100b9d50", DAT_100b9d50, 0);
        mech_trace::gw!("1003bc40", "100b9d80", DAT_100b9d80, 0);
        mech_trace::gw!("1003bc40", "100b9d5c", DAT_100b9d5c, 0);
        mech_trace::gw!("1003bc40", "100b9d64", DAT_100b9d64, 0);
        mech_trace::gw!("1003bc40", "100b9d88", DAT_100b9d88, 0);
        mech_trace::gw!("1003bc40", "100b9d78", DAT_100b9d78, 0);
        mech_trace::gw!("1003bc40", "100b9d70", DAT_100b9d70, 0);
        mech_trace::gw!("1003bc40", "100b9d74", DAT_100b9d74, 0);
        mech_trace::gw!("1003bc40", "100b9d84", DAT_100b9d84, 0);
        mech_trace::gw!("1003bc40", "100b9d54", DAT_100b9d54, 0);
    }
}
