// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b392c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10036a72(param_1: i32);
    fn FUN_10036a37(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10065d43() {
    // SAFETY: the callees and the global are the game's own; the global is
    // read by value each time, as the original does.
    unsafe {
        FUN_10036a72(DAT_100b392c);
        loop {
            let i_var1 = FUN_10036a37(DAT_100b392c);
            if i_var1 >= 0x5b {
                break;
            }
        }
    }
}
