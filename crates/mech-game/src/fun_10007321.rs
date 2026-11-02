// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a41f0: i8;
}

unsafe extern "C" {
    fn strncpy(dest: *mut i8, src: *const i8, n: u32) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10007321(source: *const i8) {
    // SAFETY: the debug log buffer is the game's own memory, at least 0x100
    // bytes; source is the caller's string, copied as the original does.
    unsafe {
        strncpy(&raw mut DAT_100a41f0, source, 0x100);
    }
}
