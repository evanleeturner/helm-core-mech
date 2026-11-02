// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009390c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001a730(param_1: i32, param_2: i32, param_3: i32, param_4: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10036260() {
    // SAFETY: the callee and the global are the game's own; the second
    // argument is the fixed address 0x10093914 passed by value.
    unsafe {
        FUN_1001a730(DAT_1009390c, 0x10093914, 0, core::ptr::null_mut());
    }
}
