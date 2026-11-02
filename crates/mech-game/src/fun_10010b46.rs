// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c6588: u8;
    static mut DAT_100a9e8c: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(param_1: *mut u8, param_2: *mut u8, param_3: u32, param_4: u32) -> u32;
    fn FUN_1005dd5a(param_1: *mut u8);
    fn FUN_1006a38c(param_1: *mut u8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10010b46(param_1: u32, param_2: u32) -> *mut u8 {
    // SAFETY: the buffer, the format string and the callees are the game's own;
    // the calls run in the original's order on the same buffer.
    unsafe {
        _sprintf(
            &raw mut DAT_100c6588,
            &raw mut DAT_100a9e8c,
            param_1,
            param_2,
        );
        FUN_1005dd5a(&raw mut DAT_100c6588);
        FUN_1006a38c(&raw mut DAT_100c6588);
        &raw mut DAT_100c6588
    }
}
