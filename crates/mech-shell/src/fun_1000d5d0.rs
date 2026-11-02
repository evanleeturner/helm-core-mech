// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a790(param_1: i32) -> u8;
    fn FUN_1000a800(param_1: i32) -> u8;
    fn FUN_1000a8c0(param_1: i32) -> u8;
    fn FUN_1000d880(param_1: i32) -> u8;
    fn FUN_1000aa80(param_1: i32) -> u8;
    fn FUN_1000a890(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d5d0(param_1: i32) {
    if param_1 != 0 {
        // SAFETY: the callees are the game's own functions, called in the
        // original order with the same argument.
        unsafe {
            FUN_1000a790(param_1);
            FUN_1000a800(param_1);
            FUN_1000a8c0(param_1);
            FUN_1000d880(param_1);
            FUN_1000aa80(param_1);
            FUN_1000a890(param_1);
        }
    }
}
