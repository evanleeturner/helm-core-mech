// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: i32;
    static mut DAT_101686f0: i32;
    static mut DAT_100af0b4: i32;
    static mut DAT_100b0894: i32;
    static mut DAT_100b0880: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10065e40(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_100664ef(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_10056fc4(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10050fd0() {
    // SAFETY: the globals and callees are the game's own; values are passed
    // and written exactly where the original does.
    unsafe {
        let p_var1 = FUN_1008668b(DAT_100af17c, DAT_101686f0.wrapping_add(0x43), DAT_100af0b4);
        if p_var1 != 0 {
            if DAT_100b0894 != 0 {
                DAT_100b0880 = 0x101f5300;
                FUN_10065e40(0x101f5300, 0x100b0880, 0x100b0880);
                FUN_100664ef(0x100b0880, 0x100b0880, p_var1, 0);
                DAT_100b0894 = 0;
            }
            FUN_10056fc4(0x100b0880, p_var1, 0, 0, 0);
        }
    }
}
