// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c9c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003c13a(param_1: u32, param_2: i32);
    fn FUN_1002cdde(param_1: u32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100811fd(param_1: u32, param_2: i32) {
    // SAFETY: the callees and the global are the game's own; DAT_100b3c9c
    // holds a pointer to a structure whose field at offset 0x1c is written.
    unsafe {
        FUN_1003c13a(param_1, param_2);
        FUN_1002cdde(param_1, param_2);
        (DAT_100b3c9c as usize as *mut u8)
            .byte_add(0x1c)
            .cast::<i32>()
            .write_unaligned(param_2);
    }
}
