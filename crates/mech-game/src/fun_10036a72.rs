// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100add20: u8;
    static mut DAT_100ade20: u8;
    static mut DAT_100adf20: u32;
    static mut DAT_100adf24: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036a72(param_1: u32) {
    let mut param_1 = param_1;
    // SAFETY: the globals are the game's own memory; the tables are arrays
    // at these addresses, and the store lands where the original writes.
    unsafe {
        let p: *mut u32;
        let v: u32;
        if param_1 & 0x80 == 0 {
            p = (&raw mut DAT_100ade20).cast::<u32>();
            v = DAT_100adf24;
        } else {
            param_1 ^= 0x80;
            p = (&raw mut DAT_100add20).cast::<u32>();
            v = DAT_100adf20;
        }
        p.wrapping_add(param_1 as usize).write_unaligned(v);
    }
}
