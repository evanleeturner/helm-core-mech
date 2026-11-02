// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3928: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10036a37(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10065d96() {
    mech_trace::enter!("10065d96");
    // SAFETY: the callee and the global are the game's own.
    unsafe {
        FUN_10036a37(mech_trace::gr!("10065d96", "100b3928", DAT_100b3928));
    }
}
