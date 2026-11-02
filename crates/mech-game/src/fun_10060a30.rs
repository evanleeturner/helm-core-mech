// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b36dc: u32;
    static mut DAT_100b36e0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10060a30(param_1: u32) {
    mech_trace::enter!("10060a30", param_1);
    // SAFETY: the globals are the game's own memory; written in the original's order.
    unsafe {
        mech_trace::gw!("10060a30", "100b36dc", DAT_100b36dc, param_1);
        mech_trace::gw!("10060a30", "100b36e0", DAT_100b36e0, param_1);
    }
}
