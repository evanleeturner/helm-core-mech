// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1f4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10036b20(param_1: u32, param_2: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1006ae8d();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100511f2() {
    // SAFETY: the global and the callees are the game's own; called in the
    // original's order with the original's arguments.
    unsafe {
        if DAT_100aa1f4 == 0 {
            FUN_10036b20(0x80, 1);
            FUN_1006ae8d();
        }
    }
}
