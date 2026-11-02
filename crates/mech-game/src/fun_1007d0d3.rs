// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c96b4: i32;
    static mut DAT_100c9ed8: u8;
    static mut DAT_100c9ee0: u8;
    static mut DAT_100c9ee4: u8;
    static mut DAT_100c9ee8: u8;
    static mut DAT_100c9eec: u8;
    static mut DAT_100c9ef0: u8;
    static mut DAT_100c9ef4: u8;
    static mut DAT_100c9ef8: u8;
    static mut DAT_100c9efc: u8;
    static mut DAT_100c9f00: u8;
    static mut DAT_100c9f04: u8;
    static mut DAT_100c9f08: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007d0d3(param_1: i32, param_2: u8, param_3: *mut u32) {
    // SAFETY: the globals are the game's own tables; param_3 points at the
    // caller's record of at least 11 dwords, and the table entries at
    // index param_1 are written where the original does.
    unsafe {
        if !param_3.is_null() && -1 < param_1 && param_1 < DAT_100c96b4 {
            let off = param_1.wrapping_mul(0x40) as usize;
            if (param_2 & 1) != 0 {
                (&raw mut DAT_100c9ee4)
                    .byte_add(off)
                    .cast::<u32>()
                    .write_unaligned(param_3.add(1).read_unaligned());
                (&raw mut DAT_100c9ee8)
                    .byte_add(off)
                    .cast::<u32>()
                    .write_unaligned(param_3.add(2).read_unaligned());
                (&raw mut DAT_100c9eec)
                    .byte_add(off)
                    .cast::<u32>()
                    .write_unaligned(param_3.add(3).read_unaligned());
            }
            if (param_2 & 2) != 0 {
                (&raw mut DAT_100c9ee0)
                    .byte_add(off)
                    .cast::<u32>()
                    .write_unaligned(param_3.read_unaligned());
            }
            if (param_2 & 4) != 0 {
                (&raw mut DAT_100c9f00)
                    .byte_add(off)
                    .cast::<u32>()
                    .write_unaligned(param_3.add(8).read_unaligned());
            }
            if (param_2 & 8) == 0 {
                (&raw mut DAT_100c9f08)
                    .byte_add(off)
                    .cast::<u32>()
                    .write_unaligned(0xffffffff);
            } else {
                (&raw mut DAT_100c9f08)
                    .byte_add(off)
                    .cast::<u32>()
                    .write_unaligned(param_3.add(10).read_unaligned());
            }
            if (param_2 & 0x10) != 0 {
                (&raw mut DAT_100c9f04)
                    .byte_add(off)
                    .cast::<u32>()
                    .write_unaligned(param_3.add(9).read_unaligned());
            }
            if (param_2 & 0x20) != 0 {
                (&raw mut DAT_100c9ef0)
                    .byte_add(off)
                    .cast::<u32>()
                    .write_unaligned(param_3.add(4).read_unaligned());
                (&raw mut DAT_100c9ef4)
                    .byte_add(off)
                    .cast::<u32>()
                    .write_unaligned(param_3.add(5).read_unaligned());
                (&raw mut DAT_100c9ef8)
                    .byte_add(off)
                    .cast::<u32>()
                    .write_unaligned(param_3.add(6).read_unaligned());
            }
            if (param_2 & 0x40) != 0 {
                (&raw mut DAT_100c9efc)
                    .byte_add(off)
                    .cast::<u32>()
                    .write_unaligned(param_3.add(7).read_unaligned());
            }
            if (param_2 & 0x80) == 0 {
                (&raw mut DAT_100c9ed8)
                    .byte_add(off)
                    .cast::<u32>()
                    .write_unaligned(0);
            } else {
                (&raw mut DAT_100c9ed8)
                    .byte_add(off)
                    .cast::<u32>()
                    .write_unaligned(1);
            }
        }
    }
}
