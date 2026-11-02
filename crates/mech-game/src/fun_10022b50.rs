// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(param_1: u32, param_2: u32, param_3: u32, param_4: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10022b50(param_1: u32, param_2: i32) -> i32 {
    let hi: u32 =
        (((param_1 as i32).wrapping_shr(0x1f) as u32).wrapping_shl(0x1d)) | param_1.wrapping_shr(3);
    let lo: u32 = param_1.wrapping_shl(0x1d);

    let num: i64 = ((((hi as u64) << 32) | (lo as u64)) as i64);
    let den: i64 = param_2 as i64;

    if den == 0 {
        // SAFETY: RaiseException is the system import; a null argument list is valid.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        return 0;
    }

    let quot: i64 = num.wrapping_div(den);

    if quot > 0x7FFF_FFFFi64 || quot < -0x8000_0000i64 {
        // SAFETY: RaiseException is the system import; a null argument list is valid.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        return 0;
    }

    quot as i32
}
