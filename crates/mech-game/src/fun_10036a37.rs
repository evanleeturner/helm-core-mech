// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100add20: [u8; 256];
    static mut DAT_100ade20: [u8; 256];
    static mut DAT_100adf20: u32;
    static mut DAT_100adf24: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036a37(param_1: u32) -> i32 {
    let mut param_1 = param_1;
    let p: *mut u8;
    let i_var1: u32;
    // SAFETY: the globals are the game's own memory; the table base is taken
    // by address and indexed as u32 entries, as the original does.
    unsafe {
        if (param_1 & 0x80) == 0 {
            p = &raw mut DAT_100ade20 as *mut u8;
            i_var1 = DAT_100adf24;
        } else {
            param_1 ^= 0x80;
            p = &raw mut DAT_100add20 as *mut u8;
            i_var1 = DAT_100adf20;
        }
        let v = p
            .byte_add((param_1 as usize).wrapping_mul(4))
            .cast::<u32>()
            .read_unaligned();
        i_var1.wrapping_sub(v) as i32
    }
}
