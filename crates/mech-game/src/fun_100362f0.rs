// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0710: u32;
    static mut _DAT_100adcf8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d01b(param_1: u32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100362f0(param_1: i32) {
    mech_trace::enter!("100362f0", param_1);
    // SAFETY: the globals and the callee are the game's own; they are
    // written and called in the original's order.
    unsafe {
        mech_trace::gw!("100362f0", "100adcf8", _DAT_100adcf8, 0);
        mech_trace::cr!(
            "100362f0",
            "1004d01b",
            FUN_1004d01b(
                mech_trace::gr!("100362f0", "100b0710", DAT_100b0710),
                param_1
            )
        );
    }
}
