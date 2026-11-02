// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10083808() -> i32;
    fn FUN_1002f28d(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100837cb() -> u32 {
    // SAFETY: the callees are the game's own functions; the value from the
    // first is passed to the second as the original does.
    unsafe {
        let i_var1 = FUN_10083808();
        if i_var1 == 0 { 0 } else { FUN_1002f28d(i_var1) }
    }
}
