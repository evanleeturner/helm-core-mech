// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a4080: u8;
    static mut DAT_100a4084: u8;
    static mut DAT_100a40a0: i32;
    static mut DAT_100c4c70: u32;
    static mut DAT_100c4c88: u32;
    static mut DAT_10218074: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100609fa(param_1: i32, param_2: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10002383(param_1: i32) {
    // SAFETY: the globals and the callee are the game's own; the tables at
    // DAT_100a4080 and DAT_100a4084 are indexed in 8-byte entries as the
    // original does.
    unsafe {
        if DAT_100c4c88 == 1 && param_1 != DAT_100a40a0 {
            let mut local_8: u32;
            if DAT_10218074 == 1 {
                local_8 = 0xb5;
                DAT_10218074 = 0;
            } else {
                local_8 = (&raw mut DAT_100a4084)
                    .cast::<u8>()
                    .byte_offset(param_1.wrapping_mul(8) as isize)
                    .cast::<u32>()
                    .read_unaligned();
            }
            if DAT_100c4c70 == 2 {
                DAT_100c4c70 = 3;
                local_8 = 0x16a;
            }
            let arg = (&raw mut DAT_100a4080)
                .cast::<u8>()
                .byte_offset(param_1.wrapping_mul(8) as isize)
                .cast::<i32>()
                .read_unaligned();
            FUN_100609fa(arg, local_8);
            DAT_100a40a0 = param_1;
        }
    }
}
