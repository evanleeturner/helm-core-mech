// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c06c0: [u8; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10088ea8(param_1: i16) -> u32 {
    mech_trace::enter!("10088ea8", param_1);
    let hi = ((param_1 as u16) >> 8) as u8 as i8 as i32 as u32;
    // SAFETY: DAT_100c06c0 is the game's own table; the index is read where the original reads it.
    let lo = unsafe {
        mech_trace::rd!(
            "10088ea8",
            (&raw mut DAT_100c06c0)
                .cast::<u8>()
                .offset(param_1 as isize),
            read
        )
    };
    mech_trace::rt!(
        "10088ea8",
        (hi.wrapping_shl(8) | u32::from(lo)) & 0xffff00ff
    )
}
