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
    // SAFETY: the globals and callees are the game's own; the table writes
    // stay within the original's 8-entry arrays.
    unsafe {
        if DAT_100aefb8 == 0 {
            let mut local_8: i32 = 0;
            while local_8 < 8 {
                (&raw mut DAT_10160e90)
                    .byte_add(local_8.wrapping_mul(4) as usize)
                    .cast::<u32>()
                    .write_unaligned(0xffffffff);
                (&raw mut DAT_10160e80)
                    .byte_add(local_8.wrapping_mul(2) as usize)
                    .cast::<u16>()
                    .write_unaligned(0);
                local_8 = local_8.wrapping_add(1);
            }
            _AIL_set_preference_8(9, 0xc);
            DAT_100aefb0 = FUN_1006af3b();
            if DAT_100aefb0 == 0 {
                0xfffffffb
            } else {
                let i_var2 = FUN_10003d60(DAT_100b3c54 as i32, 0x7f);
                FUN_1003c756(i_var2 as i16);
                DAT_100aefb8 = 1;
                1
            }
        } else {
            0xffffffff
        }
    }
}
