// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004890b(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1007dd05() {
    // SAFETY: the callee is the game's own function; called in the original order.
    unsafe {
        FUN_1004890b(6);
        FUN_1004890b(2);
        FUN_1004890b(4);
        FUN_1004890b(7);
        FUN_1004890b(8);
        FUN_1004890b(5);
    }
}
