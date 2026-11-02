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
    // SAFETY: the globals and the callee are the game's own; they are
    // written and called in the original's order.
    unsafe {
        _DAT_100adcf8 = 0;
        FUN_1004d01b(DAT_100b0710, param_1);
    }
}
