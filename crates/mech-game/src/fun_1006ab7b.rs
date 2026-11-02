// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c40: u32;
    static mut DAT_100b3c44: u32;
    static mut DAT_100b3c58: u8;
    static mut DAT_100b3ca0: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1003c32b() -> u8;
    fn FUN_1004668e() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006ab7b() {
    mech_trace::enter!("1006ab7b");
    // SAFETY: the globals and callees are the game's own memory and code,
    // accessed in the same order as the original.
    unsafe {
        if mech_trace::gr!("1006ab7b", "100b3ca0", DAT_100b3ca0) == 0 {
            if mech_trace::gr!("1006ab7b", "100b3c44", DAT_100b3c44) != 0xFFFF_FFFF
                && (mech_trace::gr!("1006ab7b", "100b3c58", DAT_100b3c58) & 4) != 0
            {
                mech_trace::cr!("1006ab7b", "1003c32b", FUN_1003c32b());
            }
            if mech_trace::gr!("1006ab7b", "100b3c40", DAT_100b3c40) != 0xFFFF_FFFF
                && (mech_trace::gr!("1006ab7b", "100b3c58", DAT_100b3c58) & 8) != 0
            {
                mech_trace::cr!("1006ab7b", "1004668e", FUN_1004668e());
            }
            mech_trace::gw!("1006ab7b", "100b3ca0", DAT_100b3ca0, 1);
        }
    }
}
