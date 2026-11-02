// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7888: i32;
    static mut _DAT_100a7920: i32;
    static mut _DAT_100a7958: i32;
    static mut _DAT_100a7990: i32;
    static mut _DAT_100a79c8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063de0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10062400() {
    // SAFETY: the globals and the callee are the game's own; the address of
    // DAT_100a7888 is passed on without creating a reference.
    unsafe {
        _DAT_100a7920 = 0;
        _DAT_100a7958 = 0;
        _DAT_100a7990 = 0;
        _DAT_100a79c8 = 0;
        FUN_10063de0(&raw mut DAT_100a7888);
    }
}
