// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aae84: u32;
    static mut DAT_100b3c9c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002cd87(param_1: u32, param_2: i32) -> u8;
    fn FUN_10039649(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008117d(param_1: u32, param_2: i32) {
    mech_trace::enter!("1008117d", param_1, param_2);
    let _ = param_1;
    // SAFETY: the callees and globals are the game's own; DAT_100b3c9c holds
    // a pointer to a structure whose field at offset 0x18 is written.
    unsafe {
        mech_trace::cr!("1008117d", "1002cd87", FUN_1002cd87(0x800, param_2));
        mech_trace::cr!("1008117d", "10039649", FUN_10039649(param_2));
        mech_trace::gw!("1008117d", "100aae84", DAT_100aae84, param_2 as u32);
        mech_trace::wr!(
            "1008117d",
            ((mech_trace::gr!("1008117d", "100b3c9c", DAT_100b3c9c) as usize + 0x18) as *mut i32),
            write_unaligned,
            param_2
        );
    }
}
