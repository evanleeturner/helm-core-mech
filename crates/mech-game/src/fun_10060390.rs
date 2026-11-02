// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100f3040: u8;
    static mut DAT_100f3120: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10060390(param_1: *mut u32) {
    // SAFETY: the globals are the game's own buffers, large enough for the
    // 11 records of 0x14 bytes and the 20 dwords; param_1 points at the
    // caller's 5 dwords, read where the original does.
    unsafe {
        let mut local_8: i32 = 0;
        while local_8 < 0xb {
            let mut puvar2 = param_1;
            let mut puvar3 = (&raw mut DAT_100f3040)
                .byte_add(local_8.wrapping_mul(0x14) as usize)
                .cast::<u32>();
            let mut i_var1: i32 = 5;
            while i_var1 != 0 {
                puvar3.write_unaligned(puvar2.read_unaligned());
                puvar2 = puvar2.add(1);
                puvar3 = puvar3.add(1);
                i_var1 = i_var1.wrapping_sub(1);
            }
            local_8 = local_8.wrapping_add(1);
        }
        local_8 = 0;
        while local_8 < 0x14 {
            (&raw mut DAT_100f3120)
                .byte_add(local_8.wrapping_mul(4) as usize)
                .cast::<u32>()
                .write_unaligned(0);
            local_8 = local_8.wrapping_add(1);
        }
    }
}
