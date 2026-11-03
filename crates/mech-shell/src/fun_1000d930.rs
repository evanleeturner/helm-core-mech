// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008d104: u32;
    static mut _DAT_1008d108: u32;
    static mut DAT_100b055c: u32;
    static mut DAT_100b0558: u32;
    static mut DAT_100b0560: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006fab0(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000d930() -> u32 {
    mech_trace::enter!("1000d930");
    // SAFETY: the callee and the globals are the game's own; the globals
    // are read and written where the original does.
    unsafe {
        let mut u_var1: u32 = 1;
        mech_trace::gw!(
            "1000d930",
            "1008d104",
            DAT_1008d104,
            mech_trace::cr!("1000d930", "1006fab0", FUN_1006fab0(32000))
        );
        mech_trace::gw!(
            "1000d930",
            "1008d108",
            _DAT_1008d108,
            mech_trace::gr!("1000d930", "1008d104", DAT_1008d104)
        );
        mech_trace::gw!(
            "1000d930",
            "100b055c",
            DAT_100b055c,
            mech_trace::cr!("1000d930", "1006fab0", FUN_1006fab0(4000))
        );
        mech_trace::gw!(
            "1000d930",
            "100b0558",
            DAT_100b0558,
            mech_trace::cr!("1000d930", "1006fab0", FUN_1006fab0(4000))
        );
        mech_trace::gw!(
            "1000d930",
            "100b0560",
            DAT_100b0560,
            mech_trace::cr!("1000d930", "1006fab0", FUN_1006fab0(4000))
        );
        if mech_trace::gr!("1000d930", "1008d104", DAT_1008d104) == 0
            || mech_trace::gr!("1000d930", "100b055c", DAT_100b055c) == 0
            || mech_trace::gr!("1000d930", "100b0558", DAT_100b0558) == 0
            || mech_trace::gr!("1000d930", "100b0560", DAT_100b0560) == 0
        {
            u_var1 = 0;
        }
        mech_trace::rt!("1000d930", u_var1)
    }
}
