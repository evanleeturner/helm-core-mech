// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007afe4(param_1: i32, param_2: u32) -> i32 {
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut u_var3: u32 = (param_1 < 0) as u32;
    if param_1 < 0 {
        param_1 = 0u32.wrapping_sub(param_1 as u32) as i32;
    }
    if (param_2 as i32) < 0 {
        u_var3 = u_var3.wrapping_sub(1);
        param_2 = 0u32.wrapping_sub(param_2);
    }

    let hi: u32 = (param_1 >> 2) as u32;
    let lo: u32 = (param_1 as u32).wrapping_shl(0x1e);
    let u_var1: u64 = ((hi as u64) << 32) | (lo as u64);

    if param_2 == 0 || hi >= param_2 {
        // SAFETY: the OS routine raises a structured exception; no pointer is passed.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        return 0;
    }

    let quot: u32 = (u_var1 / (param_2 as u64)) as u32;
    let rem: u32 = (u_var1 % (param_2 as u64)) as u32;

    let half: u32 = (param_2 >> 1).wrapping_add(((param_2 & 1) != 0) as u32);
    let mut i_var2: u32 = quot.wrapping_add((half.wrapping_sub(1) < rem) as u32);
    if u_var3 != 0 {
        i_var2 = 0u32.wrapping_sub(i_var2);
    }
    i_var2 as i32
}
