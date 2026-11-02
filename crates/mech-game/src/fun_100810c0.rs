// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002cd5d(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100810c0() -> u32 {
    // SAFETY: the callee is the game's own function; it takes a plain integer.
    unsafe {
        if FUN_1002cd5d(0x100) == 0 && FUN_1002cd5d(0x200) == 0 {
            return 0;
        }
    }
    1
}
