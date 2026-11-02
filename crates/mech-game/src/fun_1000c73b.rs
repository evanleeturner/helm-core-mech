// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7580: *mut core::ffi::c_void;
    static mut DAT_100a7570: *mut core::ffi::c_void;
    static mut DAT_100a7560: *mut core::ffi::c_void;
    static mut DAT_100a7574: *mut core::ffi::c_void;
    static mut DAT_100a7564: *mut core::ffi::c_void;
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_100a7578: i32;
    static mut DAT_100a757c: i32;
    static mut DAT_101f5510: i32;
    static mut DAT_10215624: i32;
    static mut DAT_10215620: i32;
    static mut DAT_101f5518: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(hHeap: *mut core::ffi::c_void, dwFlags: u32, lpMem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000c73b() {
    mech_trace::enter!("1000c73b");
    // SAFETY: the globals and HeapFree are the game's own; each non-null
    // block is freed on the game's heap and its pointer cleared.
    unsafe {
        if !mech_trace::gr!("1000c73b", "100a7580", DAT_100a7580).is_null() {
            mech_trace::cr!(
                "1000c73b",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("1000c73b", "100adf74", DAT_100adf74),
                    1,
                    mech_trace::gr!("1000c73b", "100a7580", DAT_100a7580)
                )
            );
            mech_trace::gw!("1000c73b", "100a7580", DAT_100a7580, core::ptr::null_mut());
        }
        if !mech_trace::gr!("1000c73b", "100a7570", DAT_100a7570).is_null() {
            mech_trace::cr!(
                "1000c73b",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("1000c73b", "100adf74", DAT_100adf74),
                    1,
                    mech_trace::gr!("1000c73b", "100a7570", DAT_100a7570)
                )
            );
            mech_trace::gw!("1000c73b", "100a7570", DAT_100a7570, core::ptr::null_mut());
        }
        if !mech_trace::gr!("1000c73b", "100a7560", DAT_100a7560).is_null() {
            mech_trace::cr!(
                "1000c73b",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("1000c73b", "100adf74", DAT_100adf74),
                    1,
                    mech_trace::gr!("1000c73b", "100a7560", DAT_100a7560)
                )
            );
            mech_trace::gw!("1000c73b", "100a7560", DAT_100a7560, core::ptr::null_mut());
        }
        if !mech_trace::gr!("1000c73b", "100a7574", DAT_100a7574).is_null() {
            mech_trace::cr!(
                "1000c73b",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("1000c73b", "100adf74", DAT_100adf74),
                    1,
                    mech_trace::gr!("1000c73b", "100a7574", DAT_100a7574)
                )
            );
            mech_trace::gw!("1000c73b", "100a7574", DAT_100a7574, core::ptr::null_mut());
        }
        if !mech_trace::gr!("1000c73b", "100a7564", DAT_100a7564).is_null() {
            mech_trace::cr!(
                "1000c73b",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("1000c73b", "100adf74", DAT_100adf74),
                    1,
                    mech_trace::gr!("1000c73b", "100a7564", DAT_100a7564)
                )
            );
            mech_trace::gw!("1000c73b", "100a7564", DAT_100a7564, core::ptr::null_mut());
        }
        mech_trace::gw!("1000c73b", "100a7578", DAT_100a7578, 0);
        mech_trace::gw!("1000c73b", "100a757c", DAT_100a757c, 0);
        mech_trace::gw!("1000c73b", "101f5510", DAT_101f5510, 0);
        mech_trace::gw!("1000c73b", "10215624", DAT_10215624, 0);
        mech_trace::gw!("1000c73b", "10215620", DAT_10215620, 0);
        mech_trace::gw!("1000c73b", "101f5518", DAT_101f5518, 0);
    }
}
