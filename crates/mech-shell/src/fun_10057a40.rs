// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a46f8: i32;
    static mut DAT_100a46fc: i32;
    static mut DAT_100a4700: i32;
    static mut DAT_100a4704: i32;
    static mut DAT_100a4708: i32;
    static mut DAT_100a470c: i32;
    static mut DAT_100a4710: i32;
    static mut DAT_100a4714: i32;
    static mut DAT_100a4718: i32;
    static mut DAT_100a471c: u8;
    static mut DAT_100a4720: i32;
    static mut DAT_100a4724: i32;
    static mut DAT_100a4728: i32;
    static mut DAT_100a472c: i32;
    static mut DAT_100a4730: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10057a40() {
    mech_trace::enter!("10057a40");
    // SAFETY: the globals are the game's own memory; each is written once, in the original's order.
    unsafe {
        mech_trace::gw!("10057a40", "100a46f8", DAT_100a46f8, 0);
        mech_trace::gw!("10057a40", "100a46fc", DAT_100a46fc, 0);
        mech_trace::gw!("10057a40", "100a4700", DAT_100a4700, 0);
        mech_trace::gw!("10057a40", "100a4704", DAT_100a4704, 0);
        mech_trace::gw!("10057a40", "100a4708", DAT_100a4708, 0);
        mech_trace::gw!("10057a40", "100a470c", DAT_100a470c, 0);
        mech_trace::gw!("10057a40", "100a4710", DAT_100a4710, 0);
        mech_trace::gw!("10057a40", "100a4714", DAT_100a4714, 0);
        mech_trace::gw!("10057a40", "100a4718", DAT_100a4718, 0);
        mech_trace::gw!("10057a40", "100a471c", DAT_100a471c, 0);
        mech_trace::gw!("10057a40", "100a4720", DAT_100a4720, 0);
        mech_trace::gw!("10057a40", "100a4724", DAT_100a4724, 0);
        mech_trace::gw!("10057a40", "100a4728", DAT_100a4728, 0);
        mech_trace::gw!("10057a40", "100a472c", DAT_100a472c, 0);
        mech_trace::gw!("10057a40", "100a4730", DAT_100a4730, 0);
    }
}
