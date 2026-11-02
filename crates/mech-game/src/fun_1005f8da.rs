// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b351c: u32;
    static mut DAT_100f3170: u8;
    static mut DAT_100f3172: u8;
    static mut DAT_100f3174: u8;
    static mut DAT_10113170: u8;
    static mut DAT_10113172: u8;
    static mut DAT_10113174: u8;
    static mut DAT_10113176: u8;
    static mut DAT_10113178: u8;
    static mut DAT_1011317a: u8;
    static mut DAT_100af0ac: u32;
    static mut DAT_100af100: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005f8da() {
    // SAFETY: the globals and the callee are the game's own; the tables are
    // the game's arrays, accessed at the same offsets as the original.
    unsafe {
        let mut local_10: i32 = 0;
        while local_10 < 0x200 {
            let mut local_14: i32 = 0;
            while local_14 < 0x20 {
                let i_var1 = local_10
                    .wrapping_mul(0x100)
                    .wrapping_add(local_14.wrapping_mul(8));
                let p0 = (&raw mut DAT_100f3170).byte_add(i_var1 as usize);
                let p2 = (&raw mut DAT_100f3172).byte_add(i_var1 as usize);
                let p4 = (&raw mut DAT_100f3174).byte_add(i_var1 as usize);
                if p4.cast::<i32>().read_unaligned() != 0 {
                    FUN_1008664f(
                        p0.cast::<i16>().read_unaligned() as i32 as u32,
                        DAT_100af0ac as usize as *mut i32,
                    );
                    p4.cast::<i32>().write_unaligned(0);
                }
                p0.cast::<i16>().write_unaligned(0);
                p2.cast::<u16>().write_unaligned(0);
                local_14 = local_14.wrapping_add(1);
            }
            local_10 = local_10.wrapping_add(1);
        }
        FUN_1008664f(DAT_100b351c, DAT_100af100 as usize as *mut i32);
        local_10 = 0;
        while local_10 < 0x200 {
            let i_var1 = local_10.wrapping_mul(0xe) as usize;
            (&raw mut DAT_10113170)
                .byte_add(i_var1)
                .cast::<u16>()
                .write_unaligned(0xffff);
            (&raw mut DAT_10113172)
                .byte_add(i_var1)
                .cast::<u16>()
                .write_unaligned(0);
            (&raw mut DAT_10113174)
                .byte_add(i_var1)
                .cast::<u16>()
                .write_unaligned(0);
            (&raw mut DAT_10113176)
                .byte_add(i_var1)
                .cast::<u16>()
                .write_unaligned(0);
            (&raw mut DAT_10113178)
                .byte_add(i_var1)
                .cast::<u16>()
                .write_unaligned(0xfffe);
            (&raw mut DAT_1011317a)
                .byte_add(i_var1)
                .cast::<u32>()
                .write_unaligned(0xffffffff);
            local_10 = local_10.wrapping_add(1);
        }
    }
}
