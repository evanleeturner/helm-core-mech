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
pub unsafe extern "C" fn FUN_100364d3(param_1: i32) {
    if param_1 < 8 {
        // SAFETY: the global array and the callee are the game's own; the
        // element pointer scales param_1 * 4 by the int size, as the C does.
        unsafe {
            let p = (&raw mut DAT_100adcf8).wrapping_offset(param_1.wrapping_mul(4) as isize);
            p.write_unaligned(5);
            FUN_1002ae60(param_1, 8);
        }
    }
}
