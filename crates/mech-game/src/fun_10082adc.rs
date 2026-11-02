// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9f60: i32;
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_10215670: u8;
    static mut DAT_10215674: u8;
    static mut DAT_10215678: u8;
    static mut DAT_1021567c: u8;
    static mut DAT_10215688: u8;
    static mut DAT_1021568c: u8;
    static mut DAT_10215690: u8;
    static mut DAT_10215694: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_10083a59(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: *mut i32,
        param_5: *mut u32,
        param_6: *mut u32,
        param_7: *mut i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10082adc(param_1: u32, param_2: i32, param_3: u32) -> u32 {
    // SAFETY: all globals and callees are the game's own memory; the pointers
    // derived from them and from the table entry are read and written exactly
    // where the original does.
    unsafe {
        if param_2 < 0 {
            return 0xffffffff;
        }
        if param_2 >= DAT_100b9f60 {
            return 0xfffffffe;
        }
        if (param_3 & 0xe) != 0 {
            return 0xfffffff7;
        }
        let local_18 = (&raw mut DAT_100eef40)
            .cast::<u32>()
            .add(param_1 as usize)
            .read_unaligned() as i32;
        let off = param_2.wrapping_mul(0x54) as isize;
        let local_24 = (&raw mut DAT_10215670).byte_offset(off).cast::<i32>();
        if local_24.read_unaligned() == 0 {
            return 0xfffffffd;
        }
        let flags = (&raw mut DAT_10215694)
            .byte_offset(off)
            .cast::<u16>()
            .read_unaligned();
        if !((param_3 & 0x10000) == 0 || (flags & 0x40) != 0) {
            return 0xfffffffc;
        }
        let flags = (&raw mut DAT_10215694)
            .byte_offset(off)
            .cast::<u16>()
            .read_unaligned();
        if (flags & 1) == 0 {
            if (param_3 & 0x100) != 0 {
                return 0xfffffffa;
            }
        } else if (&raw mut DAT_10215678)
            .byte_offset(off)
            .cast::<u32>()
            .read_unaligned()
            != (param_1 | 0x200)
        {
            return 0xfffffffa;
        }
        let p = local_18 as usize as *mut u8;
        let a = p.byte_add(8).cast::<i32>().read_unaligned();
        let b = (&raw mut DAT_1021567c)
            .byte_offset(off)
            .cast::<i32>()
            .read_unaligned();
        if a != b {
            return 0xfffffffa;
        }
        let t = (&raw mut DAT_10215674)
            .byte_offset(off)
            .cast::<i32>()
            .read_unaligned();
        if t != 0 {
            FUN_1002f322(
                (&raw mut DAT_10215674)
                    .byte_offset(off)
                    .cast::<i32>()
                    .read_unaligned(),
                (&raw mut DAT_10215688).byte_offset(off).cast::<u32>(),
                (&raw mut DAT_1021568c).byte_offset(off).cast::<u32>(),
                (&raw mut DAT_10215690).byte_offset(off).cast::<u32>(),
            );
        }
        let i_var1 = local_24.add(6).read_unaligned();
        let local_8 = local_24.add(7).read_unaligned();
        let local_10 = local_24.add(8).read_unaligned();
        let local_c =
            (local_10 as u32).wrapping_sub(p.byte_add(0x58).cast::<i32>().read_unaligned() as u32);
        let mut local_20: i32 = 0;
        let mut local_28: u32 = 0;
        let mut local_1c: u32 = 0;
        let mut local_14: i32 = 0;
        FUN_10083a59(
            (i_var1 as u32).wrapping_sub(p.byte_add(0x50).cast::<i32>().read_unaligned() as u32),
            (local_8 as u32).wrapping_sub(p.byte_add(0x54).cast::<i32>().read_unaligned() as u32),
            local_c,
            &raw mut local_20,
            &raw mut local_28,
            &raw mut local_1c,
            &raw mut local_14,
        );
        p.byte_add(200).cast::<i32>().write_unaligned(i_var1);
        p.byte_add(0xcc).cast::<i32>().write_unaligned(local_8);
        p.byte_add(0xd0).cast::<i32>().write_unaligned(local_10);
        p.byte_add(0xd4).cast::<i32>().write_unaligned(local_20);
        p.byte_add(0xc4).cast::<u32>().write_unaligned(local_28);
        p.byte_add(0xc0).cast::<u32>().write_unaligned(local_1c);
        p.byte_add(0xd8).cast::<i32>().write_unaligned(local_14);
        p.byte_add(0xf0).cast::<i32>().write_unaligned(i_var1);
        p.byte_add(0xf4).cast::<i32>().write_unaligned(local_8);
        p.byte_add(0xf8).cast::<i32>().write_unaligned(local_10);
        p.byte_add(0xfc).cast::<i32>().write_unaligned(local_20);
        p.byte_add(0xec).cast::<u32>().write_unaligned(local_28);
        p.byte_add(0xe8).cast::<u32>().write_unaligned(local_1c);
        p.byte_add(0x100).cast::<i32>().write_unaligned(local_14);
        1
    }
}
