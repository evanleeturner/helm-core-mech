// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7bc8: u8;
    static mut DAT_100a7bcc: u8;
    static mut DAT_100aa1ac: u32;
    static mut DAT_100f2fb0: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100635ce(param_1: *mut i32) {
    // SAFETY: the globals are the game's own memory; param_1 and the
    // structures it points to are the caller's, accessed at the original's offsets.
    unsafe {
        let flags = &raw mut DAT_100f2fb0;
        let tab_c8 = &raw mut DAT_100a7bc8;
        let tab_cc = &raw mut DAT_100a7bcc;
        let mut local_10: i32 = 0;
        while local_10 < 0xe {
            let slot = flags
                .byte_add(local_10.wrapping_mul(4) as usize)
                .cast::<i32>();
            if slot.read_unaligned() != 0 {
                slot.write_unaligned(0);
                let i_var2 = param_1
                    .add(0x15)
                    .read_unaligned()
                    .wrapping_add(local_10.wrapping_mul(0x40));
                let p = i_var2 as u32 as usize as *mut u8;
                let i_var1 = p
                    .byte_add(4)
                    .cast::<i32>()
                    .read_unaligned()
                    .wrapping_mul(0x68);
                if p.byte_add(8).cast::<i32>().read_unaligned() != -1 || DAT_100aa1ac != 0 {
                    p.byte_add(8).cast::<u32>().write_unaligned(2);
                    let v = tab_cc
                        .byte_offset(i_var1 as isize)
                        .cast::<u32>()
                        .read_unaligned();
                    p.byte_add(0x24).cast::<u32>().write_unaligned(v);
                    p.byte_add(0x10).cast::<u32>().write_unaligned(0);
                    let w = tab_c8
                        .byte_offset(i_var1 as isize)
                        .cast::<i32>()
                        .read_unaligned();
                    let f = param_1.add(0x110).cast::<u16>().read_unaligned();
                    if w == 0 || (f & 0x80) == 0 {
                        p.byte_add(0x1c).cast::<u32>().write_unaligned(0);
                        let z = p.byte_add(0x1c).cast::<u32>().read_unaligned();
                        p.byte_add(0x18).cast::<u32>().write_unaligned(z);
                    } else {
                        let q = (param_1.read_unaligned().wrapping_add(0xdc)) as u32 as usize
                            as *const u32;
                        p.byte_add(0x18)
                            .cast::<u32>()
                            .write_unaligned(q.read_unaligned() & 0xff);
                        let q2 = (param_1.read_unaligned().wrapping_add(0xdc)) as u32 as usize
                            as *const u32;
                        p.byte_add(0x1c)
                            .cast::<u32>()
                            .write_unaligned(q2.read_unaligned() & 0xf00);
                    }
                }
            }
            local_10 = local_10.wrapping_add(1);
        }
    }
}
