// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: i32;
    static mut DAT_10123120: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004fad9() {
    mech_trace::enter!("1004fad9");
    // SAFETY: the globals are the game's own memory; the divisor is a nonzero
    // constant other than -1, so the division cannot fault.
    unsafe {
        mech_trace::gw!(
            "1004fad9",
            "10123120",
            DAT_10123120,
            mech_trace::gr!("1004fad9", "100b38fc", DAT_100b38fc) / 0xb5
        );
    }
}
