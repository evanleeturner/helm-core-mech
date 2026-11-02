// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa6bc: i32;
    static mut DAT_100aa6b8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006dbc0(param_1: i32) -> i32 {
    // SAFETY: the globals are the game's own, and the addresses computed from
    // param_1 and the globals point at the game's structures, read where the
    // original reads.
    unsafe {
        let idx = ((param_1.wrapping_add(0x364)) as *const i32)
            .read_unaligned()
            .wrapping_mul(0x12);
        let a = (DAT_100aa6bc.wrapping_add(0x760).wrapping_add(idx)) as *const i32;
        let b = (DAT_100aa6b8.wrapping_add(0x760).wrapping_add(idx)) as *const i32;
        let i_var1 = a.read_unaligned().wrapping_sub(b.read_unaligned());
        (i_var1 & (0xFFFFFF00u32 as i32)) | ((i_var1 < 1) as i32)
    }
}
