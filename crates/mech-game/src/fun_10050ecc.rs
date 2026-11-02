// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0864: i32;
    static mut DAT_101155c4: u8;
    static mut DAT_101155e4: u8;
    static mut DAT_101155e8: u8;
    static mut DAT_10115606: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001d671(param_1: *mut u8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10050ecc(param_1: i32) {
    let mut local_8: i32 = 0;
    let i_var1: i32;
    // SAFETY: the globals are the game's own memory; the arrays are tables of
    // 0x48-byte records, read and written at the same offsets as the original.
    unsafe {
        loop {
            if DAT_100b0864 <= local_8 {
                return;
            }
            let off = local_8.wrapping_mul(0x48);
            if (&raw mut DAT_101155e8)
                .byte_add(off as usize)
                .cast::<i32>()
                .read_unaligned()
                == param_1
            {
                i_var1 = off;
                break;
            }
            local_8 = local_8.wrapping_add(1);
        }
        let p = (&raw mut DAT_101155e4)
            .byte_add(i_var1 as usize)
            .cast::<i32>();
        if p.read_unaligned() != 0 {
            FUN_1001d671(p.read_unaligned() as usize as *mut u8);
        }
        p.write_unaligned(0);
        (&raw mut DAT_101155c4)
            .byte_add(i_var1 as usize)
            .cast::<u32>()
            .write_unaligned(0xffff_ffff);
        (&raw mut DAT_10115606)
            .byte_add(i_var1 as usize)
            .cast::<u16>()
            .write_unaligned(1);
    }
}
