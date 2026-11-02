// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c9c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003e425(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008124c(_param_1: u32, param_2: i32) {
    // SAFETY: the callee and the global are the game's own; DAT_100b3c9c holds
    // an address whose field at offset 0x20 is written where the original does.
    unsafe {
        FUN_1003e425(param_2);
        (DAT_100b3c9c.wrapping_add(0x20) as *mut i32).write_unaligned(param_2);
    }
}
