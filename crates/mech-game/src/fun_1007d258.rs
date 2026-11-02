// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c96b4: i32;
    static mut DAT_100c9ed8: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007d258(param_1: i32, param_2: i32) {
    // SAFETY: the globals are the game's own memory; the index is bounds
    // checked against DAT_100c96b4 before the 0x40-byte stride write.
    unsafe {
        if -1 < param_1 && param_1 < DAT_100c96b4 {
            let p = (&raw mut DAT_100c9ed8)
                .cast::<u8>()
                .byte_offset((param_1.wrapping_mul(0x40)) as isize)
                .cast::<u32>();
            if param_2 == 0 {
                p.write_unaligned(0);
            } else {
                p.write_unaligned(1);
            }
        }
    }
}
