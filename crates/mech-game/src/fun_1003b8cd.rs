// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160f54: i32;
    static mut DAT_10160eb0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003b8cd(param_1: i32, param_2: i32) {
    mech_trace::enter!("1003b8cd", param_1, param_2);
    // SAFETY: the globals are the game's own memory; param_1 is an address
    // the caller supplies, and the write lands at offset 0x3c as the original does.
    unsafe {
        mech_trace::gw!("1003b8cd", "10160f54", DAT_10160f54, param_2);
        mech_trace::wr!(
            "1003b8cd",
            (param_1.wrapping_add(0x3c) as *mut i32),
            write_unaligned,
            param_2
        );
        mech_trace::gw!(
            "1003b8cd",
            "10160eb0",
            DAT_10160eb0,
            (param_2 as u32).wrapping_shl(2)
        );
    }
}
