// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f5300: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10065ebf(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_100668b6(param_1: *mut i32, param_2: *mut i8, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005eafd(param_1: *mut i32, param_2: *mut u8) {
    mech_trace::enter!("1005eafd", param_1, param_2);
    let in_stack_00000018: *mut i32 = core::ptr::null_mut();

    if !param_1.is_null() && !in_stack_00000018.is_null() {
        // SAFETY: the pointers are the caller's and the game's own memory;
        // reads, writes and calls happen in the original's order.
        unsafe {
            let i_var1 = mech_trace::rd!("1005eafd", in_stack_00000018, read_unaligned);
            let i_var2 = mech_trace::rd!("1005eafd", param_1, read_unaligned);
            if i_var2 != 0 {
                let i_var3 = mech_trace::rd!("1005eafd", param_1.byte_add(0x30), read_unaligned);
                if i_var3 != 0 && !param_2.is_null() {
                    let p_var4 = mech_trace::rd!(
                        "1005eafd",
                        param_2.byte_add(8).cast::<*mut i32>(),
                        read_unaligned
                    );
                    if !p_var4.is_null() {
                        let pi_var5 =
                            mech_trace::rd!("1005eafd", p_var4, read_unaligned) as *mut i32;
                        if !pi_var5.is_null() {
                            let pc_var6 = mech_trace::rd!(
                                "1005eafd",
                                p_var4.byte_add(4).cast::<*mut i8>(),
                                read_unaligned
                            );
                            if !pc_var6.is_null() {
                                if mech_trace::rd!("1005eafd", pi_var5, read_unaligned) == 0 {
                                    mech_trace::wr!(
                                        "1005eafd",
                                        pi_var5,
                                        write_unaligned,
                                        (&raw mut DAT_101f5300) as i32
                                    );
                                    FUN_10065ebf(i_var2, pi_var5 as i32, pi_var5 as i32);
                                }
                                FUN_100668b6(pi_var5, pc_var6, i_var3);
                                let func = mech_trace::rd!(
                                    "1005eafd",
                                    param_2
                                        .byte_add(0x18)
                                        .cast::<Option<unsafe extern "C" fn(i32, i32)>>(),
                                    read_unaligned
                                );
                                if func.is_some() && i_var1 != 4 && i_var1 != 5 {
                                    let arg = mech_trace::rd!(
                                        "1005eafd",
                                        param_2.byte_add(0xc).cast::<i32>(),
                                        read_unaligned
                                    );
                                    if let Some(f) = func {
                                        mech_trace::cr!("1005eafd", "ind", f(arg, 0));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
