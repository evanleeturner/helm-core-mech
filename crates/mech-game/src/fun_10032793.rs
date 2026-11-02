// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101600f0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004cf27(param_1: i32, param_2: *mut i8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10032793(param_1: i32) {
    mech_trace::enter!("10032793", param_1);
    let mut local_c: i32 = 0;
    while local_c < 0x10 {
        // SAFETY: the callee and the global table are the game's own memory;
        // param_1 is an address of the caller's buffer, and the table entry
        // at local_c * 0x3c is written where the original does.
        unsafe {
            mech_trace::cr!(
                "10032793",
                "1004cf27",
                FUN_1004cf27(
                    local_c,
                    local_c
                        .wrapping_mul(0x11)
                        .wrapping_add(param_1)
                        .wrapping_add(8) as *mut i8,
                )
            );
            mech_trace::wr!(
                "10032793",
                (&raw mut DAT_101600f0)
                    .byte_add(local_c.wrapping_mul(0x3c) as usize)
                    .cast::<u32>(),
                write_unaligned,
                0
            );
        }
        local_c = local_c.wrapping_add(1);
    }
}
