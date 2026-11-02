// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3a1c: u8;
    static mut DAT_100b3a18: u32;
    static mut DAT_100b3a14: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006a338() {
    mech_trace::enter!("1006a338");
    // SAFETY: the globals are the game's own memory; written in the original's order.
    unsafe {
        mech_trace::gw!("1006a338", "100b3a1c", DAT_100b3a1c, 0);
        mech_trace::gw!("1006a338", "100b3a18", DAT_100b3a18, 3);
        mech_trace::gw!("1006a338", "100b3a14", DAT_100b3a14, 3);
    }
}
