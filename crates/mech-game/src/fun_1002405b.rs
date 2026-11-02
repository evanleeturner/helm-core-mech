// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab374: i32;
    static mut DAT_1016ce56: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10023a8c(param_1: i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002405b(param_1: i32) {
    // SAFETY: the globals and the callee are the game's own; the write goes to
    // the game's table entry at the offset the original computes.
    unsafe {
        if 0 < param_1 && param_1 < DAT_100ab374 {
            (&raw mut DAT_1016ce56)
                .byte_add(param_1.wrapping_mul(0x1fa) as usize)
                .cast::<u32>()
                .write_unaligned(0);
            FUN_10023a8c(param_1);
        }
    }
}
