// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04b8: u8;
    static mut DAT_100b04c4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10046a29(param_1: *mut i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100467fd() {
    // SAFETY: the globals and the callee are the game's own; the address of
    // DAT_100b04b8 is passed on without creating a reference.
    unsafe {
        if DAT_100b04c4 != 0 {
            FUN_10046a29((&raw mut DAT_100b04b8).cast::<i32>(), 1);
        }
    }
}
