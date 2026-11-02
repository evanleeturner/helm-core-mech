// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003c170(param_1: i32, param_2: i32, param_3: i32) -> u32 {
    mech_trace::enter!("1003c170", param_1, param_2, param_3);
    let num: i64 = (param_1 as i64).wrapping_mul(param_2 as i64);

    let d: i32 = (param_2 as u32).wrapping_add(param_3 as u32) as i32;
    let den: i64 = d as i64;

    if den == 0 {
        // SAFETY: RaiseException is the OS routine; null argument list with zero count.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        return mech_trace::rt!("1003c170", 0);
    }

    // |num| <= 2^62 and den != 0, so this division can neither trap nor overflow.
    let quot: i64 = num / den;

    if quot > 0x7FFF_FFFF || quot < -0x8000_0000 {
        // SAFETY: RaiseException is the OS routine; null argument list with zero count.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        return mech_trace::rt!("1003c170", 0);
    }

    mech_trace::rt!("1003c170", ((quot as i32) as u32) >> 0xc)
}
