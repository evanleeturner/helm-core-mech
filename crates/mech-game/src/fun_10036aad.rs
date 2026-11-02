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
pub unsafe extern "C" fn FUN_10036aad(param_1: u32, param_2: i32) {
    let mut param_1 = param_1;
    // SAFETY: the globals are the game's own memory; the table written is the
    // game's array, indexed where the original indexes it.
    unsafe {
        let p_var2: *mut u32;
        let i_var1: u32;
        if (param_1 & 0x80) == 0 {
            p_var2 = (&raw mut DAT_100ade20).cast::<u32>();
            i_var1 = DAT_100adf24;
        } else {
            param_1 ^= 0x80;
            p_var2 = (&raw mut DAT_100add20).cast::<u32>();
            i_var1 = DAT_100adf20;
        }
        p_var2
            .add(param_1 as usize)
            .write_unaligned(i_var1.wrapping_sub(param_2 as u32));
    }
}
