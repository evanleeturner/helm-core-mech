// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut core::ffi::c_void, flags: u32, size: u32) -> *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001bf60(
        param_1: i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut i32,
        param_5: *mut i32,
        param_6: *mut i32,
        param_7: *mut i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001be70(param_1: i32) -> *mut i32 {
    mech_trace::enter!("1001be70", param_1);
    // SAFETY: the heap handle, allocator and callee are the game's own; the
    // allocation is 0x78 bytes, so the 0x1e zeroed ints and the pointers
    // into it stay in bounds.
    unsafe {
        let pi_var1 = mech_trace::cr!(
            "1001be70",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("1001be70", "100adf74", DAT_100adf74),
                1,
                0x78
            )
        );
        if !pi_var1.is_null() {
            let mut pi_var3 = pi_var1;
            let mut i_var2: i32 = 0x1e;
            while i_var2 != 0 {
                mech_trace::wr!("1001be70", pi_var3, write_unaligned, 0);
                pi_var3 = pi_var3.add(1);
                i_var2 -= 1;
            }
            FUN_1001bf60(
                param_1,
                pi_var1,
                pi_var1.add(1),
                pi_var1.add(2),
                pi_var1.add(3),
                pi_var1.add(4),
                pi_var1.add(5),
            );
        }
        mech_trace::rt!("1001be70", pi_var1)
    }
}
