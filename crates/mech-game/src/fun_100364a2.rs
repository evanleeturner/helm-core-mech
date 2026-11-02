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
pub unsafe extern "C" fn FUN_100364a2(param_1: i32) {
    mech_trace::enter!("100364a2", param_1);
    if param_1 < 8 {
        // SAFETY: the global array and the callee are the game's own; the
        // write lands at the same element offset (param_1 * 4 ints) as the original.
        unsafe {
            let p = (&raw mut DAT_100adcf8).wrapping_offset(param_1.wrapping_mul(4) as isize);
            mech_trace::wr!("100364a2", p, write_unaligned, 4);
            FUN_1002ae60(param_1, 7);
        }
    }
}
