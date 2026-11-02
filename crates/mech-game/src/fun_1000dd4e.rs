// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7584: i32;
    static mut DAT_101f64c0: [i8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000dd4e(param_1: i32) -> u32 {
    let mut local_8: u32 = 0;
    // SAFETY: the globals are the game's own memory; the table index is
    // bounds-checked against DAT_100a7584 before the read.
    unsafe {
        if param_1 < DAT_100a7584 && -1 < param_1 {
            local_8 = (&raw mut DAT_101f64c0)
                .cast::<u8>()
                .byte_add((param_1 as u32).wrapping_mul(0x7c) as usize)
                .cast::<u32>()
                .read_unaligned();
        }
    }
    local_8
}
