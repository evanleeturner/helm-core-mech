// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3928: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10036a72(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10065db4() {
    mech_trace::enter!("10065db4");
    // SAFETY: the callee and the global are the game's own; the global is
    // read by value and passed on as the original does.
    unsafe {
        FUN_10036a72(mech_trace::gr!("10065db4", "100b3928", DAT_100b3928));
    }
}
