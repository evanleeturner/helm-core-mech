// SPDX-License-Identifier: MIT

#[repr(C)]
pub struct tagPOINT {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub struct tagMSG {
    pub hwnd: *mut core::ffi::c_void,
    pub message: u32,
    pub w_param: u32,
    pub l_param: i32,
    pub time: u32,
    pub pt: tagPOINT,
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098204: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn WaitMessage() -> i32;
    fn PeekMessageA(
        p1: *mut core::ffi::c_void,
        hwnd: *mut core::ffi::c_void,
        min: u32,
        max: u32,
        remove: u32,
    ) -> i32;
    fn PostQuitMessage(code: i32);
    fn TranslateMessage(msg: *const core::ffi::c_void) -> i32;
    fn DispatchMessageA(msg: *const core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100164f0() -> u32 {
    let mut local_1c = tagMSG {
        hwnd: core::ptr::null_mut(),
        message: 0,
        w_param: 0,
        l_param: 0,
        time: 0,
        pt: tagPOINT { x: 0, y: 0 },
    };
    let msg = &raw mut local_1c;
    // SAFETY: the globals and Win32 functions are the game's own; msg points
    // at a live local MSG structure.
    unsafe {
        if DAT_10098204 == 0 {
            WaitMessage();
        }
        let b_var1 = PeekMessageA(msg.cast(), core::ptr::null_mut(), 0, 0, 1);
        if b_var1 != 0 {
            if (*msg).message == 0x12 {
                PostQuitMessage((*msg).w_param as i32);
                return 0;
            }
            let mut u_var2: u32 = 1;
            TranslateMessage(msg.cast());
            if (*msg).message == 0x111
                && ((*msg).w_param == 0x9c41
                    || (*msg).w_param == 0x9c97
                    || (*msg).w_param == 0x9c43)
            {
                u_var2 = 0;
            }
            DispatchMessageA(msg.cast());
            return u_var2;
        }
        1
    }
}
