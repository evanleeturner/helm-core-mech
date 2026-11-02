// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001bbf4(param_1: i32, param_2: i32, param_3: u32) -> u32 {
    let den32: i32 = (param_1 as u32).wrapping_sub(param_2 as u32) as i32;
    if den32 != 0 {
        let hi: u32 = (((param_3 as i32) >> 0x1f) as u32).wrapping_shl(0x11) | (param_3 >> 0xf);
        let lo: u32 = param_3.wrapping_shl(0x11);

        let num: i64 = (((hi as u64) << 32) | (lo as u64)) as i64;
        let den: i64 = den32 as i64;

        let quot: i64 = num / den;

        if quot > 0x7FFF_FFFF || quot < -0x8000_0000 {
            // SAFETY: RaiseException is the OS routine; it takes no pointer
            // when the argument count is zero.
            unsafe {
                RaiseException(0xC000_0094, 0, 0, core::ptr::null());
            }
            return 0;
        }

        return quot as u32;
    }
    0
}
