// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b67f0: u8;
    static mut DAT_100af114: *mut u8;
    static mut DAT_100af11c: *mut u8;
    static mut DAT_100af120: *mut u8;
    static mut DAT_100af134: *mut u8;
    static mut DAT_100af13c: *mut u8;
    static mut DAT_100af144: *mut u8;
    static mut DAT_100af148: *mut u8;
    static mut DAT_100af14c: *mut u8;
    static mut DAT_100af150: *mut u8;
    static mut DAT_100af15c: *mut u8;
    static mut DAT_100af164: *mut u8;
    static mut DAT_100af168: *mut u8;
    static mut DAT_100af16c: *mut u8;
    static mut DAT_100af174: *mut u8;
    static mut DAT_100af178: *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007bac8(param_1: u32) -> *mut u8 {
    mech_trace::enter!("1007bac8", param_1);
    // SAFETY: the globals are the game's own pointer table and data array;
    // they are read by value or addressed without creating references.
    unsafe {
        mech_trace::rt!(
            "1007bac8",
            match param_1 {
                1 => mech_trace::gr!("1007bac8", "100af15c", DAT_100af15c),
                3 => mech_trace::gr!("1007bac8", "100af144", DAT_100af144),
                4 => mech_trace::gr!("1007bac8", "100af134", DAT_100af134),
                5 => mech_trace::gr!("1007bac8", "100af150", DAT_100af150),
                6 => mech_trace::gr!("1007bac8", "100af14c", DAT_100af14c),
                7 => mech_trace::gr!("1007bac8", "100af148", DAT_100af148),
                8 => mech_trace::gr!("1007bac8", "100af164", DAT_100af164),
                10 => mech_trace::gr!("1007bac8", "100af114", DAT_100af114),
                0xb => mech_trace::gr!("1007bac8", "100af168", DAT_100af168),
                0xc => mech_trace::gr!("1007bac8", "100af120", DAT_100af120),
                0xd => mech_trace::gr!("1007bac8", "100af13c", DAT_100af13c),
                0xe => mech_trace::gr!("1007bac8", "100af11c", DAT_100af11c),
                0xf => mech_trace::gr!("1007bac8", "100af16c", DAT_100af16c),
                0x11 => mech_trace::gr!("1007bac8", "100af174", DAT_100af174),
                0x14 => mech_trace::gr!("1007bac8", "100af178", DAT_100af178),
                _ => &raw mut DAT_100b67f0,
            }
        )
    }
}
