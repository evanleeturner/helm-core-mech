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
    mech_trace::enter!("10064e6a", ch);
    // SAFETY: _putc and the stream global are the game's own; the stream
    // pointer is read by value and passed through unchanged.
    unsafe {
        let i_var1 = mech_trace::cr!(
            "10064e6a",
            "_putc",
            _putc(
                u32::from(ch) as i32,
                mech_trace::gr!("10064e6a", "100c7f9c", DAT_100c7f9c)
            )
        );
        mech_trace::rt!("10064e6a", if i_var1 == -1 { 2 } else { 0 })
    }
}
