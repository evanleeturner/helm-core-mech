// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10113170: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005f7ad(param_1: i16, param_2: u16) {
    let off = (param_1 as i32).wrapping_mul(0xe) as isize;
    // SAFETY: DAT_10113170 is the game's own table; entries are read and
    // written at the same offsets the original uses.
    unsafe {
        let base = (&raw mut DAT_10113170).cast::<u8>();
        if base.offset(off).byte_add(8).cast::<i16>().read_unaligned() != -2 {
            base.offset(off)
                .byte_add(6)
                .cast::<u16>()
                .write_unaligned(param_2);
        }
        if base.offset(off).byte_add(4).cast::<i16>().read_unaligned() == 0 {
            base.offset(off)
                .byte_add(4)
                .cast::<u16>()
                .write_unaligned(0x2d);
        }
    }
}
