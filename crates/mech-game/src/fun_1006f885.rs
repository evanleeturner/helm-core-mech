// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3de0: *mut c_void;
    static mut DAT_100b9a2c: *mut u32;
    static mut DAT_100b9d30: u32;
    static mut DAT_100c9678: u32;
    static mut DAT_100eee30: u32;
    static mut DAT_100b3f7c: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *mut i8);
}

unsafe extern "stdcall" {
    fn SendMessageA(hwnd: *mut c_void, msg: u32, wparam: u32, lparam: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006f885() -> i32 {
    mech_trace::enter!("1006f885");
    let mut local: [u32; 3] = [0; 3];
    // SAFETY: the globals and callees are the game's own; the message
    // argument points at a local buffer that outlives the call.
    unsafe {
        local[0] = 0x2400401;
        if mech_trace::gr!("1006f885", "100b9d30", DAT_100b9d30) == 0 {
            local[1] = 0;
        } else {
            local[1] = mech_trace::gr!("1006f885", "100c9678", DAT_100c9678);
        }
        local[2] = mech_trace::gr!("1006f885", "100b9d30", DAT_100b9d30);
        mech_trace::gw!(
            "1006f885",
            "100eee30",
            DAT_100eee30,
            mech_trace::cr!(
                "1006f885",
                "SendMessageA",
                SendMessageA(
                    mech_trace::gr!("1006f885", "100b3de0", DAT_100b3de0),
                    0x4a,
                    0,
                    (&raw mut local) as *mut u32 as i32
                )
            ) as u32
        );
        if mech_trace::gr!("1006f885", "100eee30", DAT_100eee30) != 0 {
            FUN_10002168((&raw mut DAT_100b3f7c) as *mut i8);
        }
        mech_trace::wr!(
            "1006f885",
            mech_trace::gr!("1006f885", "100b9a2c", DAT_100b9a2c),
            write_unaligned,
            0
        );
        mech_trace::rt!(
            "1006f885",
            mech_trace::gr!("1006f885", "100eee30", DAT_100eee30) as i32
        )
    }
}
