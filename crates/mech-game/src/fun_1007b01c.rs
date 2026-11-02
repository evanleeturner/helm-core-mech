// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007b01c(param_1: u32) -> i32 {
    mech_trace::enter!("1007b01c", param_1);
    let mut param_1 = param_1;
    let mut i_var2: i32 = 0;
    if (param_1 as i32) < 0 {
        i_var2 = -1;
        param_1 = 0u32.wrapping_sub(param_1);
    }

    let dividend: u64 = 0x400000000000u64;

    if param_1 == 0 || 0x4000u32 >= param_1 {
        // SAFETY: RaiseException is the OS routine; a null argument array
        // with a zero count is valid.
        unsafe {
            RaiseException(0xC0000094u32, 0, 0, core::ptr::null());
        }
        return mech_trace::rt!("1007b01c", 0);
    }

    let mut quot = (dividend / (param_1 as u64)) as u32;
    let rem = (dividend % (param_1 as u64)) as u32;

    let u_var1 = ((param_1 >> 1).wrapping_add(((param_1 & 1) != 0) as u32)).wrapping_sub(1);
    quot = quot.wrapping_add((u_var1 < rem) as u32);

    if i_var2 != 0 {
        quot = 0u32.wrapping_sub(quot);
    }
    mech_trace::rt!("1007b01c", quot as i32)
}
