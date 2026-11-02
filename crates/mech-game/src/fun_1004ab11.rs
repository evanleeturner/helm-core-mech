// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b06f4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004ab46(
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u32,
        a5: u32,
        a6: u32,
        a7: u32,
        a8: u32,
        a9: u32,
        a10: u32,
        a11: u32,
        a12: u32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004ab11(param_1: u32, param_2: u32) {
    // SAFETY: the global and the callee are the game's own; the call passes
    // the same arguments in the same order as the original.
    unsafe {
        DAT_100b06f4 = param_2;
        FUN_1004ab46(0xfffffffe, 0, param_1, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    }
}
