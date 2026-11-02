// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b4e5c: u32;
    static mut DAT_100cfad0: u8;
    static mut DAT_100cfb10: u8;
    static mut DAT_100cfb14: u8;
    static mut DAT_100cfb18: u8;
    static mut DAT_100cfb1c: u8;
    static mut DAT_100cfb20: u8;
    static mut DAT_100cfb24: u8;
    static mut DAT_100cfb28: u8;
    static mut DAT_100cfb2c: u8;
}

unsafe extern "C" {
    fn strncpy(dest: *mut i8, source: *const i8, count: usize) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003219a(param_1: i32) -> u32 {
    // SAFETY: the globals are the game's own tables; param_1 points at the
    // caller's record, read where the original does.
    unsafe {
        let i_var1 = DAT_100b4e5c;
        let u_var2: u32 = ((((param_1.wrapping_add(4)) as *const i32).read_unaligned()) as u32)
            .wrapping_sub(0x48)
            / 0x1c;
        if DAT_100b4e5c < 0x40 && u_var2 < 0x41 {
            DAT_100b4e5c = DAT_100b4e5c.wrapping_add(1);
            let base = (i_var1 as i32).wrapping_mul(0x744);
            ((&raw mut DAT_100cfb10 as *mut u8).byte_add(base as usize) as *mut u32)
                .write_unaligned(u_var2);
            strncpy(
                (&raw mut DAT_100cfad0 as *mut u8).byte_add(base as usize) as *mut i8,
                param_1.wrapping_add(8) as *const i8,
                0x40,
            );
            let mut local_c: i32 = 0;
            while local_c < u_var2 as i32 {
                let off = local_c.wrapping_mul(0x1c).wrapping_add(base) as usize;
                let src = local_c.wrapping_mul(0x1c);
                let rd = |o: i32| -> u32 {
                    (param_1.wrapping_add(o).wrapping_add(src) as *const u32).read_unaligned()
                };
                ((&raw mut DAT_100cfb14 as *mut u8).byte_add(off) as *mut u32)
                    .write_unaligned(rd(0x48));
                ((&raw mut DAT_100cfb18 as *mut u8).byte_add(off) as *mut u32)
                    .write_unaligned(rd(0x4c));
                ((&raw mut DAT_100cfb1c as *mut u8).byte_add(off) as *mut u32)
                    .write_unaligned(rd(0x50));
                ((&raw mut DAT_100cfb20 as *mut u8).byte_add(off) as *mut u32)
                    .write_unaligned(rd(0x54).wrapping_shl(0x10));
                ((&raw mut DAT_100cfb24 as *mut u8).byte_add(off) as *mut u32)
                    .write_unaligned(rd(0x58).wrapping_shl(0x10));
                ((&raw mut DAT_100cfb28 as *mut u8).byte_add(off) as *mut u32)
                    .write_unaligned(rd(0x5c).wrapping_shl(0x10));
                ((&raw mut DAT_100cfb2c as *mut u8).byte_add(off) as *mut u32)
                    .write_unaligned(rd(0x60));
                local_c = local_c.wrapping_add(1);
            }
            1
        } else {
            0
        }
    }
}
