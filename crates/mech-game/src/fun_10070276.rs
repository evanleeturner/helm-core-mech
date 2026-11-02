// SPDX-License-Identifier: MIT

#[repr(C)]
pub struct tagPOINT {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub struct tagMSG {
    pub hwnd: *mut u8,
    pub message: u32,
    pub w_param: u32,
    pub l_param: i32,
    pub time: u32,
    pub pt: tagPOINT,
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100704a0(param_1: u32, param_2: u32, param_3: i32) -> u8;
    fn FUN_10070381(param_1: u32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn DefWindowProcA(hwnd: *mut u8, msg: u32, w_param: u32, l_param: i32) -> i32;
    fn PeekMessageA(msg: *mut tagMSG, hwnd: *mut u8, min: u32, max: u32, remove: u32) -> i32;
    fn TranslateMessage(msg: *const tagMSG) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10070276(param_1: u32, param_2: u32, param_3: u32) {
    mech_trace::enter!("10070276", param_1, param_2, param_3);
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    let mut msg = tagMSG {
        hwnd: core::ptr::null_mut(),
        message: 0,
        w_param: 0,
        l_param: 0,
        time: 0,
        pt: tagPOINT { x: 0, y: 0 },
    };
    let mut local_8: i32 = 0;
    loop {
        // SAFETY: the callees are the game's own and the Win32 calls receive
        // a valid local MSG; arguments are passed as the original does.
        unsafe {
            match param_1 {
                0x100 | 0x104 => {
                    mech_trace::cr!("10070276", "100704a0", FUN_100704a0(param_2, param_3, 1));
                }
                0x101 | 0x105 => {
                    mech_trace::cr!("10070276", "10070381", FUN_10070381(param_2));
                    mech_trace::cr!("10070276", "100704a0", FUN_100704a0(param_2, param_3, 0));
                }
                _ => {}
            }
            if 0x103 < param_1 {
                mech_trace::cr!(
                    "10070276",
                    "DefWindowProcA",
                    DefWindowProcA(msg.hwnd, msg.message, msg.w_param, msg.l_param)
                );
            }
            let message_available = mech_trace::cr!(
                "10070276",
                "PeekMessageA",
                PeekMessageA(&raw mut msg, core::ptr::null_mut(), 0x100, 0x108, 1)
            );
            if message_available == 0 {
                local_8 = 1;
            } else {
                mech_trace::cr!(
                    "10070276",
                    "TranslateMessage",
                    TranslateMessage(&raw const msg)
                );
                param_1 = msg.message;
                param_2 = msg.w_param;
                param_3 = msg.l_param as u32;
            }
        }
        if local_8 != 0 {
            break;
        }
    }
}
