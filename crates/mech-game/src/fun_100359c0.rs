// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100359c0(
    param_1: u32,
    param_2: i32,
    param_3: u8,
    param_4: i32,
) -> i32 {
    let s: u32 = (param_3 & 0x1f) as u32;
    let sign: u32 = ((param_1 as i32) >> 0x1f) as u32;

    let right: u32 = if s == 0 {
        0
    } else {
        param_1.wrapping_shr(0x20u32.wrapping_sub(s))
    };
    let hi: u32 = sign.wrapping_shl(s) | right;
    let lo: u32 = param_1.wrapping_shl(s);

    let num: i64 = ((((hi as u64) << 32) | (lo as u64)) as i64);
    let den: i64 = param_2 as i64;

    if den == 0 {
        // SAFETY: RaiseException is the system's own routine; a null argument
        // list with zero arguments is valid.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        return 0;
    }

    let quot: i64 = num.wrapping_div(den);

    if quot > 0x7FFF_FFFFi64 || quot < -0x8000_0000i64 {
        // SAFETY: RaiseException is the system's own routine; a null argument
        // list with zero arguments is valid.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        return 0;
    }

    let mut q32: i32 = quot as i32;
    q32 = (q32 as u32).wrapping_add(2) as i32;
    q32 >>= 2;
    (q32 as u32).wrapping_add(param_4 as u32) as i32
}
