// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100988d0: u32;
    static mut DAT_100988d4: u32;
    static mut DAT_100988d8: u32;
    static mut DAT_100988dc: u32;
    static mut DAT_100988e0: u32;
    static mut DAT_100988e4: u32;
    static mut DAT_100988e8: u32;
    static mut DAT_100988ec: u32;
    static mut DAT_100ba218: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100428e0() {
    mech_trace::enter!("100428e0");
    // SAFETY: the globals are the game's own memory; they are written in the
    // same order as the original.
    unsafe {
        mech_trace::gw!("100428e0", "100988d4", DAT_100988d4, 0xffffffff);
        mech_trace::gw!("100428e0", "100988d0", DAT_100988d0, 0);
        mech_trace::gw!("100428e0", "100988d8", DAT_100988d8, 0);
        mech_trace::gw!("100428e0", "100988dc", DAT_100988dc, 0);
        mech_trace::gw!("100428e0", "100988e0", DAT_100988e0, 0);
        mech_trace::gw!("100428e0", "100988e4", DAT_100988e4, 0);
        mech_trace::gw!("100428e0", "100988e8", DAT_100988e8, 0);
        mech_trace::gw!("100428e0", "100988ec", DAT_100988ec, 0);
        mech_trace::gw!("100428e0", "100ba218", DAT_100ba218, 0);
    }
}
