// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab2dc: unsafe extern "C" fn(u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001dfab(param_1: u32) {
    mech_trace::enter!("1001dfab", param_1);
    // SAFETY: the global holds the game's own function pointer, called
    // with the caller's argument as the original does.
    unsafe {
        mech_trace::cr!(
            "1001dfab",
            "ind",
            (mech_trace::gr!("1001dfab", "100ab2dc", DAT_100ab2dc))(param_1)
        );
    }
}
