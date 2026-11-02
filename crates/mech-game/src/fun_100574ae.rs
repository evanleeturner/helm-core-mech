// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b2b10: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100574ae(param_1: i32, param_2: u8) {
    mech_trace::enter!("100574ae", param_1, param_2);
    // SAFETY: DAT_100b2b10 is the game's own byte table; the original writes
    // one byte at the given offset from its address.
    unsafe {
        mech_trace::wr!(
            "100574ae",
            (&raw mut DAT_100b2b10).byte_offset(param_1 as isize),
            write,
            param_2
        );
    }
}
