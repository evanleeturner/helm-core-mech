// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af0a0: u32;
    static mut DAT_100af0a4: u32;
    static mut DAT_100c7b70: u32;
    static mut DAT_100c7b88: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003e768(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003e728() {
    mech_trace::enter!("1003e728");
    // SAFETY: the globals and the callee are the game's own memory and code.
    unsafe {
        mech_trace::gw!("1003e728", "100c7b70", DAT_100c7b70, 0);
        mech_trace::gw!("1003e728", "100c7b88", DAT_100c7b88, 0);
        if 0 < mech_trace::gr!("1003e728", "100af0a0", DAT_100af0a0) {
            mech_trace::gw!("1003e728", "100af0a4", DAT_100af0a4, 1);
            mech_trace::cr!("1003e728", "1003e768", FUN_1003e768(0));
        }
    }
}
