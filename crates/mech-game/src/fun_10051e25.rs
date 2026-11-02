// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b09dc: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100659d3();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10051e25() {
    // SAFETY: the global and the callee are the game's own memory and code.
    unsafe {
        DAT_100b09dc = 3;
        FUN_100659d3();
    }
}
