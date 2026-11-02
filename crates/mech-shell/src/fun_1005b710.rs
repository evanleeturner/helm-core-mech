// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c40c0: u32;
    static mut DAT_100c40c4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005b760(param_1: u32, param_2: u32, param_3: u32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn PeekMessageA(msg: *mut u32, hwnd: *mut u8, min: u32, max: u32, remove: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005b710() {
    // MSG layout: hwnd, message, wParam, lParam, time, pt.x, pt.y (7 x 4 bytes)
    let mut key_message: [u32; 7] = [0; 7];
    // SAFETY: the buffer is a local MSG-sized array; the callees and globals
    // are the game's own and the OS's.
    unsafe {
        let got_message = PeekMessageA(
            key_message.as_mut_ptr(),
            core::ptr::null_mut(),
            0x100,
            0x108,
            1,
        );
        if got_message != 0 {
            FUN_1005b760(key_message[1], key_message[2], key_message[3]);
        }
        DAT_100c40c4 = 0;
        DAT_100c40c0 = 0;
    }
}
