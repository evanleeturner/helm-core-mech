// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008eaa8: u32;
    static mut DAT_1008eaac: u32;
    static mut DAT_1008eab0: u32;
    static mut DAT_1008eab4: u32;
    static mut DAT_1008eab8: u32;
    static mut DAT_1008eabc: u32;
    static mut DAT_1008eac0: u32;
    static mut DAT_1008eac4: u32;
    static mut DAT_1008eac8: u32;
    static mut DAT_1008eacc: u32;
    static mut DAT_1008ead0: u32;
    static mut DAT_1008ead4: u32;
    static mut DAT_1008ead8: u32;
    static mut DAT_1008eadc: u32;
    static mut DAT_1008eae0: u32;
    static mut DAT_1008eae8: u32;
    static mut DAT_100b0b34: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100116b0() {
    mech_trace::enter!("100116b0");
    // SAFETY: the globals are the game's own memory; each is written zero
    // in the original's order.
    unsafe {
        mech_trace::gw!("100116b0", "1008eaa8", DAT_1008eaa8, 0);
        mech_trace::gw!("100116b0", "1008eaac", DAT_1008eaac, 0);
        mech_trace::gw!("100116b0", "1008eab0", DAT_1008eab0, 0);
        mech_trace::gw!("100116b0", "1008eab8", DAT_1008eab8, 0);
        mech_trace::gw!("100116b0", "1008eabc", DAT_1008eabc, 0);
        mech_trace::gw!("100116b0", "1008eac0", DAT_1008eac0, 0);
        mech_trace::gw!("100116b0", "1008eac4", DAT_1008eac4, 0);
        mech_trace::gw!("100116b0", "1008eac8", DAT_1008eac8, 0);
        mech_trace::gw!("100116b0", "1008eacc", DAT_1008eacc, 0);
        mech_trace::gw!("100116b0", "1008ead0", DAT_1008ead0, 0);
        mech_trace::gw!("100116b0", "1008ead4", DAT_1008ead4, 0);
        mech_trace::gw!("100116b0", "1008ead8", DAT_1008ead8, 0);
        mech_trace::gw!("100116b0", "1008eadc", DAT_1008eadc, 0);
        mech_trace::gw!("100116b0", "1008eae0", DAT_1008eae0, 0);
        mech_trace::gw!("100116b0", "100b0b34", DAT_100b0b34, 0);
        mech_trace::gw!("100116b0", "1008eae8", DAT_1008eae8, 0);
        mech_trace::gw!("100116b0", "1008eab4", DAT_1008eab4, 0);
    }
}
