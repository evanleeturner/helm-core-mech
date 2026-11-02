// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7848: i32;
    static mut DAT_100a7f18: i32;
    static mut DAT_100a7f1c: i32;
    static mut DAT_100a7f20: i32;
    static mut DAT_100a7f24: i32;
    static mut DAT_100a7f28: i32;
    static mut DAT_100a7f2c: i32;
    static mut DAT_100a7f30: i32;
    static mut DAT_100a7f34: i32;
    static mut DAT_100a7f38: u8;
    static mut DAT_100a7f3c: i32;
    static mut DAT_100a7f40: i32;
    static mut DAT_100a7f44: i32;
    static mut DAT_100a7f48: i32;
    static mut DAT_100a7f4c: i32;
    static mut DAT_100a7f50: i32;
    static mut DAT_100a784c: i32;
    static mut DAT_100a7850: i32;
    static mut DAT_100a7854: i32;
    static mut DAT_100c4ae8: i32;
    static mut DAT_100c4adc: i32;
    static mut DAT_100c4af4: i32;
    static mut DAT_100a7858: i32;
    static mut DAT_100a785c: i32;
    static mut DAT_100a7860: i32;
    static mut DAT_100c4ad8: i32;
    static mut DAT_100a7864: i32;
    static mut DAT_100a7868: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10060bc0() {
    mech_trace::enter!("10060bc0");
    // SAFETY: the globals are the game's own memory; each is written once
    // in the original's order.
    unsafe {
        mech_trace::gw!("10060bc0", "100a7848", DAT_100a7848, 0);
        mech_trace::gw!("10060bc0", "100a7f18", DAT_100a7f18, 0);
        mech_trace::gw!("10060bc0", "100a7f1c", DAT_100a7f1c, 0);
        mech_trace::gw!("10060bc0", "100a7f20", DAT_100a7f20, 0);
        mech_trace::gw!("10060bc0", "100a7f24", DAT_100a7f24, 0);
        mech_trace::gw!("10060bc0", "100a7f28", DAT_100a7f28, 0);
        mech_trace::gw!("10060bc0", "100a7f2c", DAT_100a7f2c, 0);
        mech_trace::gw!("10060bc0", "100a7f30", DAT_100a7f30, 0);
        mech_trace::gw!("10060bc0", "100a7f34", DAT_100a7f34, 0);
        mech_trace::gw!("10060bc0", "100a7f38", DAT_100a7f38, 0);
        mech_trace::gw!("10060bc0", "100a7f3c", DAT_100a7f3c, 0);
        mech_trace::gw!("10060bc0", "100a7f40", DAT_100a7f40, 0);
        mech_trace::gw!("10060bc0", "100a7f44", DAT_100a7f44, 0);
        mech_trace::gw!("10060bc0", "100a7f48", DAT_100a7f48, 0);
        mech_trace::gw!("10060bc0", "100a7f4c", DAT_100a7f4c, 0);
        mech_trace::gw!("10060bc0", "100a7f50", DAT_100a7f50, 0);
        mech_trace::gw!("10060bc0", "100a784c", DAT_100a784c, 0);
        mech_trace::gw!("10060bc0", "100a7850", DAT_100a7850, 0);
        mech_trace::gw!("10060bc0", "100a7854", DAT_100a7854, 0);
        mech_trace::gw!("10060bc0", "100c4ae8", DAT_100c4ae8, 0);
        mech_trace::gw!("10060bc0", "100c4adc", DAT_100c4adc, 0);
        mech_trace::gw!("10060bc0", "100c4af4", DAT_100c4af4, 0);
        mech_trace::gw!("10060bc0", "100a7858", DAT_100a7858, 0);
        mech_trace::gw!("10060bc0", "100a785c", DAT_100a785c, 0);
        mech_trace::gw!("10060bc0", "100a7860", DAT_100a7860, 0);
        mech_trace::gw!("10060bc0", "100c4ad8", DAT_100c4ad8, 0);
        mech_trace::gw!("10060bc0", "100a7864", DAT_100a7864, 0);
        mech_trace::gw!("10060bc0", "100a7868", DAT_100a7868, 0);
    }
}
