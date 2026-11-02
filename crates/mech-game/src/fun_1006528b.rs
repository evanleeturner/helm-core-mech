// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7f98: u32;
    static mut DAT_100c7e88: u8;
    static mut DAT_100c7fa0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006528b() {
    mech_trace::enter!("1006528b");
    // SAFETY: the globals are the game's own memory, written where the original does.
    unsafe {
        mech_trace::gw!("1006528b", "100c7f98", DAT_100c7f98, 0);
        mech_trace::gw!("1006528b", "100c7e88", DAT_100c7e88, 0);
        mech_trace::gw!("1006528b", "100c7fa0", DAT_100c7fa0, 8);
    }
}
