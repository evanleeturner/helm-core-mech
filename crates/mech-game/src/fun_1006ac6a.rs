// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: u32;
    static mut DAT_100b3c40: u32;
    static mut DAT_100b3c58: u8;
    static mut DAT_100b3ca0: u32;
    static mut DAT_100b3ca8: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10046928() -> u32;
    fn FUN_1004663a() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10046b4d(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006ac6a() {
    mech_trace::enter!("1006ac6a");
    // SAFETY: the globals and callees are the game's own; accessed in the
    // same order as the original.
    unsafe {
        mech_trace::gw!(
            "1006ac6a",
            "100b3ca8",
            DAT_100b3ca8,
            mech_trace::gr!("1006ac6a", "100b3ca8", DAT_100b3ca8).wrapping_add(mech_trace::gr!(
                "1006ac6a",
                "100b3904",
                DAT_100b3904
            ))
        );
        if 0x169 < mech_trace::gr!("1006ac6a", "100b3ca8", DAT_100b3ca8) {
            mech_trace::gw!("1006ac6a", "100b3ca8", DAT_100b3ca8, 0);
            if mech_trace::gr!("1006ac6a", "100b3ca0", DAT_100b3ca0) == 0
                && (mech_trace::gr!("1006ac6a", "100b3c58", DAT_100b3c58) & 8) != 0
            {
                let i_var1 = mech_trace::cr!("1006ac6a", "10046928", FUN_10046928());
                if i_var1 != 0
                    && mech_trace::gr!("1006ac6a", "100b3c40", DAT_100b3c40) != 0xFFFF_FFFF
                {
                    mech_trace::cr!(
                        "1006ac6a",
                        "10046b4d",
                        FUN_10046b4d(mech_trace::gr!("1006ac6a", "100b3c40", DAT_100b3c40) as i32)
                    );
                    mech_trace::cr!("1006ac6a", "1004663a", FUN_1004663a());
                }
            }
        }
    }
}
