// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: i32;
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_101600e0: u8;
    static mut DAT_101604d0: u8;
    static mut DAT_101604f0: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d0e5(param_1: i32, param_2: i32) -> u32 {
    let mut local_8: u32 = 0;
    // SAFETY: the globals are the game's own memory; offsets are computed
    // exactly as the original does and accessed unaligned.
    unsafe {
        let slot = (&raw mut DAT_101600e0)
            .byte_offset(param_1.wrapping_mul(0x3c) as isize)
            .cast::<u32>();
        if param_2 == -1 {
            slot.write_unaligned(0xffff_ffff);
        } else if param_2 < DAT_100b38d8 && param_1 < 0x10 {
            slot.write_unaligned(param_2 as u32);
            let row = param_1.wrapping_mul(0x70);
            let entry = (&raw const DAT_100eef40)
                .cast::<u32>()
                .offset(param_2 as isize)
                .read_unaligned();
            let idx = ((entry.wrapping_add(0xc)) as *const i32).read_unaligned();
            let off = idx.wrapping_mul(4).wrapping_add(row);
            let iv2 = (&raw mut DAT_101604d0)
                .byte_offset(off as isize)
                .cast::<i32>()
                .read_unaligned();
            let entry = (&raw const DAT_100eef40)
                .cast::<u32>()
                .offset(param_2 as isize)
                .read_unaligned();
            let idx = ((entry.wrapping_add(0xc)) as *const i32).read_unaligned();
            let off = idx.wrapping_mul(4).wrapping_add(row);
            let iv3 = (&raw mut DAT_101604f0)
                .byte_offset(off as isize)
                .cast::<i32>()
                .read_unaligned();
            let mut u_var1: u16 = 0;
            while u_var1 < 8 {
                let off = (u_var1 as i32).wrapping_mul(4).wrapping_add(row);
                let p = (&raw mut DAT_101604d0)
                    .byte_offset(off as isize)
                    .cast::<i32>();
                p.write_unaligned(p.read_unaligned().wrapping_sub(iv2));
                let q = (&raw mut DAT_101604f0)
                    .byte_offset(off as isize)
                    .cast::<i32>();
                q.write_unaligned(q.read_unaligned().wrapping_sub(iv3));
                u_var1 = u_var1.wrapping_add(1);
            }
            local_8 = 1;
        }
    }
    local_8
}
