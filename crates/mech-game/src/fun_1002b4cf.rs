// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ad1c0: u8;
    static mut DAT_100ad200: u8;
    static mut DAT_100ad240: u8;
    static mut DAT_100ad280: u8;
    static mut DAT_100ad2c0: u8;
    static mut DAT_100ad300: u8;
    static mut DAT_1016a6a0: u8;
}

unsafe fn lookup(base: *mut u8, off: i32) -> u32 {
    // SAFETY: base and DAT_1016a6a0 are the game's own tables; the offsets
    // are read exactly as the original does.
    unsafe {
        let v = base
            .byte_offset(off as isize)
            .cast::<i16>()
            .read_unaligned() as i32;
        (&raw mut DAT_1016a6a0)
            .byte_offset(v.wrapping_mul(4) as isize)
            .cast::<u32>()
            .read_unaligned()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002b4cf(param_1: *mut u32, param_2: i16, param_3: i16, param_4: i16) {
    // SAFETY: param_1 points at the caller's record; the globals are the
    // game's own tables.
    unsafe {
        let dst = param_1.offset((param_2 as isize).wrapping_add(0x52));
        if param_3 == -1 {
            dst.write_unaligned(0);
        } else {
            let local_8: u16 = (param_1.add(1).read_unaligned() != (param_4 as i32 as u32)) as u16;
            let off = (param_3 as i32)
                .wrapping_mul(4)
                .wrapping_add((local_8 as i16 as i32).wrapping_mul(2));
            let base: *mut u8 = match param_1.read_unaligned() {
                1 => &raw mut DAT_100ad1c0,
                2 => &raw mut DAT_100ad280,
                3 => &raw mut DAT_100ad200,
                4 => &raw mut DAT_100ad2c0,
                5 => &raw mut DAT_100ad300,
                6 => &raw mut DAT_100ad240,
                8 => &raw mut DAT_100ad300,
                _ => return,
            };
            dst.write_unaligned(lookup(base, off));
        }
    }
}
