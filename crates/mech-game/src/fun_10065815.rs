// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7e78: u32;
    static mut DAT_100c7e7c: u32;
    static mut DAT_100c7e80: u32;
    static mut DAT_100c7f88: u32;
    static mut DAT_100c7f8c: unsafe extern "C" fn(u32, u32) -> i32;
    static mut DAT_100c7f90: u32;
    static mut DAT_100c7f94: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10065815() -> i32 {
    // SAFETY: the globals and the callback are the game's own memory; they are
    // read and written in the order the original does.
    unsafe {
        let u_var1: i32;
        if DAT_100c7e7c < DAT_100c7f88 {
            let cb = DAT_100c7f8c;
            u_var1 = cb(
                DAT_100c7f90.wrapping_add(DAT_100c7e80),
                DAT_100c7e7c.wrapping_add(DAT_100c7e78),
            );
            DAT_100c7e80 = DAT_100c7e80.wrapping_add(1);
            if DAT_100c7f94 <= DAT_100c7e80 {
                DAT_100c7e80 = 0;
                DAT_100c7e7c = DAT_100c7e7c.wrapping_add(1);
            }
        } else {
            u_var1 = -1;
        }
        u_var1
    }
}
