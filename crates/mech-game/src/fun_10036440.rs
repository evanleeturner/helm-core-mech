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
pub unsafe extern "C" fn FUN_10036440(param_1: i32) {
    mech_trace::enter!("10036440", param_1);
    if param_1 < 8 {
        // SAFETY: the global and the callee are the game's own; the write goes
        // to the slot the original computes (element offset param_1 * 4 from
        // the global's address).
        unsafe {
            let p = (&raw mut DAT_100adcf8).wrapping_offset(param_1.wrapping_mul(4) as isize);
            mech_trace::wr!("10036440", p, write_unaligned, 1);
            FUN_1002ae60(param_1, 3);
        }
    }
}
