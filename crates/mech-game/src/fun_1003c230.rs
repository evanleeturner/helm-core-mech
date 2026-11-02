// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003c230(
    param_1: i32,
    param_2: i32,
    param_3: u32,
    param_4: i32,
) -> i32 {
    mech_trace::enter!("1003c230", param_1, param_2, param_3, param_4);
    let prod: u64 = (param_1 as i64 as u64).wrapping_mul(param_2 as i64 as u64);

    let hi: u32 = ((((param_3 as i32) >> 0x1f) as u32).wrapping_shl(0x10)) | (param_3 >> 0x10);
    let lo: u32 = param_3.wrapping_shl(0x10);
    let addend: u64 = ((hi as u64) << 32) | (lo as u64);

    let num: i64 = prod.wrapping_add(addend) as i64;
    let den: i64 = param_4 as i64;

    if den == 0 {
        // SAFETY: raises the divide-fault exception through the OS, no pointers passed.
        unsafe {
            RaiseException(0xC0000094u32, 0, 0, core::ptr::null());
        }
        return mech_trace::rt!("1003c230", 0);
    }

    let quot: i64 = if den == -1 {
        0u64.wrapping_sub(num as u64) as i64
    } else {
        num / den
    };

    if quot > 0x7FFF_FFFFi64 || quot < -0x8000_0000i64 {
        // SAFETY: raises the divide-fault exception through the OS, no pointers passed.
        unsafe {
            RaiseException(0xC0000094u32, 0, 0, core::ptr::null());
        }
        return mech_trace::rt!("1003c230", 0);
    }

    mech_trace::rt!("1003c230", quot as i32)
}
