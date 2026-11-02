// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1012327a: u8;
    static mut DAT_10123281: u8;
    static mut DAT_10123285: u8;
    static mut DAT_10123289: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004e5e3(param_1: i32, param_2: i32, param_3: i32) -> u32 {
    mech_trace::enter!("1004e5e3", param_1, param_2, param_3);
    // SAFETY: the globals are the game's own tables; offsets are computed
    // as the original does and read where the original reads.
    unsafe {
        let i_var3 = param_1
            .wrapping_mul(0x3c0a)
            .wrapping_add(param_2.wrapping_mul(0x13f));
        let off = param_3.wrapping_mul(0xc).wrapping_add(i_var3);
        let i_var2 = mech_trace::rd!(
            "1004e5e3",
            (&raw mut DAT_10123281)
                .byte_offset(off as isize)
                .cast::<i32>(),
            read_unaligned
        );
        let a = mech_trace::rd!(
            "1004e5e3",
            (&raw mut DAT_10123289)
                .byte_offset(off as isize)
                .cast::<u32>(),
            read_unaligned
        );
        let b = mech_trace::rd!(
            "1004e5e3",
            (&raw mut DAT_10123285)
                .byte_offset(off as isize)
                .cast::<i32>(),
            read_unaligned
        );
        let idx = (a & 0xf)
            .wrapping_mul(0x3c0a)
            .wrapping_add(b.wrapping_mul(0x13f) as u32);
        let c_var1 = mech_trace::rd!(
            "1004e5e3",
            (&raw mut DAT_1012327a)
                .byte_offset(idx as i32 as isize)
                .cast::<i8>(),
            read
        );
        mech_trace::rt!(
            "1004e5e3",
            if (i_var2 == 1 && (c_var1 == 5 || c_var1 == 6))
                || (i_var2 == 2 && c_var1 == 5)
                || (i_var2 == 3 && c_var1 == 6)
            {
                1
            } else {
                0
            }
        )
    }
}
