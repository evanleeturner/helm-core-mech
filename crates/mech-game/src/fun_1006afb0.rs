// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003783(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006afb0(param_1: u32, param_2: u32, param_3: u32) {
    let _ = param_1;
    if param_2 > 0 {
        // SAFETY: the callee is the game's own function; arguments are plain values.
        unsafe {
            FUN_10003783(0, 0, param_2, 0x32, param_3, 0x32);
        }
    }
}
