// SPDX-License-Identifier: MIT

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
pub unsafe extern "C" fn FUN_10064ea3(param_1: u32) -> u32 {
    mech_trace::enter!("10064ea3", param_1);
    // SAFETY: _putc and the global file pointer are the game's own.
    unsafe {
        let i_var1 = mech_trace::cr!(
            "10064ea3",
            "_putc",
            _putc(
                (param_1 & 0xff) as i32,
                mech_trace::gr!("10064ea3", "100c7f9c", DAT_100c7f9c)
            )
        );
        if i_var1 == -1 {
            return mech_trace::rt!("10064ea3", 2);
        }
        let i_var1 = mech_trace::cr!(
            "10064ea3",
            "_putc",
            _putc(
                (param_1 >> 8) as i32,
                mech_trace::gr!("10064ea3", "100c7f9c", DAT_100c7f9c)
            )
        );
        mech_trace::rt!("10064ea3", if i_var1 == -1 { 2 } else { 0 })
    }
}
