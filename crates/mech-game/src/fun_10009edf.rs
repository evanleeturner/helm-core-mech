// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b30: [u8; 0];
    static mut DAT_100aafb0: u32;
    static mut DAT_10215658: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d5e3(param_1: i32) -> u32;
    fn FUN_1004d60d(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10009edf(param_1: *mut u16, param_2: i32, param_3: u32) -> u32 {
    // SAFETY: the globals and callees are the game's own; param_1 and the
    // pointers reached through it are the caller's structures, read where
    // the original reads them.
    unsafe {
        let entry_ptr = (&raw mut DAT_100a7b30)
            .cast::<u8>()
            .byte_add(DAT_10215658.wrapping_mul(4) as usize)
            .cast::<i32>();
        let entry = entry_ptr.read_unaligned();
        if entry == 0 || param_1.is_null() || param_2 == 0 {
            return 0;
        }
        let mut local_14: u32 = (param_1.read_unaligned() & 0xf000) as u32;
        if local_14 == 0x8000 || local_14 == 0x9000 {
            return local_14;
        }
        let i_var1: i32 = ((entry as usize).wrapping_add(0x74) as *const i32).read_unaligned();
        local_14 = 0;
        if DAT_100aafb0 != 0 {
            local_14 = 0xf0;
        }
        let sub: *const u8 =
            param_1.byte_add(0x20).cast::<i32>().read_unaligned() as usize as *const u8;
        let u_var4: u16 = sub.byte_add(2).cast::<u16>().read_unaligned() & 0xf00;
        let b_var3: u8 = (param_3 >> 8) as u8;
        if u_var4 < 0x201 {
            if u_var4 == 0x200 {
                let w = sub.byte_add(0x14).cast::<u16>().read_unaligned();
                let i_var2 = FUN_1004d60d(w as u32 as i32) as i32;
                let addr = i_var1
                    .wrapping_add(0xc)
                    .wrapping_add(i_var2.wrapping_mul(4));
                return local_14 | (addr as usize as *const u32).read_unaligned();
            }
            if u_var4 == 0x100 {
                let w = sub.byte_add(0x14).cast::<u16>().read_unaligned();
                let i_var2 = FUN_1004d5e3(w as u32 as i32) as i32;
                let addr = i_var1.wrapping_add(i_var2.wrapping_mul(4));
                return local_14 | (addr as usize as *const u32).read_unaligned();
            }
        } else {
            if u_var4 == 0x400 {
                let addr = i_var1.wrapping_add(0x18);
                return local_14 | (addr as usize as *const u32).read_unaligned();
            }
            if u_var4 == 0x800 {
                let local_8: u32 = if (b_var3 & 0xf0) == 0x30 {
                    (i_var1.wrapping_add(0x28) as usize as *const u32).read_unaligned()
                } else {
                    (param_3 & 0xf00) >> 4
                };
                return local_8 | 0x4000;
            }
        }
        let sel = sub.byte_add(2).cast::<u16>().read_unaligned() & 0xf0;
        match sel {
            0x10 | 0x20 | 0x60 => {
                local_14 = param_3;
            }
            0x40 => {
                local_14 |= (i_var1.wrapping_add(0x1c) as usize as *const u32).read_unaligned();
            }
            0x50 => {
                local_14 |= (i_var1.wrapping_add(0x20) as usize as *const u32).read_unaligned();
            }
            0x80 => {
                local_14 |= (i_var1.wrapping_add(0x24) as usize as *const u32).read_unaligned();
            }
            _ => {
                let local_8: u32 = if (b_var3 & 0xf0) == 0x30 {
                    (i_var1.wrapping_add(0x28) as usize as *const u32).read_unaligned()
                } else {
                    (param_3 & 0xf00) >> 4
                };
                local_14 = local_8 | 0x4000;
            }
        }
        local_14
    }
}
