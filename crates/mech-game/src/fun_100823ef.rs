// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9a30: u8;
    static mut DAT_100b9a32: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100823ef(param_1: i32, param_2: i32, param_3: i32) -> u32 {
    if param_3 == 0
        || param_1 < 0
        || 0xff < param_1
        || param_2 < 1
        || 0x100i32.wrapping_sub(param_1) < param_2
    {
        return 0xffff_ffff;
    }
    let mut local_8: i32 = 0;
    while local_8 < param_2 {
        let i_var1 = local_8.wrapping_add(param_1).wrapping_mul(3);
        let dst = local_8.wrapping_mul(3).wrapping_add(param_3) as *mut u8;
        // SAFETY: the tables are the game's own memory and the destination is
        // the caller's buffer; reads and writes happen as in the original.
        unsafe {
            let src0 = (&raw mut DAT_100b9a30).byte_offset(i_var1 as isize);
            dst.cast::<u16>()
                .write_unaligned(src0.cast::<u16>().read_unaligned());
            let src2 = (&raw mut DAT_100b9a32).byte_offset(i_var1 as isize);
            dst.byte_add(2).write(src2.read());
        }
        local_8 = local_8.wrapping_add(1);
    }
    0
}
