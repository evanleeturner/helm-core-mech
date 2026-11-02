// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10090198: u32;
    static mut DAT_1009019c: u32;
    static mut DAT_100901a0: u32;
    static mut DAT_100901a4: u32;
    static mut DAT_100901a8: u32;
    static mut DAT_100901ac: u32;
    static mut DAT_100b5124: u32;
    static mut DAT_100b5114: u32;
    static mut DAT_100b5110: u32;
    static mut DAT_100b5100: u32;
    static mut DAT_100b50f8: u32;
    static mut DAT_100b5138: u32;
    static mut DAT_100b50f4: u32;
    static mut DAT_100b5128: u32;
    static mut DAT_100b5118: u32;
    static mut DAT_100b5108: u32;
    static mut DAT_100b50fc: u32;
    static mut DAT_100b5134: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001db00() {
    mech_trace::enter!("1001db00");
    // SAFETY: the globals are the game's own memory; each is written with zero
    // in the original's order.
    unsafe {
        mech_trace::gw!("1001db00", "10090198", DAT_10090198, 0);
        mech_trace::gw!("1001db00", "1009019c", DAT_1009019c, 0);
        mech_trace::gw!("1001db00", "100901a0", DAT_100901a0, 0);
        mech_trace::gw!("1001db00", "100901a4", DAT_100901a4, 0);
        mech_trace::gw!("1001db00", "100901a8", DAT_100901a8, 0);
        mech_trace::gw!("1001db00", "100901ac", DAT_100901ac, 0);
        mech_trace::gw!("1001db00", "100b5124", DAT_100b5124, 0);
        mech_trace::gw!("1001db00", "100b5114", DAT_100b5114, 0);
        mech_trace::gw!("1001db00", "100b5110", DAT_100b5110, 0);
        mech_trace::gw!("1001db00", "100b5100", DAT_100b5100, 0);
        mech_trace::gw!("1001db00", "100b50f8", DAT_100b50f8, 0);
        mech_trace::gw!("1001db00", "100b5138", DAT_100b5138, 0);
        mech_trace::gw!("1001db00", "100b50f4", DAT_100b50f4, 0);
        mech_trace::gw!("1001db00", "100b5128", DAT_100b5128, 0);
        mech_trace::gw!("1001db00", "100b5118", DAT_100b5118, 0);
        mech_trace::gw!("1001db00", "100b5108", DAT_100b5108, 0);
        mech_trace::gw!("1001db00", "100b50fc", DAT_100b50fc, 0);
        mech_trace::gw!("1001db00", "100b5134", DAT_100b5134, 0);
    }
}
