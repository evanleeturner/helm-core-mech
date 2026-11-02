// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a4bb8: Option<unsafe extern "C" fn(u32)>;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10059e80(param_1: u32) {
    mech_trace::enter!("10059e80", param_1);
    // SAFETY: the global holds the game's own function pointer, called with
    // the argument as the original does.
    unsafe {
        mech_trace::cr!(
            "10059e80",
            "ind",
            (mech_trace::gr!("10059e80", "100a4bb8", DAT_100a4bb8).unwrap_unchecked())(param_1)
        );
    }
}
