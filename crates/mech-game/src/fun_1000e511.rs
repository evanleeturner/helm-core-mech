// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a75e4: u32;
    static mut DAT_100b3c9c: *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000e511(param_1: u32, param_2: u32) {
    let _ = param_1;
    // SAFETY: the globals are the game's own; DAT_100b3c9c points at a
    // structure with a writable u32 at offset 0x24.
    unsafe {
        DAT_100a75e4 = param_2;
        DAT_100b3c9c
            .byte_add(0x24)
            .cast::<u32>()
            .write_unaligned(param_2);
    }
}
