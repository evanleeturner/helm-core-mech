// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001e3ac(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003eacf() {
    mech_trace::enter!("1003eacf");
    // SAFETY: the callee and the global are the game's own; the global is
    // read by value and passed on as the original does.
    unsafe {
        FUN_1001e3ac(mech_trace::gr!("1003eacf", "100af17c", DAT_100af17c));
    }
}
