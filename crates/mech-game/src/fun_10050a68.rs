// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0864: i32;
    static mut DAT_101155e8: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10050a68(param_1: i32) -> u32 {
    let mut local_8: u32 = 0;
    // SAFETY: the globals are the game's own memory; the table index is
    // bounds-checked against DAT_100b0864 before the read.
    unsafe {
        if param_1 < DAT_100b0864 && -1 < param_1 {
            local_8 = (&raw mut DAT_101155e8)
                .cast::<u8>()
                .byte_add(param_1.wrapping_mul(0x48) as usize)
                .cast::<u32>()
                .read_unaligned();
        }
    }
    local_8
}
