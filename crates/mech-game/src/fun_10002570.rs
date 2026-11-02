// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10002570(param_1: u32, param_2: i32) -> i32 {
    mech_trace::enter!("10002570", param_1, param_2);
    let hi: u32 = ((((param_1 as i32) >> 0x1f) as u32) << 0x10) | (param_1 >> 0x10);
    let lo: u32 = param_1 << 0x10;
    let dividend: i64 = ((((hi as u64) << 32) | (lo as u64)) as i64);

    if param_2 == 0 {
        // SAFETY: RaiseException is the system routine; null args with zero count is valid.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
    }
    if param_2 == -1 && dividend == i64::MIN {
        // SAFETY: RaiseException is the system routine; null args with zero count is valid.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
    }

    let q: i64;
    if param_2 == 0 || (param_2 == -1 && dividend == i64::MIN) {
        let r: i32;
        // SAFETY: the machine's own idiv is used so the divide fault is raised
        // exactly as the original would on a zero divisor or overflow.
        unsafe {
            core::arch::asm!(
                "idiv {d:e}",
                d = in(reg) param_2,
                inout("eax") lo => r,
                inout("edx") hi => _,
            );
        }
        q = r as i64;
    } else {
        q = dividend / (param_2 as i64);
    }

    if q > 2147483647i64 || q < -2147483647i64 - 1 {
        // SAFETY: RaiseException is the system routine; null args with zero count is valid.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
    }
    mech_trace::rt!("10002570", q as i32)
}
