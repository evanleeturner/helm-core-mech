// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(param_1: u32, param_2: u32, param_3: u32, param_4: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10011e60(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    let prod: i64 = (param_1 as i64).wrapping_mul(param_2 as i64);
    let den: i64 = param_3 as i64;

    if den == 0 {
        // SAFETY: RaiseException is the system's own function; a null argument
        // list with zero count is valid.
        unsafe {
            RaiseException(0xC0000094u32, 0, 0, core::ptr::null());
        }
        return 0;
    }

    // The product of two i32 values never equals i64::MIN, and den is nonzero,
    // so this division cannot trap.
    let quot: i64 = prod / den;

    if quot < -2147483648i64 || quot > 2147483647i64 {
        // SAFETY: RaiseException is the system's own function; a null argument
        // list with zero count is valid.
        unsafe {
            RaiseException(0xC0000094u32, 0, 0, core::ptr::null());
        }
        return 0;
    }

    quot as i32
}
