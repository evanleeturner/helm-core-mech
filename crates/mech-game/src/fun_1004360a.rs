// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: i16) -> bool;
    fn FUN_100297b4(param_1: i32) -> i32;
    fn FUN_10022b70(param_1: i32, param_2: i32) -> i32;
    fn FUN_10044a83(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004360a(param_1: i32, param_2: i16) -> u32 {
    // SAFETY: the callees are the game's own; param_1 is an address of the
    // game's object whose fields are read and written where the original does.
    unsafe {
        FUN_1002950f(param_1, param_2);
        let i_var2 = FUN_100297b4(param_1);
        FUN_10022b70(param_1, i_var2);
        let p20 = (param_1.wrapping_add(0x20) as *const i32).read_unaligned();
        let u_var1 = (p20.wrapping_add(0x3d8) as *const u32).read_unaligned();
        let p4c = (param_1.wrapping_add(0x4c) as *const i32).read_unaligned();
        (p4c.wrapping_add(8) as *mut u32).write_unaligned(0x400);
        FUN_10044a83(param_1);
        u_var1
    }
}
