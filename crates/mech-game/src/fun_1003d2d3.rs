// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: i32;
    static mut DAT_10160c00: i32;
    static mut DAT_100af098: i32;
    static mut DAT_10160c04: i32;
    static mut DAT_10160e60: i32;
    static mut DAT_10160e68: i32;
    static mut DAT_10160bf8: i32;
    static mut DAT_10160e6c: i32;
    static mut DAT_100c7ae0: i32;
    static mut DAT_100c7ae4: i32;
    static mut DAT_100c6d5c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003d2d3(param_1: i32, param_2: i32) {
    let mut local_18: i32 = 0;
    // SAFETY: all globals are the game's own memory; table and output
    // pointers are computed at the same byte offsets as the original.
    unsafe {
        if DAT_100aefc0 == 0 {
            DAT_10160c00 = DAT_100af098;
            DAT_10160c04 = 0;
        } else {
            DAT_10160e60 = param_1.wrapping_add(150000) / 25000;
            DAT_10160e68 = param_2.wrapping_add(150000) / 25000;
            DAT_10160bf8 = (((param_1.wrapping_add(150000) % 0xc) / 6).wrapping_mul(2))
                | ((param_2.wrapping_add(150000) % 0xc) / 6);
            DAT_10160e6c = 0;
            if DAT_100aefc0 == 0 {
                DAT_10160c00 = DAT_100af098;
                DAT_10160c04 = 0;
            } else {
                while DAT_10160e6c < 4 {
                    let off = DAT_10160e6c
                        .wrapping_mul(8)
                        .wrapping_add(DAT_10160bf8.wrapping_mul(0x20));
                    let i_var1 = (&raw mut DAT_100c7ae0)
                        .cast::<u8>()
                        .byte_offset(off as isize)
                        .cast::<i32>()
                        .read_unaligned()
                        .wrapping_add(DAT_10160e60);
                    let i_var2 = (&raw mut DAT_100c7ae4)
                        .cast::<u8>()
                        .byte_offset(off as isize)
                        .cast::<i32>()
                        .read_unaligned()
                        .wrapping_add(DAT_10160e68);
                    DAT_10160e6c = DAT_10160e6c.wrapping_add(1);
                    if -1 < i_var1 && i_var1 < 0xc && -1 < i_var2 && i_var2 < 0xc {
                        let eoff = i_var2
                            .wrapping_mul(0x18)
                            .wrapping_add(i_var1.wrapping_mul(0x120));
                        let entry = (&raw mut DAT_100c6d5c)
                            .cast::<u8>()
                            .byte_offset(eoff as isize)
                            .cast::<i32>()
                            .read_unaligned();
                        if (entry as u32 as usize as *const u8)
                            .byte_add(8)
                            .cast::<i32>()
                            .read_unaligned()
                            != 0
                        {
                            let ooff = local_18.wrapping_mul(4) as isize;
                            (&raw mut DAT_10160c00)
                                .cast::<u8>()
                                .byte_offset(ooff)
                                .cast::<i32>()
                                .write_unaligned(entry);
                            (&raw mut DAT_10160c04)
                                .cast::<u8>()
                                .byte_offset(ooff)
                                .cast::<i32>()
                                .write_unaligned(0);
                            local_18 = local_18.wrapping_add(1);
                        }
                    }
                }
            }
        }
    }
}
