// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100add1c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036b20(param_1: u16, set: i16) {
    mech_trace::enter!("10036b20", param_1, set);
    let mut mask: u32 = 0;
    if (param_1 & 0x80) != 0 {
        mask = 0x200;
    }
    if (param_1 & 0x100) != 0 {
        mask |= 0x100;
    }
    // SAFETY: DAT_100add1c is the game's own global, read and written by value.
    unsafe {
        if set == 0 {
            mech_trace::gw!(
                "10036b20",
                "100add1c",
                DAT_100add1c,
                mech_trace::gr!("10036b20", "100add1c", DAT_100add1c) & !mask
            );
        } else {
            mech_trace::gw!(
                "10036b20",
                "100add1c",
                DAT_100add1c,
                mech_trace::gr!("10036b20", "100add1c", DAT_100add1c) | mask
            );
        }
    }
}
