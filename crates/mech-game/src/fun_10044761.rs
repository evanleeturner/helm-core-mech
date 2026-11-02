// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: i16) -> bool;
    fn FUN_1002a132(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10044761(param_1: i32, param_2: i16) -> i32 {
    // SAFETY: param_1 is the game's object address, read at offset 0xdc as the
    // original does; the callees are the game's own functions.
    unsafe {
        let u_var1 = ((param_1.wrapping_add(0xdc)) as *const u32).read_unaligned();
        FUN_1002950f(param_1, param_2);
        let i_var2 = FUN_1002a132(param_1);
        FUN_1002950f(param_1, u_var1 as i16);
        i_var2.wrapping_add(0x1680000) % 0x1680000
    }
}
