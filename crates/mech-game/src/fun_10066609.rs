// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10056489(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10066609(param_1: *mut i32, param_2: i32) {
    // SAFETY: param_1 points at the caller's int array with at least five
    // elements; the callee is the game's own.
    unsafe {
        let i_var1 = param_1
            .add(3)
            .read_unaligned()
            .wrapping_sub(param_1.add(1).read_unaligned());
        let i_var2 = param_1
            .add(4)
            .read_unaligned()
            .wrapping_sub(param_1.add(2).read_unaligned());
        FUN_10056489(param_1, 0, 0, i_var1, 0, 0, param_2);
        FUN_10056489(param_1, i_var1, 0, i_var1, i_var2, 0, param_2);
        FUN_10056489(param_1, i_var1, i_var2, 0, i_var2, 0, param_2);
        FUN_10056489(param_1, 0, i_var2, 0, 0, 0, param_2);
    }
}
