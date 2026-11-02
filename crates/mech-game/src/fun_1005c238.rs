// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: i32;
    static mut DAT_100af0d4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut core::ffi::c_void;
    fn FUN_1005c0df(param_1: i32, param_2: i32, param_3: *mut i8, param_4: i32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005c238(
    param_1: i32,
    param_2: i32,
    param_3: u32,
    param_4: i32,
    param_5: i32,
) {
    // SAFETY: the globals and callees are the game's own; the returned string
    // is a NUL-terminated buffer which is negated in place, as the original does.
    unsafe {
        let p_var1 =
            FUN_1008668b(DAT_100af17c, param_3, DAT_100af0d4 as usize as *mut i32) as *mut i8;
        let mut local_8 = p_var1;
        if !p_var1.is_null() {
            while *local_8 != 0 {
                *local_8 = (*local_8).wrapping_neg();
                local_8 = local_8.byte_add(1);
            }
            FUN_1005c0df(param_1, param_2, p_var1, param_4, param_5);
        }
    }
}
