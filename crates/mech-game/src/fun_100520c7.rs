// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101151be: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100520c7(param_1: *mut i8) -> u32 {
    mech_trace::enter!("100520c7", param_1);
    let mut local_c: u32 = 0xffff_ffff;
    let mut pcvar2: *const i8 = param_1;
    // SAFETY: param_1 is a NUL-terminated string from the caller; DAT_101151be
    // is the game's own table, read backwards as the original does.
    unsafe {
        loop {
            if local_c == 0 {
                break;
            }
            local_c = local_c.wrapping_sub(1);
            let c_var1 = mech_trace::dr!("100520c7", pcvar2);
            pcvar2 = pcvar2.wrapping_add(1);
            if c_var1 == 0 {
                break;
            }
        }
        local_c = !local_c;
        let mut local_8: *const u8 = &raw mut DAT_101151be;
        let mut local_10: *const u8 =
            (param_1 as *const u8).wrapping_add(local_c.wrapping_sub(2) as usize);
        loop {
            local_c = local_c.wrapping_sub(1);
            if local_c == 0 {
                return mech_trace::rt!("100520c7", 1);
            }
            if (mech_trace::rd!("100520c7", local_10, read) ^ 10)
                != mech_trace::rd!("100520c7", local_8, read)
            {
                break;
            }
            local_10 = local_10.wrapping_sub(1);
            local_8 = local_8.wrapping_sub(1);
        }
        mech_trace::rt!("100520c7", 0)
    }
}
