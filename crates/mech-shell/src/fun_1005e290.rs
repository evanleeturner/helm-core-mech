// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10095b00: u32;
    static mut DAT_100a50f0: u32;
    static mut DAT_100a62f4: u32;
    static mut DAT_100c6064: u32;
    static mut DAT_100c6068: u8;
    static mut DAT_100c61b4: *mut u32;
}

unsafe extern "C" {
    #[link_name = "strncpy"]
    fn c_strncpy(dest: *mut i8, src: *const i8, count: usize) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005e290(
    param_1: i32,
    param_2: *mut i8,
    param_3: *mut i8,
    param_4: *mut i8,
    param_5: i32,
    param_6: *mut i8,
    param_7: i32,
    param_8: u16,
    param_9: u16,
    param_10: u16,
    param_11: i32,
    param_12: u16,
    param_13: u16,
) {
    let mut param_7 = param_7;
    let mut param_11 = param_11;
    // SAFETY: the globals and the record buffer are the game's own memory;
    // DAT_100c61b4 points at a record with room for 100 bytes, and the
    // string pointers are the caller's.
    unsafe {
        let p = DAT_100c61b4;
        let b2 = DAT_100c6064 == 0;
        if b2 {
            DAT_100c6064 = 1;
        }
        DAT_100a62f4 = b2 as u32;
        let dest = p.wrapping_add(9) as *mut i8;
        p.byte_add(10).cast::<u16>().write_unaligned(0xffff);
        c_strncpy(dest, param_4, 8);
        p.byte_add(8).cast::<u16>().write_unaligned(0xfffe);
        if param_5 == 0 {
            p.byte_add(8).cast::<u16>().write_unaligned(0xffff);
        }
        c_strncpy(p.byte_add(0x2d).cast::<i8>(), param_6, 8);
        p.byte_add(12).cast::<u8>().write(DAT_100c6068);
        if param_1 == 0 && DAT_100a62f4 == 0 {
            p.byte_add(0xd).cast::<u8>().write(0);
        } else {
            p.byte_add(0xd).cast::<u8>().write(1);
        }
        p.byte_add(0xe).cast::<u8>().write(0);
        if param_1 == 0 {
            p.byte_add(0xe).cast::<u8>().write(2);
        }
        p.byte_add(0x20).cast::<u16>().write_unaligned(6);
        if param_7 < 1 || 5 < param_7 {
            param_7 = 5;
        }
        p.byte_add(0x10)
            .cast::<i16>()
            .write_unaligned(param_7 as i16);
        p.byte_add(0x12).cast::<u16>().write_unaligned(param_8);
        p.byte_add(0x14).cast::<u16>().write_unaligned(param_9);
        p.byte_add(0x16).cast::<u16>().write_unaligned(param_10);
        if param_11 < 1 || 5 < param_11 {
            param_11 = 5;
        }
        p.byte_add(0x18)
            .cast::<i16>()
            .write_unaligned(param_11 as i16);
        p.byte_add(0x1a).cast::<u16>().write_unaligned(param_12);
        p.byte_add(0x1c).cast::<u16>().write_unaligned(param_13);
        p.byte_add(0x1e).cast::<u16>().write_unaligned(0);
        c_strncpy(p.byte_add(0x36).cast::<i8>(), param_2, 0x15);
        c_strncpy(p.wrapping_add(0x13) as *mut i8, param_3, 0x15);
        p.byte_add(0x22).cast::<u16>().write_unaligned(0x1000);
        p.write_unaligned(DAT_10095b00);
        p.wrapping_add(1).write_unaligned(100);
        if DAT_100a50f0 < 0x65 {
            DAT_100a50f0 = 100;
        }
        let step = p.wrapping_add(1).read_unaligned();
        DAT_100c61b4 = DAT_100c61b4.wrapping_byte_add(step as usize);
    }
}
