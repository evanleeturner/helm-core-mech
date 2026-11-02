// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab2d4: *mut u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001ead0(param_1: i32, param_2: *mut i8, param_3: u16, param_4: *mut u8) -> u32;
    fn FUN_10071045(param_1: *mut u32, param_2: i32) -> u32;
    fn FUN_10071107(param_1: *mut u32, param_2: *mut u8, param_3: u32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100710f7();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001ec56(
    param_1: i32,
    param_2: *mut i8,
    param_3: u16,
    param_4: *mut u8,
) -> u32 {
    if param_3 < 0xf000 {
        // SAFETY: the callee is the game's own function; arguments are passed through unchanged.
        return unsafe { FUN_1001ead0(param_1, param_2, param_3, param_4) };
    }
    // SAFETY: the callees and the global are the game's own; the stream pointer
    // is read from the game's global and passed on as the original does.
    unsafe {
        let s_var2 = FUN_10071045(DAT_100ab2d4, (param_3 as i32).wrapping_sub(0xf000));
        if s_var2 == 0xffffffff {
            0xffffffff
        } else {
            let s_var3 = FUN_10071107(DAT_100ab2d4, param_4, s_var2);
            FUN_100710f7();
            if s_var3 == s_var2 { 0 } else { 0xffffffff }
        }
    }
}
