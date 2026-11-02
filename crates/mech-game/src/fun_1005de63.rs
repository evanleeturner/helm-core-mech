// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab238: u32;
    static mut DAT_100ab23c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001d916(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005de63() -> u32 {
    mech_trace::enter!("1005de63");
    // SAFETY: the globals and the callee are the game's own; they are read,
    // written and called in the original's order.
    unsafe {
        mech_trace::gw!(
            "1005de63",
            "100ab23c",
            DAT_100ab23c,
            mech_trace::gr!("1005de63", "100ab238", DAT_100ab238)
        );
        mech_trace::cr!(
            "1005de63",
            "1001d916",
            FUN_1001d916(mech_trace::gr!("1005de63", "100ab238", DAT_100ab238))
        );
    }
    mech_trace::rt!("1005de63", 0)
}
