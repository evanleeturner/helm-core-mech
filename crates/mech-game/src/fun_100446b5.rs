// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10083a59(
        param_1: i32,
        param_2: i32,
        param_3: u32,
        param_4: *mut i32,
        param_5: *mut i32,
        param_6: *mut u32,
        param_7: *mut i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100446b5(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) -> i32 {
    mech_trace::enter!("100446b5", param_1, param_2, param_3, param_4, param_5);
    let mut local_14: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_10: u32 = 0;
    let mut local_c: i32 = 0;
    // SAFETY: param_1 and param_2 are addresses of the game's structures; the
    // fields are read where the original reads them, and the callee writes
    // only through the local out-pointers.
    unsafe {
        let local_8: u32 = (mech_trace::rd!(
            "100446b5",
            (param_2.wrapping_add(0x3c) as *const i32),
            read_unaligned
        ))
        .wrapping_sub(param_5) as u32;
        let a = mech_trace::rd!(
            "100446b5",
            (param_2.wrapping_add(0x34) as *const i32),
            read_unaligned
        )
        .wrapping_sub(param_3);
        let b = mech_trace::rd!(
            "100446b5",
            (param_2.wrapping_add(0x38) as *const i32),
            read_unaligned
        )
        .wrapping_sub(param_4);
        FUN_10083a59(
            a,
            b,
            local_8,
            &raw mut local_14,
            &raw mut local_18,
            &raw mut local_10,
            &raw mut local_c,
        );
        let f60 = mech_trace::rd!(
            "100446b5",
            (param_1.wrapping_add(0x60) as *const i32),
            read_unaligned
        );
        local_14 = local_14.wrapping_sub(f60);
        if local_14 < 0xb40001 {
            if local_14 < -0xb40000 {
                local_14 = local_14.wrapping_add(0x1680000);
            }
        } else {
            local_14 = local_14.wrapping_sub(0x1680000);
        }
        let f60b = mech_trace::rd!(
            "100446b5",
            (param_1.wrapping_add(0x60) as *const i32),
            read_unaligned
        );
        let u_var2 = ((f60b as u32) >> 16) as u16;
        mech_trace::rt!(
            "100446b5",
            if local_14 < 0 {
                ((u32::from(u_var2) << 16) | 1) as i32
            } else {
                ((u32::from(u_var2) << 16) | 0xffff) as i32
            }
        )
    }
}
