// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut PTR_s_MW2_EXE___warning_100a9e18: u32;
    static mut DAT_100a9e1c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10010b46(param_1: u32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10010b8f(param_1: u32) {
    mech_trace::enter!("10010b8f", param_1);
    // SAFETY: the callee and the globals are the game's own memory; the
    // calls and writes happen in the original's order.
    unsafe {
        FUN_10010b46(
            mech_trace::gr!(
                "10010b8f",
                "PTR_s_MW2_EXE___warning_100a9e18",
                PTR_s_MW2_EXE___warning_100a9e18
            ),
            param_1,
        );
        mech_trace::gw!("10010b8f", "100a9e1c", DAT_100a9e1c, 0);
    }
}
