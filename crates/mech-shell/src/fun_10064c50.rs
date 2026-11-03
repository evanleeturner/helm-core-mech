// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10040680(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006ec30(param_1: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_10064c50(param_1: i32) {
    mech_trace::enter!("10064c50", param_1);
    // SAFETY: param_1 points at the game's object; the callees are the game's own
    // and receive the pointers stored at offsets 0x1c and 0x20.
    unsafe {
        let mut pv_var1 = mech_trace::rd!(
            "10064c50",
            ((param_1 as usize).wrapping_add(0x1c) as *const *mut core::ffi::c_void),
            read_unaligned
        );
        if !pv_var1.is_null() {
            FUN_10040680(pv_var1 as i32);
            FUN_1006ec30(pv_var1);
        }
        pv_var1 = mech_trace::rd!(
            "10064c50",
            ((param_1 as usize).wrapping_add(0x20) as *const *mut core::ffi::c_void),
            read_unaligned
        );
        if !pv_var1.is_null() {
            FUN_10040680(pv_var1 as i32);
            FUN_1006ec30(pv_var1);
        }
    }
}
