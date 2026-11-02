// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab2d4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001e798(param_1: i32, param_2: *mut i8, param_3: u16) -> i32;
    fn FUN_10071002(param_1: i32, param_2: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001e8a8(param_1: i32, param_2: *mut i8, param_3: u16) -> i32 {
    if (param_3 as u32) == 0xffff_ffffu32 {
        -1
    } else if param_3 < 0xf000 {
        // SAFETY: the callee is the game's own function; arguments are passed through unchanged.
        unsafe { FUN_1001e798(param_1, param_2, param_3) }
    } else {
        // SAFETY: the callee and the global are the game's own; the global is read by value.
        unsafe { FUN_10071002(DAT_100ab2d4, (param_3.wrapping_add(0x1000)) as u32) }
    }
}
