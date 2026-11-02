// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006b20(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10006b9a(param_1: i32) {
    mech_trace::enter!("10006b9a", param_1);
    // SAFETY: the callee is the game's own function; the argument is passed by value.
    unsafe {
        mech_trace::cr!(
            "10006b9a",
            "10006b20",
            FUN_10006b20(param_1.wrapping_add(0x5a0000))
        );
    }
}
