// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7178: i32;
    static mut DAT_100a7180: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(param_1: i32, param_2: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a37c(param_1: i32, param_2: i32) -> i32 {
    // SAFETY: param_1 points at the caller's struct; the callees and globals
    // are the game's own.
    unsafe {
        let field = (param_1 as usize as *const i32)
            .byte_add(0x18)
            .read_unaligned();
        let arg = field.wrapping_sub(param_2 >> 2).wrapping_sub(DAT_100a7178);
        let i_var1 = FUN_10002570(arg, DAT_100a7180);
        let mut local_8 = FUN_10003d60(i_var1, 0x10);
        if local_8 < 0 {
            local_8 = 0;
        } else if 0xf < local_8 {
            local_8 = 0xf;
        }
        local_8
    }
}
