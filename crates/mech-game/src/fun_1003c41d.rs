// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefb0: u32;
    static mut DAT_100aefb8: u32;
    static mut DAT_100b3c54: u32;
    static mut DAT_10160e80: u8;
    static mut DAT_10160e90: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_set_preference_8(param_1: u32, param_2: u32) -> u32;
    fn FUN_1006af3b() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1003c756(param_1: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003c41d() -> u32 {
    mech_trace::enter!("1003c41d");
    // SAFETY: the globals and callees are the game's own; the table writes
    // stay within the original's 8-entry arrays.
    unsafe {
        mech_trace::rt!(
            "1003c41d",
            if mech_trace::gr!("1003c41d", "100aefb8", DAT_100aefb8) == 0 {
                let mut local_8: i32 = 0;
                while local_8 < 8 {
                    mech_trace::wr!(
                        "1003c41d",
                        (&raw mut DAT_10160e90)
                            .byte_add(local_8.wrapping_mul(4) as usize)
                            .cast::<u32>(),
                        write_unaligned,
                        0xffffffff
                    );
                    mech_trace::wr!(
                        "1003c41d",
                        (&raw mut DAT_10160e80)
                            .byte_add(local_8.wrapping_mul(2) as usize)
                            .cast::<u16>(),
                        write_unaligned,
                        0
                    );
                    local_8 = local_8.wrapping_add(1);
                }
                mech_trace::cr!(
                    "1003c41d",
                    "_AIL_set_preference_8",
                    _AIL_set_preference_8(9, 0xc)
                );
                mech_trace::gw!(
                    "1003c41d",
                    "100aefb0",
                    DAT_100aefb0,
                    mech_trace::cr!("1003c41d", "1006af3b", FUN_1006af3b())
                );
                if mech_trace::gr!("1003c41d", "100aefb0", DAT_100aefb0) == 0 {
                    0xfffffffb
                } else {
                    let i_var2 = mech_trace::cr!(
                        "1003c41d",
                        "10003d60",
                        FUN_10003d60(
                            mech_trace::gr!("1003c41d", "100b3c54", DAT_100b3c54) as i32,
                            0x7f
                        )
                    );
                    FUN_1003c756(i_var2 as i16);
                    mech_trace::gw!("1003c41d", "100aefb8", DAT_100aefb8, 1);
                    1
                }
            } else {
                0xffffffff
            }
        )
    }
}
