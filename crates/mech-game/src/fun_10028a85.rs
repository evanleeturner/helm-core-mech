// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aca50: u32;
    static mut DAT_100aca54: u32;
    static mut DAT_100aca58: u32;
    static mut DAT_100aca5c: u32;
    static mut DAT_100aca60: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10027502();
    fn FUN_1002753b();
    fn FUN_10027ec0();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100280a3(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10028a85() {
    mech_trace::enter!("10028a85");
    // SAFETY: the globals and callees are the game's own memory and code,
    // accessed in the same order as the original.
    unsafe {
        if mech_trace::gr!("10028a85", "100aca50", DAT_100aca50)
            != mech_trace::gr!("10028a85", "100aca60", DAT_100aca60)
        {
            mech_trace::gw!(
                "10028a85",
                "100aca60",
                DAT_100aca60,
                mech_trace::gr!("10028a85", "100aca50", DAT_100aca50)
            );
            mech_trace::gw!("10028a85", "100aca58", DAT_100aca58, 0xffffffff);
            mech_trace::gw!("10028a85", "100aca5c", DAT_100aca5c, 0xffffffff);
            mech_trace::gw!("10028a85", "100aca54", DAT_100aca54, 0);
            FUN_10027502();
        }
        if mech_trace::gr!("10028a85", "100aca50", DAT_100aca50) == 0xffffffff {
            FUN_1002753b();
        } else if mech_trace::gr!("10028a85", "100aca50", DAT_100aca50) == 0xfffffffe {
            FUN_10027ec0();
        } else {
            FUN_100280a3(mech_trace::gr!("10028a85", "100aca50", DAT_100aca50) as i32);
        }
    }
}
