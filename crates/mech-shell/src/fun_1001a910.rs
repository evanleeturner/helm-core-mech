// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100361e0(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32) -> u8;
    fn FUN_10035f30(param_1: i32, param_2: *mut i8, param_3: *mut i8, param_4: *mut i8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001a910(param_1: i32, param_2: *mut i8) {
    mech_trace::enter!("1001a910", param_1, param_2);
    // SAFETY: the callees are the game's own; param_2 points at the caller's
    // structure, read at the same offsets and order as the original.
    unsafe {
        let mut i_var2: i32 = 0;
        mech_trace::cr!(
            "1001a910",
            "100361e0",
            FUN_100361e0(
                param_1,
                0,
                mech_trace::rd!(
                    "1001a910",
                    param_2.byte_add(0xc).cast::<i32>(),
                    read_unaligned
                ),
                mech_trace::rd!(
                    "1001a910",
                    param_2.byte_add(8).cast::<i32>(),
                    read_unaligned
                ),
                mech_trace::rd!(
                    "1001a910",
                    param_2.byte_add(4).cast::<i32>(),
                    read_unaligned
                ),
            )
        );
        let mut pc_var1: *mut i8 = param_2;
        if 0 < mech_trace::rd!(
            "1001a910",
            param_2.byte_add(0xc).cast::<i32>(),
            read_unaligned
        ) {
            loop {
                mech_trace::cr!(
                    "1001a910",
                    "10035f30",
                    FUN_10035f30(
                        i_var2,
                        pc_var1.byte_add(0x10),
                        core::ptr::null_mut(),
                        core::ptr::null_mut(),
                    )
                );
                i_var2 = i_var2.wrapping_add(1);
                pc_var1 = pc_var1.byte_add(0x10);
                if i_var2
                    >= mech_trace::rd!(
                        "1001a910",
                        param_2.byte_add(0xc).cast::<i32>(),
                        read_unaligned
                    )
                {
                    break;
                }
            }
        }
        mech_trace::cr!(
            "1001a910",
            "100361e0",
            FUN_100361e0(
                param_1,
                0,
                mech_trace::rd!(
                    "1001a910",
                    param_2.byte_add(0xc).cast::<i32>(),
                    read_unaligned
                ),
                mech_trace::rd!(
                    "1001a910",
                    param_2.byte_add(8).cast::<i32>(),
                    read_unaligned
                ),
                mech_trace::rd!(
                    "1001a910",
                    param_2.byte_add(4).cast::<i32>(),
                    read_unaligned
                ),
            )
        );
    }
}
