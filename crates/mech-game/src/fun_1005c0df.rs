// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: i32;
    static mut DAT_101686f0: i32;
    static mut DAT_100af0b4: *mut i32;
    static mut DAT_100af0b8: *mut i32;
    static mut DAT_101f5300: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: i32, param_3: *mut i32) -> *mut u16;
    fn FUN_10066ad9(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_10066342(param_1: *mut core::ffi::c_void, param_2: *mut i32, param_3: i32);
    fn FUN_10066dc9(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_100668b6(param_1: *mut i32, param_2: *mut i8, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005c0df(
    param_1: i32,
    param_2: i32,
    param_3: *mut i8,
    param_4: i32,
    param_5: i32,
) {
    let mut local_34: [i32; 5] = [0; 5];
    let mut local_1c: [i32; 5] = [0; 5];
    let mut local_20: *mut u16 = core::ptr::null_mut();

    if param_3.is_null() {
        return;
    }
    // SAFETY: the globals and callees are the game's own; the locals live
    // for the whole block and their addresses are passed as the original does.
    unsafe {
        if param_1 != -1 {
            local_20 = FUN_1008668b(
                DAT_100af17c,
                DAT_101686f0.wrapping_add(param_1),
                DAT_100af0b4,
            );
        }
        let local_8 = FUN_1008668b(
            DAT_100af17c,
            DAT_101686f0.wrapping_add(param_2),
            DAT_100af0b8,
        );
        if local_8.is_null() {
            return;
        }
        local_1c[0] = (&raw mut DAT_101f5300) as i32;
        FUN_10066ad9(param_3, local_8 as i32, local_1c.as_mut_ptr() as i32);

        if param_4 < 0 {
            local_34 = local_1c;
            FUN_10066342(
                (&raw mut DAT_101f5300).cast(),
                local_34.as_mut_ptr(),
                local_34.as_mut_ptr() as i32,
            );
            local_1c[1] = local_34[1];
            local_1c[3] = local_34[3];
        } else {
            local_1c[1] = local_1c[1].wrapping_add(param_4);
            local_1c[3] = local_1c[3].wrapping_add(param_4);
        }

        if param_5 < 0 {
            local_34 = local_1c;
            FUN_10066342(
                (&raw mut DAT_101f5300).cast(),
                local_34.as_mut_ptr(),
                local_34.as_mut_ptr() as i32,
            );
            local_1c[2] = local_34[2];
            local_1c[4] = local_34[4];
        } else {
            local_1c[2] = local_1c[2].wrapping_add(param_5);
            local_1c[4] = local_1c[4].wrapping_add(param_5);
        }

        if !local_20.is_null() {
            FUN_10066dc9(local_1c.as_mut_ptr(), local_20 as i32, 0);
        }
        FUN_100668b6(local_1c.as_mut_ptr(), param_3, local_8 as i32);
    }
}
