// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adcf8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002ae60(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036504(param_1: i32) {
    if param_1 < 8 {
        // SAFETY: the global and callee are the game's own; the C adds
        // param_1 * 4 to an `int *`, so the offset is in 4-byte elements,
        // and the write lands where the original's does.
        unsafe {
            let p = (&raw mut DAT_100adcf8).wrapping_offset(param_1.wrapping_mul(4) as isize);
            p.write_unaligned(6);
            FUN_1002ae60(param_1, 0xb);
        }
    }
}
