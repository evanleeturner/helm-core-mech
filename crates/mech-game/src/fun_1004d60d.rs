// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ef03c: u8;
    static mut DAT_101604a0: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d60d(param_1: i32) -> u32 {
    // SAFETY: both globals are the game's own tables; addresses are taken
    // without references and read where the original reads.
    unsafe {
        let base = (&raw mut DAT_100ef03c) as *mut u8;
        let idx = base
            .byte_add(param_1.wrapping_mul(0x40) as usize)
            .cast::<i32>()
            .read_unaligned();
        if idx < 0 {
            2
        } else {
            let tbl = (&raw mut DAT_101604a0) as *mut u8;
            tbl.byte_add(idx.wrapping_mul(4) as usize)
                .cast::<u32>()
                .read_unaligned()
        }
    }
}
