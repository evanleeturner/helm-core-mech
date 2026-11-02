// SPDX-License-Identifier: MIT

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7f9c: *mut FILE;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _putc(_Ch: i32, _File: *mut FILE) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10064e6a(ch: u8) -> u32 {
    // SAFETY: _putc and the stream global are the game's own; the stream
    // pointer is read by value and passed through unchanged.
    unsafe {
        let i_var1 = _putc(u32::from(ch) as i32, DAT_100c7f9c);
        if i_var1 == -1 { 2 } else { 0 }
    }
}
