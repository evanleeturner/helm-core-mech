// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3de0: *mut core::ffi::c_void;
    static mut DAT_100b9a2c: *mut u32;
    static mut DAT_100b9d30: u32;
    static mut DAT_100c9678: u32;
    static mut DAT_100eee30: u32;
    static mut DAT_100b3f5c: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *mut i8);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn SendMessageA(hwnd: *mut core::ffi::c_void, msg: u32, wparam: u32, lparam: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006f7e5() -> i32 {
    let mut local: [u32; 3] = [0; 3];
    local[0] = 0x2400401;
    // SAFETY: globals and callees are the game's own memory; the local
    // buffer outlives the SendMessageA call and DAT_100b9a2c is a valid
    // pointer in the original.
    unsafe {
        if DAT_100b9d30 == 0 {
            local[1] = 0;
        } else {
            local[1] = DAT_100c9678;
        }
        local[2] = DAT_100b9d30;
        DAT_100eee30 = SendMessageA(DAT_100b3de0, 0x4a, 0, (&raw mut local) as usize as i32) as u32;
        if DAT_100eee30 != 0 {
            FUN_10002168(&raw mut DAT_100b3f5c);
        }
        DAT_100b9a2c.write_unaligned(0);
        DAT_100eee30 as i32
    }
}
