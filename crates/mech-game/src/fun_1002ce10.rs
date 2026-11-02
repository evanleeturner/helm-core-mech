// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1016a654: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002ce10(param_1: i32) -> u32 {
    let mut local_8: u32 = 0;
    if -1 < param_1 && param_1 < 10 {
        // SAFETY: the global table is the game's own memory; the index is
        // bounds-checked to 0..10 and each entry is 8 bytes wide.
        unsafe {
            local_8 = (&raw mut DAT_1016a654)
                .cast::<u8>()
                .byte_add(param_1.wrapping_mul(8) as usize)
                .cast::<u32>()
                .read_unaligned();
        }
    }
    local_8
}
