// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a71e0: u32;
    static mut DAT_100a71e4: u32;
    static mut DAT_100a6724: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10010820(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005f8e0(param_1: u32) {
    mech_trace::enter!("1005f8e0", param_1);
    // SAFETY: the globals and the callee are the game's own; accesses happen
    // in the original's order.
    unsafe {
        mech_trace::gw!("1005f8e0", "100a71e0", DAT_100a71e0, 1);
        mech_trace::gw!("1005f8e0", "100a71e4", DAT_100a71e4, param_1);
        FUN_10010820(mech_trace::gr!("1005f8e0", "100a6724", DAT_100a6724));
    }
}
