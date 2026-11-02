// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a8a38: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10064e59(param_1: *mut u32) {
    // SAFETY: DAT_100a8a38 is the game's own buffer of at least 13 dwords, and
    // param_1 points at the caller's 13 dwords, copied in order.
    unsafe {
        let mut p_var2 = (&raw mut DAT_100a8a38).cast::<u32>();
        let mut src = param_1;
        let mut i_var1: i32 = 0xd;
        while i_var1 != 0 {
            p_var2.write_unaligned(src.read_unaligned());
            src = src.wrapping_add(1);
            p_var2 = p_var2.wrapping_add(1);
            i_var1 = i_var1.wrapping_sub(1);
        }
    }
}
