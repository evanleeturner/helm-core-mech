// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100832d4(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002950f(param_1: i32, param_2: i16) -> bool {
    // SAFETY: param_1 is the address of a game object; the field at offset
    // 0xdc is written where the original does, and the callee is the game's own.
    unsafe {
        ((param_1.wrapping_add(0xdc)) as *mut i32).write_unaligned(param_2 as i32);
        let i_var1 = FUN_100832d4(param_1) as i32;
        0 < i_var1
    }
}
