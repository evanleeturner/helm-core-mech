// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
}

unsafe extern "stdcall" {
    fn HeapAlloc(h_heap: *mut c_void, dw_flags: u32, dw_bytes: u32) -> *mut c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10071750(
    param_1: *mut i32,
    param_2: i32,
    param_3: u8,
    param_4: i32,
) -> u32 {
    mech_trace::enter!("10071750", param_1, param_2, param_3, param_4);
    // SAFETY: the heap handle is the game's own global; param_1 and param_2
    // point at the caller's memory, read and written where the original does.
    unsafe {
        let puVar1 = mech_trace::cr!(
            "10071750",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10071750", "100adf74", DAT_100adf74),
                1,
                param_4.wrapping_mul(3) as u32
            )
        ) as *mut u8;
        if puVar1.is_null() {
            return mech_trace::rt!("10071750", 0);
        }
        let mut local_10 = puVar1;
        let mut local_c: i32 = 0;
        while local_c < param_4.wrapping_mul(3) {
            mech_trace::dw!(
                "10071750",
                local_10,
                mech_trace::dr!("10071750", (local_c.wrapping_add(param_2) as *const u8))
            );
            local_10 = local_10.wrapping_add(1);
            local_c = local_c.wrapping_add(1);
        }
        mech_trace::wr!("10071750", param_1, write_unaligned, param_2);
        mech_trace::wr!(
            "10071750",
            param_1.byte_add(4),
            write_unaligned,
            puVar1 as i32
        );
        mech_trace::wr!("10071750", param_1.byte_add(8).cast::<u8>(), write, param_3);
        mech_trace::wr!("10071750", param_1.byte_add(9), write_unaligned, param_4);
        mech_trace::rt!("10071750", 1)
    }
}
