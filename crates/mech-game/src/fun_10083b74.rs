// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008276d(param_1: i32, param_2: i32, param_3: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10083b74(param_1: i32, param_2: i32, param_3: i32) {
    let mut local_8: u32 = 1;
    if param_3 != 0 {
        local_8 = 0x101;
    }
    // SAFETY: the callee is the game's own function; arguments are passed by value.
    unsafe {
        FUN_1008276d(param_1, param_2, local_8);
    }
}
