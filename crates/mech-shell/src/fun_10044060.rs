// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098db8: u32;
    static mut DAT_10098dbc: u32;
    static mut DAT_10098dc0: u32;
    static mut DAT_10098dc4: u32;
    static mut DAT_10098dcc: u32;
    static mut DAT_10098dd0: u32;
    static mut DAT_10098dd4: u32;
    static mut DAT_10098ddc: u32;
    static mut DAT_10098de8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10044060() {
    mech_trace::enter!("10044060");
    // SAFETY: the globals are the game's own memory; each is written in the original's order.
    unsafe {
        mech_trace::gw!("10044060", "10098db8", DAT_10098db8, 0);
        mech_trace::gw!("10044060", "10098dbc", DAT_10098dbc, 0);
        mech_trace::gw!("10044060", "10098dc0", DAT_10098dc0, 0);
        mech_trace::gw!("10044060", "10098dc4", DAT_10098dc4, 0);
        mech_trace::gw!("10044060", "10098ddc", DAT_10098ddc, 0);
        mech_trace::gw!("10044060", "10098dcc", DAT_10098dcc, 0);
        mech_trace::gw!("10044060", "10098dd0", DAT_10098dd0, 0);
        mech_trace::gw!("10044060", "10098dd4", DAT_10098dd4, 0);
        mech_trace::gw!("10044060", "10098de8", DAT_10098de8, 0);
    }
}
