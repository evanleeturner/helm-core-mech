// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008da7c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000f170() -> u32 {
    // SAFETY: the global is the game's own memory, read by value.
    unsafe {
        if (DAT_1008da7c & 0x80000000) != 0 {
            if (DAT_1008da7c & 0x1000) != 0 {
                return 1;
            }
        }
    }
    0
}
