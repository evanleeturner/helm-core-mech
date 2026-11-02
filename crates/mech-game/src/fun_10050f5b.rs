// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0864: i32;
    static mut DAT_101155c4: u8;
    static mut DAT_101155e4: u8;
    static mut DAT_101155e8: u8;
    static mut DAT_10115606: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10050f5b(param_1: i32) {
    let mut local_8: i32 = 0;
    // SAFETY: the globals are the game's own tables; the entries are read and
    // written at the same offsets and widths as the original.
    unsafe {
        loop {
            if DAT_100b0864 <= local_8 {
                return;
            }
            let i_var1 = local_8.wrapping_mul(0x48) as isize;
            let v = (&raw mut DAT_101155e8)
                .byte_offset(i_var1)
                .cast::<i32>()
                .read_unaligned();
            if v == param_1 {
                (&raw mut DAT_101155e4)
                    .byte_offset(i_var1)
                    .cast::<u32>()
                    .write_unaligned(0);
                (&raw mut DAT_101155c4)
                    .byte_offset(i_var1)
                    .cast::<u32>()
                    .write_unaligned(0xffff_ffff);
                (&raw mut DAT_10115606)
                    .byte_offset(i_var1)
                    .cast::<u16>()
                    .write_unaligned(0);
                return;
            }
            local_8 = local_8.wrapping_add(1);
        }
    }
}
