// SPDX-License-Identifier: MIT

#[repr(C)]
struct tagPOINT {
    x: i32,
    y: i32,
}

#[repr(C)]
struct tagMSG {
    hwnd: *mut u8,
    message: u32,
    w_param: u32,
    l_param: i32,
    time: u32,
    pt: tagPOINT,
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005b6c0(param_1: *mut u16) -> u32;
    fn FUN_1005b760(param_1: u32, param_2: u32, param_3: u32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn PeekMessageA(msg: *mut tagMSG, hwnd: *mut u8, min: u32, max: u32, remove: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005ba70() -> i16 {
    mech_trace::enter!("1005ba70");
    let mut key_code: i16 = 0;
    let mut key_message = tagMSG {
        hwnd: core::ptr::null_mut(),
        message: 0,
        w_param: 0,
        l_param: 0,
        time: 0,
        pt: tagPOINT { x: 0, y: 0 },
    };
    // SAFETY: the callees are the game's own and the Win32 API; the pointers
    // refer to live locals of this frame.
    unsafe {
        mech_trace::cr!(
            "1005ba70",
            "1005b6c0",
            FUN_1005b6c0((&raw mut key_code).cast::<u16>())
        );
        if key_code == 0 {
            let got_message = mech_trace::cr!(
                "1005ba70",
                "PeekMessageA",
                PeekMessageA(&raw mut key_message, core::ptr::null_mut(), 0x100, 0x108, 1)
            );
            if got_message != 0 {
                mech_trace::cr!(
                    "1005ba70",
                    "1005b760",
                    FUN_1005b760(
                        key_message.message,
                        key_message.w_param,
                        key_message.l_param as u32,
                    )
                );
                mech_trace::cr!(
                    "1005ba70",
                    "1005b6c0",
                    FUN_1005b6c0((&raw mut key_code).cast::<u16>())
                );
            }
        }
        mech_trace::rt!("1005ba70", key_code)
    }
}
