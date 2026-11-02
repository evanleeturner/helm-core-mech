// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04c4: u32;
    static mut DAT_100c7cf0: u32;
    static mut DAT_100c7cf4: u32;
    static mut DAT_100c7cf8: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10046563() -> u32;
    fn FUN_100463c4();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10046824(param_1: *mut u32);
    fn FUN_100463ee(param_1: u32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10046cbe(param_1: u32, param_2: u32) {
    let mut local_14: [u32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; local_14 is a
    // valid buffer for the callee to fill.
    unsafe {
        if DAT_100b04c4 != 0 {
            let i_var1 = FUN_10046563() as i32;
            if i_var1 != 1 && 1 < i_var1 && i_var1 < 5 {
                FUN_10046824(local_14.as_mut_ptr());
                if param_1 < param_2 {
                    if i_var1 == 3 {
                        FUN_100463c4();
                        FUN_100463ee(param_1, param_2 as i32);
                    } else if i_var1 == 4 {
                        FUN_100463ee(param_1, param_2 as i32);
                        DAT_100c7cf0 = 0;
                        DAT_100c7cf4 = 0;
                        DAT_100c7cf8 = 0;
                    } else {
                        FUN_100463ee(param_1, param_2 as i32);
                    }
                }
            }
        }
    }
}
