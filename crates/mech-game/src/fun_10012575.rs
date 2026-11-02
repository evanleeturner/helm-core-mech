// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1dc: *mut c_void;
    static mut DAT_100aa1e8: *mut c_void;
    static mut DAT_100aa1ec: *mut c_void;
    static mut DAT_100aa1f0: u32;
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_101eb4c0: *mut c_void;
    static mut DAT_101eb4c4: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10013faa() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h_heap: *mut c_void, dw_flags: u32, lp_mem: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10012575() {
    mech_trace::enter!("10012575");
    // SAFETY: the callees and globals are the game's own memory; the heap
    // pointers are freed in the original's order under the original's checks.
    unsafe {
        mech_trace::cr!("10012575", "10013faa", FUN_10013faa());
        if mech_trace::gr!("10012575", "100aa1f0", DAT_100aa1f0) != 0 {
            if !mech_trace::gr!("10012575", "100aa1dc", DAT_100aa1dc).is_null() {
                mech_trace::cr!(
                    "10012575",
                    "HeapFree",
                    HeapFree(
                        mech_trace::gr!("10012575", "100adf74", DAT_100adf74),
                        1,
                        mech_trace::gr!("10012575", "100aa1dc", DAT_100aa1dc)
                    )
                );
            }
            if !mech_trace::gr!("10012575", "100aa1e8", DAT_100aa1e8).is_null() {
                mech_trace::cr!(
                    "10012575",
                    "HeapFree",
                    HeapFree(
                        mech_trace::gr!("10012575", "100adf74", DAT_100adf74),
                        1,
                        mech_trace::gr!("10012575", "100aa1e8", DAT_100aa1e8)
                    )
                );
            }
            if !mech_trace::gr!("10012575", "100aa1ec", DAT_100aa1ec).is_null() {
                mech_trace::cr!(
                    "10012575",
                    "HeapFree",
                    HeapFree(
                        mech_trace::gr!("10012575", "100adf74", DAT_100adf74),
                        1,
                        mech_trace::gr!("10012575", "100aa1ec", DAT_100aa1ec)
                    )
                );
            }
            mech_trace::cr!(
                "10012575",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("10012575", "100adf74", DAT_100adf74),
                    1,
                    mech_trace::gr!("10012575", "101eb4c4", DAT_101eb4c4)
                )
            );
            mech_trace::cr!(
                "10012575",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("10012575", "100adf74", DAT_100adf74),
                    1,
                    mech_trace::gr!("10012575", "101eb4c0", DAT_101eb4c0)
                )
            );
        }
    }
}
