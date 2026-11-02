// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(
        dw_exception_code: u32,
        dw_exception_flags: u32,
        n_number_of_arguments: u32,
        lp_arguments: *const u32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001b82d(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: u32,
    param_5: i32,
    param_6: i32,
) -> i32 {
    mech_trace::enter!(
        "1001b82d", param_1, param_2, param_3, param_4, param_5, param_6
    );
    let mut usum: u64 = (param_1 as i64 as u64).wrapping_mul(param_5 as i64 as u64);
    usum = usum.wrapping_add((param_3 as i64 as u64).wrapping_mul(param_6 as i64 as u64));
    usum = usum.wrapping_add(param_4 as u64);

    let num: i64 = usum as i64;
    let den: i64 = param_2 as i64;

    if den == 0 {
        // SAFETY: RaiseException is the OS routine; the arguments are plain values and a null list.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        return mech_trace::rt!("1001b82d", 0);
    }

    if den == -1 && num == i64::MIN {
        // SAFETY: RaiseException is the OS routine; the arguments are plain values and a null list.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        return mech_trace::rt!("1001b82d", 0);
    }

    let quot: i64 = num.wrapping_div(den);

    if quot < -2147483648i64 || quot > 2147483647i64 {
        // SAFETY: RaiseException is the OS routine; the arguments are plain values and a null list.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        return mech_trace::rt!("1001b82d", 0);
    }

    mech_trace::rt!("1001b82d", quot as i32)
}
