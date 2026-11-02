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

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100701b9(param_1: *mut u16) -> u32;
    fn FUN_10070276(param_1: u32, param_2: u32, param_3: u32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn PeekMessageA(
        msg: *mut tagMSG,
        hwnd: *mut core::ffi::c_void,
        min: u32,
        max: u32,
        remove: u32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1007069f() -> i16 {
    let mut local_24 = tagMSG {
        hwnd: core::ptr::null_mut(),
        message: 0,
        w_param: 0,
        l_param: 0,
        time: 0,
        pt: tagPOINT { x: 0, y: 0 },
    };
    let mut local_8: [i16; 2] = [0; 2];
    // SAFETY: the callees are the game's own and the OS's; the pointers
    // refer to live locals of the right size.
    unsafe {
        FUN_100701b9(local_8.as_mut_ptr().cast::<u16>());
        if local_8[0] == 0 {
            let b_var1 = PeekMessageA(&raw mut local_24, core::ptr::null_mut(), 0x100, 0x108, 1);
            if b_var1 != 0 {
                FUN_10070276(local_24.message, local_24.w_param, local_24.l_param as u32);
                FUN_100701b9(local_8.as_mut_ptr().cast::<u16>());
            }
        }
        local_8[0]
    }
}
