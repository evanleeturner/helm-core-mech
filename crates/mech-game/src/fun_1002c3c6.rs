// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0710: u32;
    static mut DAT_1015f2e0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004cf27(param_1: i32, param_2: *mut i8) -> u32;
    fn FUN_1002ae60(param_1: i32, param_2: i16) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100082ad();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002c3c6(param_1: i32, param_2: i32) {
    mech_trace::enter!("1002c3c6", param_1, param_2);
    // SAFETY: the globals and callees are the game's own; the table offset
    // is computed as the original does.
    unsafe {
        if param_1 == -1 {
            mech_trace::cr!(
                "1002c3c6",
                "1004cf27",
                FUN_1004cf27(
                    mech_trace::gr!("1002c3c6", "100b0710", DAT_100b0710) as i32,
                    (&raw mut DAT_1015f2e0)
                        .cast::<u8>()
                        .byte_offset(param_2.wrapping_mul(0x1c) as isize)
                        .cast::<i8>(),
                )
            );
            FUN_100082ad();
        } else {
            mech_trace::cr!(
                "1002c3c6",
                "1002ae60",
                FUN_1002ae60(param_2, param_1 as i16)
            );
        }
    }
}
