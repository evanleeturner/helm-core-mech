// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10055b90(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) -> i32 {
    let a: u64 = (param_1 as i64 as u64).wrapping_mul(param_4 as i64 as u64);
    let b: u64 = (param_2 as i64 as u64).wrapping_mul(param_3 as i64 as u64);
    let dividend: i64 = a.wrapping_sub(b) as i64;

    if param_5 == 0 {
        // SAFETY: RaiseException is the system's own routine; null arguments are valid.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
    }
    if param_5 == -1 && dividend == i64::MIN {
        // SAFETY: RaiseException is the system's own routine; null arguments are valid.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
    }
    let q: i64 = if param_5 == 0 {
        // SAFETY: the original divides by zero here and the machine raises its
        // own divide fault; idiv with a zero divisor does the same.
        unsafe {
            core::arch::asm!(
                "idiv {0:e}",
                in(reg) 0u32,
                inout("eax") 0u32 => _,
                inout("edx") 0u32 => _,
            );
        }
        0
    } else {
        dividend.wrapping_div(param_5 as i64)
    };
    if q > 2147483647i64 || q < -2147483647i64 - 1 {
        // SAFETY: RaiseException is the system's own routine; null arguments are valid.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
    }
    q as i32
}
