// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098ba8: u32;
    static mut DAT_10098bac: u32;
    static mut DAT_10098bb0: u32;
    static mut DAT_10098bb4: u32;
    static mut DAT_10098bb8: u32;
    static mut DAT_10098bbc: u32;
    static mut DAT_10098bc0: u32;
    static mut DAT_10098bd8: u32;
    static mut DAT_10098be8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10043110() {
    mech_trace::enter!("10043110");
    // SAFETY: the globals are the game's own memory, written in the original's order.
    unsafe {
        mech_trace::gw!("10043110", "10098ba8", DAT_10098ba8, 0);
        mech_trace::gw!("10043110", "10098bac", DAT_10098bac, 0);
        mech_trace::gw!("10043110", "10098bb0", DAT_10098bb0, 0);
        mech_trace::gw!("10043110", "10098bb4", DAT_10098bb4, 0);
        mech_trace::gw!("10043110", "10098bb8", DAT_10098bb8, 0);
        mech_trace::gw!("10043110", "10098bbc", DAT_10098bbc, 0);
        mech_trace::gw!("10043110", "10098bc0", DAT_10098bc0, 0);
        mech_trace::gw!("10043110", "10098be8", DAT_10098be8, 0);
        mech_trace::gw!("10043110", "10098bd8", DAT_10098bd8, 0xffffffff);
    }
}
