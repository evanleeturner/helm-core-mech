// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10008156(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100082ad() {
    mech_trace::enter!("100082ad");
    // SAFETY: the callee is the game's own function, called with the original's constant arguments.
    unsafe {
        FUN_10008156(2, 0);
    }
}
