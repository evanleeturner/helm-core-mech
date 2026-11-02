// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __ftol() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100504a0(param_1: u32, param_2: u32) -> i64 {
    mech_trace::enter!("100504a0", param_1, param_2);
    let _ = (param_1, param_2);
    // SAFETY: __ftol is the game's own runtime routine; every path of the
    // original reaches this single call.
    unsafe {
        let l_var1 = mech_trace::cr!("100504a0", "__ftol", __ftol());
        mech_trace::rt!("100504a0", l_var1 as i64)
    }
}
