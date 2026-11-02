// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_1002f28d(param_1: i32) -> i32;
    fn FUN_1002fe1e(param_1: *mut i32, param_2: *mut core::ffi::c_void);
    fn FUN_1001d671();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000e4a8(param_1: *mut i32) {
    let mut local_8: *mut core::ffi::c_void = core::ptr::null_mut();
    if !param_1.is_null() {
        // SAFETY: the callees are the game's own functions; param_1 is a
        // non-null pointer supplied by the caller and passed through.
        unsafe {
            let i_var1 = FUN_1002f28d(param_1 as i32);
            if i_var1 != 0 {
                local_8 = FUN_1001d671 as unsafe extern "C" fn() as *mut core::ffi::c_void;
            }
            FUN_1002fe1e(param_1, local_8);
        }
    }
}
