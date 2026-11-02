// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10055bc0(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) -> u64 {
    mech_trace::enter!("10055bc0", param_1, param_2, param_3, param_4, param_5);
    let i_var3: i32;
    let l_var1: i64;
    let base: i32;

    if param_1 < param_2 {
        i_var3 = (param_1 as u32).wrapping_sub(param_2 as u32) as i32;
        l_var1 = ((param_3 as u32).wrapping_sub(param_2 as u32) as i32 as i64)
            .wrapping_mul((param_4 as u32).wrapping_sub(param_5 as u32) as i32 as i64);
        base = param_5;
    } else {
        i_var3 = (param_2 as u32).wrapping_sub(param_1 as u32) as i32;
        l_var1 = ((param_3 as u32).wrapping_sub(param_1 as u32) as i32 as i64)
            .wrapping_mul((param_5 as u32).wrapping_sub(param_4 as u32) as i32 as i64);
        base = param_4;
    }
    let mut i_var4: i32 = ((l_var1 as u64) >> 32) as i32;
    let i_var2: i32;

    if i_var3 == 0 || (i_var4 == i32::MIN && i_var3 == -1) {
        // SAFETY: RaiseException is the OS import; arguments mirror the original.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        return mech_trace::rt!("10055bc0", 0);
    }

    let q;
    if i_var4 / i_var3 == 0 {
        let quot = l_var1 / (i_var3 as i64);
        if quot > 0x7FFFFFFF_i64 || quot < -0x80000000_i64 {
            // SAFETY: RaiseException is the OS import; arguments mirror the original.
            unsafe {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
            }
            return mech_trace::rt!("10055bc0", 0);
        }
        let rem = l_var1 % (i_var3 as i64);
        q = quot as i32;
        i_var4 = rem as i32;
    } else {
        q = -0x10000001;
    }
    i_var2 = (q as u32).wrapping_add(base as u32) as i32;

    mech_trace::rt!(
        "10055bc0",
        ((i_var4 as u32 as u64) << 32) | (i_var2 as u32 as u64)
    )
}
