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
pub unsafe extern "C" fn FUN_1003640f(param_1: i32) {
    mech_trace::enter!("1003640f", param_1);
    if param_1 < 8 {
        // SAFETY: the global and the callee are the game's own; the C scales
        // the index by 4 ints (16 bytes) through int pointer arithmetic, kept here.
        unsafe {
            let p = (&raw mut DAT_100adcf8).wrapping_offset(param_1.wrapping_mul(4) as isize);
            mech_trace::wr!("1003640f", p, write_unaligned, 2);
            FUN_1002ae60(param_1, 2);
        }
    }
}
