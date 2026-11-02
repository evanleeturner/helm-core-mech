// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ae750: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002cd5d(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1008113f() -> u32 {
    // SAFETY: the callee and the global are the game's own.
    unsafe {
        if FUN_1002cd5d(0x800) == 0 && DAT_100ae750 == 0 {
            return 0;
        }
    }
    1
}
