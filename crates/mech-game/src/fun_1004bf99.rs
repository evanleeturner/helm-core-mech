// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10165850: u8;
    static mut DAT_100b0704: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10047c9e(param_1: *mut i8, param_2: *mut u8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004bf99() {
    mech_trace::enter!("1004bf99");
    // SAFETY: the callee and the globals are the game's own; the addresses
    // of the string and buffer are passed through unchanged.
    unsafe {
        mech_trace::cr!(
            "1004bf99",
            "10047c9e",
            FUN_10047c9e(&raw mut DAT_100b0704, &raw mut DAT_10165850)
        );
    }
}
