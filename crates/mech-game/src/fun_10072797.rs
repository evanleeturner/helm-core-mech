// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b4e60: u32;
    static mut DAT_100b4e64: u32;
    static mut DAT_100b4e68: i32;
    static mut DAT_100ee9d0: u8;
    static mut DAT_100ee9d4: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100473bb(param_1: *mut i16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10072797(param_1: *mut i16) -> u32 {
    // SAFETY: the globals and the table at DAT_100ee9d0/d4 are the game's own
    // memory; param_1 points at the caller's short.
    unsafe {
        let mut local_8: u32 = 0;
        let s_var1: i16 = param_1.read_unaligned();
        let base0 = &raw mut DAT_100ee9d0;
        let base4 = &raw mut DAT_100ee9d4;
        let mut local_c: i32 = 0;
        while local_c < DAT_100b4e68
            && base0
                .byte_offset(local_c.wrapping_mul(8) as isize)
                .cast::<i32>()
                .read_unaligned()
                != s_var1 as i32
        {
            local_c = local_c.wrapping_add(1);
        }
        if local_c < DAT_100b4e68 {
            DAT_100b4e64 = base4
                .byte_offset(local_c.wrapping_mul(8) as isize)
                .cast::<i32>()
                .read_unaligned() as u32;
            local_8 = 0xffffffff;
        } else if DAT_100b4e68 < 0x3c {
            if 0 < DAT_100b4e60 {
                DAT_100b4e64 = DAT_100b4e60.wrapping_add(1);
            }
            local_8 = FUN_100473bb(param_1);
            base4
                .byte_offset(DAT_100b4e68.wrapping_mul(8) as isize)
                .cast::<i32>()
                .write_unaligned(DAT_100b4e64 as i32);
            base0
                .byte_offset(DAT_100b4e68.wrapping_mul(8) as isize)
                .cast::<i32>()
                .write_unaligned(s_var1 as i32);
            DAT_100b4e68 = DAT_100b4e68.wrapping_add(1);
        }
        local_8
    }
}
