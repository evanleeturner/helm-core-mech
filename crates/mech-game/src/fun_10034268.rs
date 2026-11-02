// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006b20(param_1: i32) -> i32;
    fn FUN_10006b9a(param_1: i32) -> u8;
    fn FUN_10015b30(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10034268(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) {
    // SAFETY: the callees are the game's own; param_1 and param_2 point at
    // the caller's i32 values, written and read where the original does.
    unsafe {
        if param_5 == 0 {
            param_1.write_unaligned(param_3);
            param_2.write_unaligned(param_4);
        } else {
            let i_var1 = FUN_10006b20(param_5.wrapping_shl(0x10));
            let i_var2 = i32::from(FUN_10006b9a(param_5.wrapping_shl(0x10)));
            let i_var3 = FUN_10015b30(param_3.wrapping_shl(0x10), i_var2);
            let i_var4 = FUN_10015b30(param_4.wrapping_shl(0x10), i_var1);
            param_1.write_unaligned(i_var3.wrapping_sub(i_var4));
            let i_var1 = FUN_10015b30(param_3.wrapping_shl(0x10), i_var1);
            let i_var2 = FUN_10015b30(param_4.wrapping_shl(0x10), i_var2);
            param_2.write_unaligned(i_var1.wrapping_add(i_var2));
            param_1.write_unaligned(param_1.read_unaligned() >> 0x10);
            param_2.write_unaligned(param_2.read_unaligned() >> 0x10);
        }
    }
}
