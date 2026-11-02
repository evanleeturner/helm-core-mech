// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100640a0(fmt: *const i8, ...) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RegOpenKeyExA(
        h_key: *mut core::ffi::c_void,
        lp_sub_key: *const i8,
        ul_options: u32,
        sam_desired: u32,
        phk_result: *mut *mut core::ffi::c_void,
    ) -> i32;
    fn RegQueryValueExA(
        h_key: *mut core::ffi::c_void,
        lp_value_name: *const i8,
        lp_reserved: *mut u32,
        lp_type: *mut u32,
        lp_data: *mut u8,
        lpcb_data: *mut u32,
    ) -> i32;
    fn RegCloseKey(h_key: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10016740(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: *mut i32,
) -> u8 {
    let mut local_10: [i32; 3] = [0; 3];
    let mut local_4: *mut core::ffi::c_void = core::ptr::null_mut();
    // SAFETY: the Registry functions and the logger are the game's own and the
    // OS's; param_1..3 point at the caller's ints, written where the original
    // writes; local buffers outlive each call.
    unsafe {
        let mut l_var1 = RegOpenKeyExA(
            0x80000002usize as *mut core::ffi::c_void,
            c"Software\\Activision\\Mercenaries\\1.0".as_ptr(),
            0,
            1,
            &raw mut local_4,
        );
        if l_var1 != 0 {
            FUN_100640a0(
                c"Could not open registry MechWarrior2 key: %d\n".as_ptr(),
                l_var1,
            );
            return 0;
        }

        let base = local_10.as_mut_ptr();

        local_10[1] = 4;
        local_10[2] = 4;
        l_var1 = RegQueryValueExA(
            local_4,
            c"QuickTips".as_ptr(),
            core::ptr::null_mut(),
            base.add(2).cast::<u32>(),
            base.cast::<u8>(),
            base.add(1).cast::<u32>(),
        );
        if l_var1 == 0 {
            *param_1 = local_10[0];
        } else {
            *param_1 = 1;
        }

        local_10[1] = 4;
        local_10[2] = 4;
        l_var1 = RegQueryValueExA(
            local_4,
            c"ShowDialog".as_ptr(),
            core::ptr::null_mut(),
            base.add(2).cast::<u32>(),
            base.cast::<u8>(),
            base.add(1).cast::<u32>(),
        );
        if l_var1 == 0 {
            *param_2 = local_10[0];
        } else {
            *param_2 = 1;
        }

        local_10[1] = 4;
        local_10[2] = 4;
        l_var1 = RegQueryValueExA(
            local_4,
            c"LittleMovies".as_ptr(),
            core::ptr::null_mut(),
            base.add(2).cast::<u32>(),
            base.cast::<u8>(),
            base.add(1).cast::<u32>(),
        );
        if l_var1 == 0 && local_10[0] != -1 {
            *param_3 = local_10[0];
        } else {
            *param_3 = 1;
        }

        RegCloseKey(local_4);
        (l_var1 == 0) as u8
    }
}
