// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c9c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002cd87(param_1: u32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10081106(param_1: u32, param_2: i32) {
    let _ = param_1;
    // SAFETY: the callee and the global are the game's own; DAT_100b3c9c holds
    // a pointer to a structure whose field at offset 0x14 is written.
    unsafe {
        FUN_1002cd87(0x100, param_2);
        FUN_1002cd87(0x200, param_2);
        ((DAT_100b3c9c as usize + 0x14) as *mut i32).write_unaligned(param_2);
    }
}
