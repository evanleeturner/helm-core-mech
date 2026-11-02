// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ef030: u8;
    static mut DAT_100ef032: u8;
    static mut DAT_100ef034: u8;
    static mut DAT_100ef038: u8;
    static mut DAT_100ef03c: u8;
    static mut DAT_100ef044: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100645c5(param_1: i32) {
    let i_var1 = param_1.wrapping_mul(0x40) as isize;
    // SAFETY: the globals are the game's own memory; the offsets address the
    // game's table entries exactly as the original does.
    unsafe {
        (&raw mut DAT_100ef030)
            .wrapping_byte_offset(i_var1)
            .cast::<u16>()
            .write_unaligned(0);
        (&raw mut DAT_100ef032)
            .wrapping_byte_offset(i_var1)
            .cast::<u16>()
            .write_unaligned(0);
        (&raw mut DAT_100ef034)
            .wrapping_byte_offset(i_var1)
            .cast::<u32>()
            .write_unaligned(0xffff_ffff);
        (&raw mut DAT_100ef038)
            .wrapping_byte_offset(i_var1)
            .cast::<u32>()
            .write_unaligned(0);
        (&raw mut DAT_100ef03c)
            .wrapping_byte_offset(i_var1)
            .cast::<u32>()
            .write_unaligned(0);
        (&raw mut DAT_100ef044)
            .wrapping_byte_offset(i_var1)
            .write(0);
    }
}
