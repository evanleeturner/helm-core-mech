// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008628d(param_1: u32, param_2: *mut i32) -> *mut u16;
    fn FUN_10085f40(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008664f(param_1: u32, param_2: *mut i32) {
    // SAFETY: the callees are the game's own functions; the pointer returned
    // by the lookup is passed on only when non-null.
    unsafe {
        let p_var1 = FUN_1008628d(param_1, param_2);
        if !p_var1.is_null() {
            FUN_10085f40(p_var1 as i32);
        }
    }
}
