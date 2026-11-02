// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adca0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10033e45() -> *mut u32 {
    mech_trace::enter!("10033e45");
    mech_trace::rt!("10033e45", &raw mut DAT_100adca0)
}
