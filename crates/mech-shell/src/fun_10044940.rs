// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_10099050: i32;
    static mut _DAT_10099088: i32;
    static mut _DAT_100990c0: i32;
    static mut _DAT_100990f8: i32;
    static mut DAT_10098fb8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063de0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10044940() {
    // SAFETY: the globals and the callee are the game's own memory and code;
    // the writes and the call happen in the original's order.
    unsafe {
        _DAT_10099050 = 1;
        _DAT_10099088 = 1;
        _DAT_100990c0 = 1;
        _DAT_100990f8 = 1;
        FUN_10063de0(&raw mut DAT_10098fb8);
    }
}
