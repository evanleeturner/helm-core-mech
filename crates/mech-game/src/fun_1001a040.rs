// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001a040(param_1: u32, param_2: u32) -> i32 {
    let hi: u32 = (((param_1 as i32).wrapping_shr(0x1f)) as u32).wrapping_shl(0x10)
        | param_1.wrapping_shr(0x10);
    let lo: u32 = param_1.wrapping_shl(0x10);

    let num: u64 = ((hi as u64) << 32) | (lo as u64);
    let den: u64 = param_2 as u64;

    if den == 0 {
        // SAFETY: RaiseException is the system's own function; a null argument list is valid.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        return 0;
    }

    let quot: u64 = num / den;

    if quot > 0xFFFF_FFFF {
        // SAFETY: RaiseException is the system's own function; a null argument list is valid.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        return 0;
    }

    quot as u32 as i32
}
