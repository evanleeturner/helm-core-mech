// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3958: u32;
    static mut DAT_100b3970: i8;
    static mut DAT_100eeee0: u8;
    static mut DAT_100eef0c: u8;
    static mut DAT_100eef10: u8;
    static mut DAT_100eef20: u8;
    static mut s_mercs_hud_100b3974: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fopen(_Filename: *mut i8, _Mode: *mut i8) -> *mut u8;
    fn _fread(_DstBuf: *mut u8, _ElementSize: u32, _Count: u32, _File: *mut u8) -> u32;
    fn _fclose(_File: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10067e7c() {
    mech_trace::enter!("10067e7c");
    // SAFETY: the globals and the C runtime functions are the game's own;
    // the buffers are the game's arrays, filled by fread from the open file.
    unsafe {
        let file = mech_trace::cr!(
            "10067e7c",
            "_fopen",
            _fopen(&raw mut s_mercs_hud_100b3974, &raw mut DAT_100b3970)
        );
        if file.is_null() {
            mech_trace::gw!("10067e7c", "100b3958", DAT_100b3958, 0);
        } else {
            mech_trace::gw!("10067e7c", "100b3958", DAT_100b3958, 1);
            mech_trace::cr!(
                "10067e7c",
                "_fread",
                _fread(&raw mut DAT_100eef20, 4, 7, file)
            );
            mech_trace::cr!(
                "10067e7c",
                "_fread",
                _fread(&raw mut DAT_100eeee0, 4, 1, file)
            );
            mech_trace::cr!(
                "10067e7c",
                "_fread",
                _fread(&raw mut DAT_100eef0c, 4, 1, file)
            );
            mech_trace::cr!(
                "10067e7c",
                "_fread",
                _fread(&raw mut DAT_100eef10, 4, 1, file)
            );
            mech_trace::cr!("10067e7c", "_fclose", _fclose(file));
        }
    }
}
