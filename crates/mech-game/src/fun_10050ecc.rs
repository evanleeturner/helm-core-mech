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
    mech_trace::enter!("10050ecc", param_1);
    let mut local_8: i32 = 0;
    let i_var1: i32;
    // SAFETY: the globals are the game's own memory; the arrays are tables of
    // 0x48-byte records, read and written at the same offsets as the original.
    unsafe {
        loop {
            if mech_trace::gr!("10050ecc", "100b0864", DAT_100b0864) <= local_8 {
                return;
            }
            let off = local_8.wrapping_mul(0x48);
            if mech_trace::rd!(
                "10050ecc",
                (&raw mut DAT_101155e8).byte_add(off as usize).cast::<i32>(),
                read_unaligned
            ) == param_1
            {
                i_var1 = off;
                break;
            }
            local_8 = local_8.wrapping_add(1);
        }
        let p = (&raw mut DAT_101155e4)
            .byte_add(i_var1 as usize)
            .cast::<i32>();
        if mech_trace::rd!("10050ecc", p, read_unaligned) != 0 {
            mech_trace::cr!(
                "10050ecc",
                "1001d671",
                FUN_1001d671(mech_trace::rd!("10050ecc", p, read_unaligned) as usize as *mut u8)
            );
        }
        mech_trace::wr!("10050ecc", p, write_unaligned, 0);
        mech_trace::wr!(
            "10050ecc",
            (&raw mut DAT_101155c4)
                .byte_add(i_var1 as usize)
                .cast::<u32>(),
            write_unaligned,
            0xffff_ffff
        );
        mech_trace::wr!(
            "10050ecc",
            (&raw mut DAT_10115606)
                .byte_add(i_var1 as usize)
                .cast::<u16>(),
            write_unaligned,
            1
        );
    }
}
