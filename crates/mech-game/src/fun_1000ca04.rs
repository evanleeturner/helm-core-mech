// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f64bc: [i8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000ca04(param_1: i32) -> u32 {
    // SAFETY: the global array is the game's own memory; the offset record
    // is read at the computed byte offset from its base.
    unsafe {
        (&raw mut DAT_101f64bc)
            .cast::<u8>()
            .byte_add((param_1 as u32).wrapping_mul(0x7c) as usize)
            .cast::<u32>()
            .read_unaligned()
    }
}
