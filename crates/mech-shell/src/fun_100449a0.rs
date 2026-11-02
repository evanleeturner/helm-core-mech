// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10099140: u32;
    static mut DAT_10099144: u32;
    static mut DAT_10099148: u32;
    static mut DAT_10099150: u32;
    static mut DAT_10099154: u32;
    static mut DAT_10099158: u32;
    static mut DAT_1009915c: u32;
    static mut DAT_10099160: u32;
    static mut DAT_10099164: u32;
    static mut DAT_10099168: u32;
    static mut DAT_1009916c: u32;
    static mut DAT_10099170: u32;
    static mut DAT_10099174: u32;
    static mut DAT_100ba424: u32;
    static mut DAT_10098f78: u32;
    static mut DAT_10098f7c: u32;
    static mut DAT_10098f80: u32;
    static mut DAT_10098f84: u32;
    static mut DAT_10098f88: u32;
    static mut DAT_10098f8c: u32;
    static mut DAT_10098f90: u32;
    static mut DAT_10098f94: u32;
    static mut DAT_10098f98: u32;
    static mut DAT_10098f9c: u32;
    static mut DAT_10098fa0: u32;
    static mut DAT_10098fa4: u32;
    static mut DAT_10098fa8: u32;
    static mut DAT_10098fac: u32;
    static mut DAT_10098fb0: u8;
    static mut DAT_10098fb4: u8;
    static mut DAT_100ba420: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100449a0() {
    mech_trace::enter!("100449a0");
    // SAFETY: the globals are the game's own memory; each is written
    // in the same order as the original.
    unsafe {
        mech_trace::gw!("100449a0", "10099140", DAT_10099140, 0);
        mech_trace::gw!("100449a0", "10099144", DAT_10099144, 0);
        mech_trace::gw!("100449a0", "10099148", DAT_10099148, 0);
        mech_trace::gw!("100449a0", "10099150", DAT_10099150, 0);
        mech_trace::gw!("100449a0", "10099154", DAT_10099154, 0);
        mech_trace::gw!("100449a0", "10099158", DAT_10099158, 0);
        mech_trace::gw!("100449a0", "1009915c", DAT_1009915c, 0);
        mech_trace::gw!("100449a0", "10099160", DAT_10099160, 0);
        mech_trace::gw!("100449a0", "10099164", DAT_10099164, 0);
        mech_trace::gw!("100449a0", "10099168", DAT_10099168, 0);
        mech_trace::gw!("100449a0", "1009916c", DAT_1009916c, 0);
        mech_trace::gw!("100449a0", "10099170", DAT_10099170, 0);
        mech_trace::gw!("100449a0", "10099174", DAT_10099174, 0);
        mech_trace::gw!("100449a0", "100ba424", DAT_100ba424, 0);
        mech_trace::gw!("100449a0", "10098f78", DAT_10098f78, 0);
        mech_trace::gw!("100449a0", "10098f7c", DAT_10098f7c, 0);
        mech_trace::gw!("100449a0", "10098f80", DAT_10098f80, 0);
        mech_trace::gw!("100449a0", "10098f84", DAT_10098f84, 0);
        mech_trace::gw!("100449a0", "10098f88", DAT_10098f88, 0);
        mech_trace::gw!("100449a0", "10098f8c", DAT_10098f8c, 0);
        mech_trace::gw!("100449a0", "10098f90", DAT_10098f90, 0);
        mech_trace::gw!("100449a0", "10098f94", DAT_10098f94, 0);
        mech_trace::gw!("100449a0", "10098f98", DAT_10098f98, 0);
        mech_trace::gw!("100449a0", "10098f9c", DAT_10098f9c, 0);
        mech_trace::gw!("100449a0", "10098fa0", DAT_10098fa0, 0);
        mech_trace::gw!("100449a0", "10098fa4", DAT_10098fa4, 0);
        mech_trace::gw!("100449a0", "10098fa8", DAT_10098fa8, 0);
        mech_trace::gw!("100449a0", "10098fac", DAT_10098fac, 0);
        mech_trace::gw!("100449a0", "10098fb0", DAT_10098fb0, 0);
        mech_trace::gw!("100449a0", "10098fb4", DAT_10098fb4, 0);
        mech_trace::gw!("100449a0", "100ba420", DAT_100ba420, 1);
    }
}
