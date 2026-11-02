// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003106f(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10081083() {
    mech_trace::enter!("10081083");
    // SAFETY: the callee is the game's own function; -1 is the argument the original passes.
    unsafe {
        FUN_1003106f(-1);
    }
}
