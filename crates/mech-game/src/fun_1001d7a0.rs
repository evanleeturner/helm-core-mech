// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101e8db0: u32;
    static mut DAT_101e8db4: u32;
    static mut DAT_100ab0c8: i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001d7a0() {
    // SAFETY: the globals are the game's own memory; the table at
    // DAT_101e8db0 spans 0x960 dwords and the source table holds 0x2e
    // entries of four shorts, accessed exactly as the original does.
    unsafe {
        let mut p_var2 = &raw mut DAT_101e8db0;
        let mut i_var1: i32 = 0x960;
        while i_var1 != 0 {
            p_var2.write_unaligned(0xffff_ffffu32);
            p_var2 = p_var2.add(1);
            i_var1 = i_var1.wrapping_sub(1);
        }
        let mut local_c: *mut i16 = &raw mut DAT_100ab0c8;
        let mut local_8: u32 = 0;
        while local_8 < 0x2e {
            let i_var1 = local_c.read_unaligned() as i32;
            if 0 < i_var1 && i_var1 < 0x4b0 {
                let off = i_var1.wrapping_mul(8) as isize;
                let base0 = (&raw mut DAT_101e8db0).cast::<u8>();
                let base4 = (&raw mut DAT_101e8db4).cast::<u8>();
                base0
                    .offset(off)
                    .cast::<i16>()
                    .write_unaligned(local_c.add(1).read_unaligned());
                base0
                    .offset(off)
                    .add(2)
                    .cast::<i16>()
                    .write_unaligned(local_c.add(2).read_unaligned());
                base4
                    .offset(off)
                    .cast::<i16>()
                    .write_unaligned(local_c.add(3).read_unaligned());
                base4.offset(off).add(2).cast::<u16>().write_unaligned(0);
            }
            local_c = local_c.add(4);
            local_8 = local_8.wrapping_add(1);
        }
    }
}
