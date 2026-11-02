// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008f988: u8;
    static mut DAT_100c41c0: u8;
    static mut DAT_10083d8c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100361e0(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32) -> u8;
    fn FUN_10035f30(param_1: i32, param_2: *mut i8, param_3: *mut i8, param_4: *mut i8) -> u32;
    fn FUN_1001ace0(param_1: *mut i8) -> u8;
    fn FUN_100361a0(param_1: i32) -> u8;
    fn strncpy(dest: *mut i8, src: *mut i8, count: usize) -> *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10036260();
    fn FUN_10036280();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001add0() {
    mech_trace::enter!("1001add0");
    // SAFETY: all globals and callees are the game's own; record pointers
    // stay within the 6 records of 0x38 bytes starting at DAT_1008f988.
    unsafe {
        let mut record_index: i32 = 0;
        let mut record_ptr: *mut u8 = &raw mut DAT_1008f988;
        let table: *mut u32 = &raw mut DAT_10083d8c;
        loop {
            let f24 = mech_trace::rd!(
                "1001add0",
                record_ptr.byte_add(0x24).cast::<i32>(),
                read_unaligned
            );
            let f28 = mech_trace::rd!(
                "1001add0",
                record_ptr.byte_add(0x28).cast::<i32>(),
                read_unaligned
            );
            match record_index {
                0 | 1 | 2 => {
                    mech_trace::cr!("1001add0", "100361e0", FUN_100361e0(0, -1, -1, -1, -1));
                    if f28 < 0 {
                        mech_trace::cr!("1001add0", "100361a0", FUN_100361a0(f24));
                    } else {
                        let p = mech_trace::rd!(
                            "1001add0",
                            table.offset(f28.wrapping_mul(10) as isize),
                            read_unaligned
                        ) as *mut i8;
                        mech_trace::cr!(
                            "1001add0",
                            "10035f30",
                            FUN_10035f30(f24, p, core::ptr::null_mut(), core::ptr::null_mut())
                        );
                        let f2c = mech_trace::rd!(
                            "1001add0",
                            record_ptr.byte_add(0x2c).cast::<i32>(),
                            read_unaligned
                        );
                        if f2c == 1 {
                            let f28b = mech_trace::rd!(
                                "1001add0",
                                record_ptr.byte_add(0x28).cast::<i32>(),
                                read_unaligned
                            );
                            let p2 = mech_trace::rd!(
                                "1001add0",
                                table.offset(f28b.wrapping_mul(10) as isize),
                                read_unaligned
                            ) as *mut i8;
                            mech_trace::cr!("1001add0", "1001ace0", FUN_1001ace0(p2));
                        }
                    }
                    FUN_10036260();
                }
                3 | 4 | 5 => {
                    mech_trace::cr!("1001add0", "100361e0", FUN_100361e0(1, -1, -1, -1, -1));
                    let i_var2 = mech_trace::rd!(
                        "1001add0",
                        record_ptr.byte_add(0x28).cast::<i32>(),
                        read_unaligned
                    );
                    if i_var2 < 0 {
                        let mut done = false;
                        if i_var2 != -1 {
                            let mut text = [0i8; 16];
                            let f24b = mech_trace::rd!(
                                "1001add0",
                                record_ptr.byte_add(0x24).cast::<i32>(),
                                read_unaligned
                            );
                            let src =
                                (&raw mut DAT_100c41c0).byte_add(f24b.wrapping_mul(0x10) as usize);
                            mech_trace::cr!(
                                "1001add0",
                                "strncpy",
                                strncpy(text.as_mut_ptr(), src.cast::<i8>(), 3)
                            );
                            text[3] = 0;
                            if text[0] != 0 {
                                let f24c = mech_trace::rd!(
                                    "1001add0",
                                    record_ptr.byte_add(0x24).cast::<i32>(),
                                    read_unaligned
                                );
                                mech_trace::cr!(
                                    "1001add0",
                                    "10035f30",
                                    FUN_10035f30(
                                        f24c.wrapping_sub(3),
                                        text.as_mut_ptr(),
                                        core::ptr::null_mut(),
                                        core::ptr::null_mut(),
                                    )
                                );
                                done = true;
                            }
                        }
                        if !done {
                            let f24d = mech_trace::rd!(
                                "1001add0",
                                record_ptr.byte_add(0x24).cast::<i32>(),
                                read_unaligned
                            );
                            mech_trace::cr!(
                                "1001add0",
                                "100361a0",
                                FUN_100361a0(f24d.wrapping_sub(3))
                            );
                        }
                    } else {
                        let f24e = mech_trace::rd!(
                            "1001add0",
                            record_ptr.byte_add(0x24).cast::<i32>(),
                            read_unaligned
                        );
                        let p = mech_trace::rd!(
                            "1001add0",
                            table.offset(i_var2.wrapping_mul(10) as isize),
                            read_unaligned
                        ) as *mut i8;
                        mech_trace::cr!(
                            "1001add0",
                            "10035f30",
                            FUN_10035f30(
                                f24e.wrapping_sub(3),
                                p,
                                core::ptr::null_mut(),
                                core::ptr::null_mut(),
                            )
                        );
                    }
                    FUN_10036280();
                }
                _ => {}
            }
            record_ptr = record_ptr.byte_add(0x38);
            record_index = record_index.wrapping_add(1);
            if 0x1008fad7usize < record_ptr as usize {
                return;
            }
        }
    }
}
