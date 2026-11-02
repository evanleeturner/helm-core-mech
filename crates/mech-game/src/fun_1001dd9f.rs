// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100af0f0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008689c(param_1: u32, param_2: *mut i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut c_void, flags: u32, mem: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001dd9f(param_1: *mut i32) {
    mech_trace::enter!("1001dd9f", param_1);
    if !param_1.is_null() {
        // SAFETY: param_1 points at the caller's structure; the globals and
        // callees are the game's own, used as the original does.
        unsafe {
            if mech_trace::rd!("1001dd9f", param_1, read_unaligned) == 0 {
                let p = mech_trace::rd!(
                    "1001dd9f",
                    param_1.byte_add(0x17).cast::<*mut c_void>(),
                    read_unaligned
                );
                mech_trace::cr!(
                    "1001dd9f",
                    "HeapFree",
                    HeapFree(mech_trace::gr!("1001dd9f", "100adf74", DAT_100adf74), 1, p)
                );
            } else {
                let v = mech_trace::rd!("1001dd9f", param_1.add(1), read_unaligned);
                mech_trace::cr!(
                    "1001dd9f",
                    "1008689c",
                    FUN_1008689c(
                        (v as i16 as i32) as u32,
                        mech_trace::gr!("1001dd9f", "100af0f0", DAT_100af0f0) as usize as *mut i32
                    )
                );
            }
        }
    }
}
