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
    fn FUN_1003c3a4() -> u8;
    fn FUN_1004668e() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006abdb() {
    mech_trace::enter!("1006abdb");
    // SAFETY: the globals and callees are the game's own memory and code;
    // they are read, written and called in the original's order.
    unsafe {
        if mech_trace::gr!("1006abdb", "100b3ca0", DAT_100b3ca0) != 0 {
            if mech_trace::gr!("1006abdb", "100b3c44", DAT_100b3c44) != u32::MAX
                && (mech_trace::gr!("1006abdb", "100b3c58", DAT_100b3c58) & 4) != 0
            {
                mech_trace::cr!("1006abdb", "1003c3a4", FUN_1003c3a4());
            }
            if mech_trace::gr!("1006abdb", "100b3c40", DAT_100b3c40) != u32::MAX
                && (mech_trace::gr!("1006abdb", "100b3c58", DAT_100b3c58) & 8) != 0
            {
                mech_trace::cr!("1006abdb", "1004668e", FUN_1004668e());
            }
            mech_trace::gw!("1006abdb", "100b3ca0", DAT_100b3ca0, 0);
        }
    }
}
