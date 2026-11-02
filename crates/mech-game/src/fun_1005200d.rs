// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b09ec: u32;
    static mut DAT_101151c0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005200d() -> u32 {
    // SAFETY: the globals are the game's own memory; the table lookup reads
    // at the same byte offset from DAT_101151c0 as the original.
    unsafe {
        let i_var1 = (DAT_100b09ec as i32).wrapping_mul(4);
        let u_var2 = (&raw mut DAT_101151c0)
            .byte_offset(i_var1 as isize)
            .read_unaligned();
        DAT_100b09ec = DAT_100b09ec.wrapping_add(1);
        DAT_100b09ec = DAT_100b09ec % 0x7f;
        u_var2
    }
}
