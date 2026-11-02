// SPDX-License-Identifier: MIT

#[repr(C)]
pub struct tagPOINT {
    pub x: i32,
    pub y: i32,
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf6c: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn ClientToScreen(param_1: u32, param_2: *mut tagPOINT) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005efd3(param_1: *mut i32, param_2: i32, param_3: i32) {
    let mut local_c = tagPOINT { x: 0, y: 0 };
    // SAFETY: the global and the Win32 call are the game's own; param_1 points
    // at the caller's four i32 values, written where the original does.
    unsafe {
        local_c.y = 0;
        local_c.x = 0;
        ClientToScreen(DAT_100adf6c, &raw mut local_c);
        param_1.write_unaligned(local_c.x);
        param_1.byte_add(4).write_unaligned(local_c.y);
        local_c.x = param_2.wrapping_sub(1);
        local_c.y = param_3.wrapping_sub(1);
        ClientToScreen(DAT_100adf6c, &raw mut local_c);
        param_1.byte_add(8).write_unaligned(local_c.x);
        param_1.byte_add(12).write_unaligned(local_c.y);
    }
}
