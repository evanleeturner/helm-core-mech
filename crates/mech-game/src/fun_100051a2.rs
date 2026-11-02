// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10005624(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100051a2(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: param_2 points at the game's structure; the pointer at +0x58 and
    // the indexed entry are read as the original does, and the callee is the game's own.
    unsafe {
        let base = (param_2.wrapping_add(0x58) as *const i32).read_unaligned();
        let addr = base
            .wrapping_add(param_3.wrapping_mul(0x28))
            .wrapping_sub(4);
        let s_var1 = (addr as *const i16).read_unaligned() as i32;
        let mut local_8: i32 = 0;
        while local_8 < s_var1 {
            FUN_10005624(param_1, param_2, param_3, 0, 1);
            local_8 = local_8.wrapping_add(1);
        }
    }
}
