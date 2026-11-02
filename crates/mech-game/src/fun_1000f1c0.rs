// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000f1c0(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    let prod: i64 = (param_1 as i64).wrapping_mul(param_2 as i64);
    let num: i64 = prod >> 8;
    let den: i64 = param_3 as i64;

    if den == 0 {
        // SAFETY: RaiseException is the OS's own function; a null argument
        // pointer with zero count is valid.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        return 0;
    }

    // SAFETY-free: den is nonzero, and num is within +/-2^55, so no overflow.
    let quot: i64 = num / den;

    if quot < -2147483648i64 || quot > 2147483647i64 {
        // SAFETY: RaiseException is the OS's own function; a null argument
        // pointer with zero count is valid.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        return 0;
    }

    (quot as i32) >> 6
}
