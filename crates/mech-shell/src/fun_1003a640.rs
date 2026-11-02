// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b6f44: u32;
    static mut DAT_100942e8: u32;
    static mut DAT_100942ec: u32;
    static mut DAT_100942f0: u32;
    static mut DAT_100942f4: u32;
    static mut DAT_100942f8: u32;
    static mut DAT_100942fc: u32;
    static mut DAT_10094300: u32;
    static mut DAT_10094304: u32;
    static mut DAT_10094308: u32;
    static mut DAT_1009430c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003a640() {
    mech_trace::enter!("1003a640");
    // SAFETY: the globals are the game's own memory, written in the original's order.
    unsafe {
        mech_trace::gw!("1003a640", "100b6f44", DAT_100b6f44, 0xffffffff);
        mech_trace::gw!("1003a640", "100942e8", DAT_100942e8, 0);
        mech_trace::gw!("1003a640", "100942ec", DAT_100942ec, 0);
        mech_trace::gw!("1003a640", "100942f0", DAT_100942f0, 0);
        mech_trace::gw!("1003a640", "100942f4", DAT_100942f4, 0);
        mech_trace::gw!("1003a640", "100942f8", DAT_100942f8, 0);
        mech_trace::gw!("1003a640", "100942fc", DAT_100942fc, 0);
        mech_trace::gw!("1003a640", "10094300", DAT_10094300, 0);
        mech_trace::gw!("1003a640", "10094304", DAT_10094304, 0);
        mech_trace::gw!("1003a640", "10094308", DAT_10094308, 0);
        mech_trace::gw!("1003a640", "1009430c", DAT_1009430c, 0);
    }
}
