// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100bd7c8: u8;
    static mut DAT_100bd948: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10050a10() {
    // SAFETY: the globals are the game's own memory; the loop zeroes 48-byte
    // blocks starting at DAT_100bd7c8 until the end at DAT_100bd948, as the
    // original does.
    unsafe {
        let end = (&raw mut DAT_100bd948) as *mut u32;
        let mut p_var2 = (&raw mut DAT_100bd7c8) as *mut u32;
        loop {
            let p_var3 = p_var2.wrapping_add(0xc);
            let mut i_var1: i32 = 0xc;
            while i_var1 != 0 {
                p_var2.write_unaligned(0);
                p_var2 = p_var2.wrapping_add(1);
                i_var1 = i_var1.wrapping_sub(1);
            }
            p_var2 = p_var3;
            if !(p_var3 < end) {
                break;
            }
        }
    }
}
