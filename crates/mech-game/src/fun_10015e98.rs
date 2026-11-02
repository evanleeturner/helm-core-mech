// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10114db8: u32;
    static mut DAT_10114df4: u32;
    static mut DAT_10114e08: u32;
    static mut DAT_10114e18: u32;
    static mut DAT_10114e2c: u32;
    static mut DAT_10114e30: u32;
    static mut DAT_10114e34: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10015cd4(param_1: i32) -> u8;
    fn FUN_10015b50(param_1: i32) -> u8;
    fn FUN_10015bc6(param_1: i32) -> u8;
    fn FUN_10015c4d(param_1: i32) -> u8;
    fn FUN_10015da1(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10015e98() {
    // SAFETY: the callees and globals are the game's own; globals are read
    // by value in the original's order and passed to each callee.
    unsafe {
        FUN_10015cd4(DAT_10114e08 as i32);
        FUN_10015cd4(DAT_10114db8 as i32);
        FUN_10015cd4(DAT_10114df4 as i32);
        FUN_10015cd4(DAT_10114e18 as i32);
        FUN_10015cd4(DAT_10114e2c as i32);
        FUN_10015cd4(DAT_10114e30 as i32);
        FUN_10015cd4(DAT_10114e34 as i32);
        FUN_10015b50(DAT_10114e08 as i32);
        FUN_10015b50(DAT_10114db8 as i32);
        FUN_10015b50(DAT_10114df4 as i32);
        FUN_10015b50(DAT_10114e18 as i32);
        FUN_10015b50(DAT_10114e2c as i32);
        FUN_10015b50(DAT_10114e30 as i32);
        FUN_10015b50(DAT_10114e34 as i32);
        FUN_10015bc6(DAT_10114e08 as i32);
        FUN_10015bc6(DAT_10114db8 as i32);
        FUN_10015bc6(DAT_10114df4 as i32);
        FUN_10015bc6(DAT_10114e18 as i32);
        FUN_10015bc6(DAT_10114e2c as i32);
        FUN_10015bc6(DAT_10114e30 as i32);
        FUN_10015bc6(DAT_10114e34 as i32);
        FUN_10015c4d(DAT_10114e08 as i32);
        FUN_10015c4d(DAT_10114db8 as i32);
        FUN_10015c4d(DAT_10114df4 as i32);
        FUN_10015c4d(DAT_10114e18 as i32);
        FUN_10015c4d(DAT_10114e2c as i32);
        FUN_10015c4d(DAT_10114e30 as i32);
        FUN_10015c4d(DAT_10114e34 as i32);
        FUN_10015da1(DAT_10114e18 as i32);
        FUN_10015da1(DAT_10114db8 as i32);
        FUN_10015da1(DAT_10114df4 as i32);
        FUN_10015da1(DAT_10114e08 as i32);
        FUN_10015da1(DAT_10114e2c as i32);
        FUN_10015da1(DAT_10114e30 as i32);
        FUN_10015da1(DAT_10114e34 as i32);
    }
}
