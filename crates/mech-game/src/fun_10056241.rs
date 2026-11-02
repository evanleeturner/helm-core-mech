// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10055f9e(
        param_1: i32,
        param_2: i32,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
    ) -> u32;
    fn FUN_1006065d(param_1: i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10056241(param_1: i32, param_2: i32, param_3: *mut i32) -> u32 {
    // SAFETY: the callees are the game's own functions; arguments are passed through unchanged.
    unsafe {
        let u_var1 = FUN_10055f9e(param_1, param_2, param_3, 0xf, 0);
        if 0x3c < (u_var1 as i32) {
            FUN_1006065d(0x11, 0xb5, 1);
        }
        u_var1
    }
}
