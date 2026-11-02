// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ada90: *mut core::ffi::c_void;
    static mut DAT_100ada94: *mut core::ffi::c_void;
    static mut DAT_100adf74: *mut core::ffi::c_void;
    static mut DAT_10168410: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, mem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003289f() {
    mech_trace::enter!("1003289f");
    // SAFETY: the globals and the heap handle are the game's own; the table
    // at DAT_10168410 holds 16 pointers, each freed and cleared in order.
    unsafe {
        if !mech_trace::gr!("1003289f", "100ada90", DAT_100ada90).is_null() {
            mech_trace::cr!(
                "1003289f",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("1003289f", "100adf74", DAT_100adf74),
                    1,
                    mech_trace::gr!("1003289f", "100ada90", DAT_100ada90)
                )
            );
        }
        mech_trace::gw!("1003289f", "100ada90", DAT_100ada90, core::ptr::null_mut());
        let mut local_8: i32 = 0;
        while local_8 < 0x10 {
            let slot = (&raw mut DAT_10168410)
                .byte_add((local_8 as usize).wrapping_mul(4))
                .cast::<*mut core::ffi::c_void>();
            if !mech_trace::rd!("1003289f", slot, read_unaligned).is_null() {
                mech_trace::cr!(
                    "1003289f",
                    "HeapFree",
                    HeapFree(
                        mech_trace::gr!("1003289f", "100adf74", DAT_100adf74),
                        1,
                        mech_trace::rd!("1003289f", slot, read_unaligned)
                    )
                );
            }
            mech_trace::wr!("1003289f", slot, write_unaligned, core::ptr::null_mut());
            local_8 = local_8.wrapping_add(1);
        }
        if !mech_trace::gr!("1003289f", "100ada94", DAT_100ada94).is_null() {
            mech_trace::cr!(
                "1003289f",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("1003289f", "100adf74", DAT_100adf74),
                    1,
                    mech_trace::gr!("1003289f", "100ada94", DAT_100ada94)
                )
            );
        }
        mech_trace::gw!("1003289f", "100ada94", DAT_100ada94, core::ptr::null_mut());
    }
}
