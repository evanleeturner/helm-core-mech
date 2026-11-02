// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000d7f0(param_1: i32);
    fn FUN_10030090();
    fn FUN_100304e0(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100304b0(param_1: i32) {
    // SAFETY: the callees are the game's own; param_1 is an address of the
    // caller's object, and the byte at offset 0x84c is written as the original does.
    unsafe {
        FUN_1000d7f0(param_1);
        FUN_10030090();
        ((param_1.wrapping_add(0x84c)) as *mut u8).write(1);
        FUN_100304e0(0);
    }
}
