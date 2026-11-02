// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3520: u32;
    static mut DAT_100b3524: u32;
    static mut DAT_100b3528: u32;
    static mut DAT_10113170: u8;
    static mut DAT_10113172: u8;
    static mut DAT_10113174: u8;
    static mut DAT_10113176: u8;
    static mut DAT_10113178: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005f5c6(param_1: i32, param_2: i32) -> u32 {
    let mut param_2 = param_2;
    // SAFETY: the globals are the game's own memory; the table entries are
    // 0xe bytes each and param_1 is bounds-checked to 0..=0x1ff before use.
    unsafe {
        if param_2 == -1 {
            param_2 = DAT_100b3520 as i32;
            DAT_100b3524 = 1;
        }
        if param_2 < 0 || 0x1ff < param_2 || param_1 < 0 || 0x1ff < param_1 {
            0
        } else {
            let off = (param_1 as usize).wrapping_mul(0xe);
            (&raw mut DAT_10113170)
                .byte_add(off)
                .cast::<i16>()
                .write_unaligned(param_2 as i16);
            (&raw mut DAT_10113178)
                .byte_add(off)
                .cast::<u16>()
                .write_unaligned(1);
            (&raw mut DAT_10113176)
                .byte_add(off)
                .cast::<u16>()
                .write_unaligned(1);
            (&raw mut DAT_10113172)
                .byte_add(off)
                .cast::<u16>()
                .write_unaligned(0);
            if DAT_100b3528 != 0 {
                (&raw mut DAT_10113174)
                    .byte_add(off)
                    .cast::<u16>()
                    .write_unaligned(0x2d);
            }
            1
        }
    }
}
