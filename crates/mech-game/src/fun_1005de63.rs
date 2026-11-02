// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab238: u32;
    static mut DAT_100ab23c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001d916(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005de63() -> u32 {
    // SAFETY: the globals and the callee are the game's own; they are read,
    // written and called in the original's order.
    unsafe {
        DAT_100ab23c = DAT_100ab238;
        FUN_1001d916(DAT_100ab238);
    }
    0
}
