// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005aae4(param_1: i32) -> u32;
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
pub unsafe extern "C" fn FUN_100666ad(
    param_1: *mut i32,
    param_2: u32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) {
    let _ = param_2;
    // SAFETY: the callees are the game's own; param_1 points at the caller's
    // int array, of which elements 1 and 3 are read as the original does.
    unsafe {
        let i_var1 = FUN_1005aae4(param_4) as i32;
        let a = param_3.wrapping_add(i_var1);
        let h = param_1
            .add(3)
            .read_unaligned()
            .wrapping_sub(param_1.add(1).read_unaligned());
        FUN_10056489(param_1, 0, a, h, a, 0, param_5);
    }
}
