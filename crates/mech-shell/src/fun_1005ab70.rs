// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static s_MW2DIF_CFG_100a50c0: [i8; 0];
    static DAT_100a50bc: [i8; 0];
    static mut DAT_100a6848: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __wfopen(filename: *const u8, mode: *const u8) -> *mut u8;
    fn _fread(ptr: *mut u8, size: i32, count: i32, stream: *mut u8) -> i32;
    fn _fclose(stream: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005ab70() {
    // SAFETY: the globals and the C runtime functions are the game's own;
    // the file handle is checked for null before use.
    unsafe {
        let file = __wfopen(
            (&raw const s_MW2DIF_CFG_100a50c0).cast::<u8>(),
            (&raw const DAT_100a50bc).cast::<u8>(),
        );
        if !file.is_null() {
            _fread((&raw mut DAT_100a6848).cast::<u8>(), 0x17, 1, file);
            _fclose(file);
        }
    }
}
