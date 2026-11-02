// SPDX-License-Identifier: MIT

#[repr(C)]
pub struct tagPOINT {
    pub x: i32,
    pub y: i32,
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100d08e8: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn ClientToScreen(param_1: u32, param_2: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10051950(param_1: *mut i32, param_2: i32, param_3: i32) {
    mech_trace::enter!("10051950", param_1, param_2, param_3);
    let mut local_8 = tagPOINT { x: 0, y: 0 };
    // SAFETY: the global and the API are the game's own; param_1 points at
    // the caller's four-i32 buffer, written where the original does.
    unsafe {
        mech_trace::cr!(
            "10051950",
            "ClientToScreen",
            ClientToScreen(
                mech_trace::gr!("10051950", "100d08e8", DAT_100d08e8),
                (&raw mut local_8).cast()
            )
        );
        mech_trace::wr!("10051950", param_1.add(1), write_unaligned, local_8.y);
        mech_trace::wr!("10051950", param_1, write_unaligned, local_8.x);
        local_8.x = param_2.wrapping_sub(1);
        local_8.y = param_3.wrapping_sub(1);
        mech_trace::cr!(
            "10051950",
            "ClientToScreen",
            ClientToScreen(
                mech_trace::gr!("10051950", "100d08e8", DAT_100d08e8),
                (&raw mut local_8).cast()
            )
        );
        mech_trace::wr!("10051950", param_1.add(2), write_unaligned, local_8.x);
        mech_trace::wr!("10051950", param_1.add(3), write_unaligned, local_8.y);
    }
}
