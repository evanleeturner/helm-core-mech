// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c6b28: [i8; 1];
    static mut s__2_2d__2_2d__2_2d_100ad6fc: [i8; 1];
}

unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002d415(param_1: i32) -> *mut u8 {
    // SAFETY: the globals and sprintf are the game's own; the buffer is the
    // game's formatting buffer, written by sprintf and returned as is.
    unsafe {
        let i_var1 = (param_1 % 0xe10) / 0x3c;
        sprintf(
            &raw mut DAT_100c6b28 as *mut i8,
            &raw const s__2_2d__2_2d__2_2d_100ad6fc as *const i8,
            param_1 / 0xe10,
            i_var1,
            (param_1 % 0xe10).wrapping_add(i_var1.wrapping_mul(-0x3c)),
        );
        &raw mut DAT_100c6b28 as *mut u8
    }
}
