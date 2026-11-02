// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093900: u8;
    static mut PTR_DAT_10093b2c: *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10035e90() {
    mech_trace::enter!("10035e90");
    // SAFETY: the globals are the game's own memory; we store the address of
    // DAT_10093900 into the pointer global.
    unsafe {
        mech_trace::gw!(
            "10035e90",
            "PTR_DAT_10093b2c",
            PTR_DAT_10093b2c,
            &raw mut DAT_10093900
        );
    }
}
