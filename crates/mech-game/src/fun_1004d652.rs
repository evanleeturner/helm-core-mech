// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101600f4: u8;
    static mut DAT_1021567c: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d652(param_1: u32) -> u32 {
    mech_trace::enter!("1004d652", param_1);
    let mut local_8: u32 = 2;
    if param_1 < 0x10 {
        // SAFETY: both globals are the game's own tables; the offsets are
        // computed as the original does and read unaligned.
        unsafe {
            let idx = mech_trace::rd!(
                "1004d652",
                (&raw mut DAT_1021567c)
                    .byte_add(param_1.wrapping_mul(0x54) as usize)
                    .cast::<i32>(),
                read_unaligned
            );
            local_8 = mech_trace::rd!(
                "1004d652",
                (&raw mut DAT_101600f4)
                    .byte_add(idx.wrapping_mul(0x3c) as usize)
                    .cast::<u32>(),
                read_unaligned
            );
        }
    }
    mech_trace::rt!("1004d652", local_8)
}
