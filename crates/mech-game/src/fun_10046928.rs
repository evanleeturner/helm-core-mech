// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04c4: i32;
    static mut DAT_100c7d00: i32;
    static mut DAT_100b04b8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10046563() -> i32;
    fn FUN_10046f04(param_1: i32);
    fn FUN_10046dbc(param_1: *mut i32) -> i32;
    fn __putch(c: i32) -> i32;
    fn _exit(code: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10046928() -> i32 {
    // SAFETY: the globals and callees are the game's own; the pointer passed
    // to FUN_10046dbc is the address of the game's global.
    unsafe {
        if DAT_100b04c4 == 0 {
            return 0;
        }
        let i_var1 = FUN_10046563();
        if DAT_100c7d00 == i_var1 {
            return 0;
        }
        match i_var1 {
            1 => {
                FUN_10046f04(0x100b04b8);
            }
            2 => {
                if DAT_100c7d00 == 1 {
                    let mut local_c: i32 = 0;
                    while local_c < 3 && FUN_10046dbc(&raw mut DAT_100b04b8) != 0 {
                        __putch(7);
                        local_c = local_c.wrapping_add(1);
                    }
                    if local_c == 3 {
                        _exit(0);
                    }
                }
            }
            _ => {}
        }
        DAT_100c7d00 = i_var1;
        1
    }
}
