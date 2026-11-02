// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008e770: i8;
    static mut DAT_100c42c0: u8;
    static mut s_mw2high_cfg_1008e774: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict___wfopen(_Filename: *mut i8, _Mode: *mut i8) -> *mut u8;
    fn _fread(_DstBuf: *mut u8, _ElementSize: u32, _Count: u32, _File: *mut u8) -> u32;
    fn _fclose(_File: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000fd70() {
    mech_trace::enter!("1000fd70");
    // SAFETY: the globals and callees are the game's own; the file handle is
    // checked for null before use and closed after the read.
    unsafe {
        let file = mech_trace::cr!(
            "1000fd70",
            "FID_conflict___wfopen",
            FID_conflict___wfopen(&raw mut s_mw2high_cfg_1008e774, &raw mut DAT_1008e770)
        );
        if !file.is_null() {
            mech_trace::cr!(
                "1000fd70",
                "_fread",
                _fread(&raw mut DAT_100c42c0, 0x2b6, 1, file)
            );
            mech_trace::cr!("1000fd70", "_fclose", _fclose(file));
        }
    }
}
