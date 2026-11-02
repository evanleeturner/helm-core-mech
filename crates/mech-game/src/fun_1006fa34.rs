// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9a2c: *mut u32;
    static mut DAT_100b9d30: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006fa34() -> u32 {
    // SAFETY: the globals are the game's own memory; DAT_100b9a2c holds a
    // pointer to a writable u32, written where the original does.
    unsafe {
        DAT_100b9a2c.write_unaligned(DAT_100b9d30);
    }
    0
}
