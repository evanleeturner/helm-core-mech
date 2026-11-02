// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __ftol() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100504a0(param_1: u32, param_2: u32) -> i64 {
    let _ = (param_1, param_2);
    // SAFETY: __ftol is the game's own runtime routine; every path of the
    // original reaches this single call.
    unsafe {
        let l_var1 = __ftol();
        l_var1 as i64
    }
}
