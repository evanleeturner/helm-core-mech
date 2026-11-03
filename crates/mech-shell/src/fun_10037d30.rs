// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut s_USERSTAR_BWD_10094360: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006f780(param_1: *const i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10037d30() {
    mech_trace::enter!("10037d30");
    // SAFETY: the callee and the string global are the game's own; the
    // string's address is passed as the original does.
    unsafe {
        FUN_1006f780((&raw mut s_USERSTAR_BWD_10094360).cast::<i8>());
    }
}
