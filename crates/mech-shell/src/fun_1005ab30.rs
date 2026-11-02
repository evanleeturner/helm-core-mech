// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut s_MW2DIF_CFG_100a50b0: [u16; 0];
    static mut DAT_100a50ac: [u16; 0];
    static mut DAT_100a6848: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __wfopen(filename: *mut u16, mode: *mut u16) -> *mut core::ffi::c_void;
    fn _fread(
        buffer: *mut core::ffi::c_void,
        size: u32,
        count: u32,
        stream: *mut core::ffi::c_void,
    ) -> u32;
    fn _fclose(stream: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005ab30() {
    // SAFETY: the globals and the C runtime functions are the game's own;
    // the file handle is checked for null before use.
    unsafe {
        let cfg_file = __wfopen(
            (&raw mut s_MW2DIF_CFG_100a50b0).cast::<u16>(),
            (&raw mut DAT_100a50ac).cast::<u16>(),
        );
        if !cfg_file.is_null() {
            _fread(
                (&raw mut DAT_100a6848).cast::<core::ffi::c_void>(),
                0x17,
                1,
                cfg_file,
            );
            _fclose(cfg_file);
        }
    }
}
