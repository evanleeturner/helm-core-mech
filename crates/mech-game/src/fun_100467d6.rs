// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04b8: u32;
    static mut DAT_100b04c4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10046a29(param_1: *mut u32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100467d6() {
    // SAFETY: the globals and the callee are the game's own; the address of
    // DAT_100b04b8 is passed without creating a reference.
    unsafe {
        if DAT_100b04c4 != 0 {
            FUN_10046a29(&raw mut DAT_100b04b8, -1);
        }
    }
}
