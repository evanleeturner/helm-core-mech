// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160be4: i32;
}

unsafe extern "C" {
    fn strncpy(dst: *mut i8, src: *const i8, n: u32) -> *mut i8;
    fn strncat(dst: *mut i8, src: *const i8, n: u32) -> *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005acaa(a: *mut i32, b: i32, c: i32, d: i32, e: *mut i8, f: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005e3fb(
    param_1: *mut i32,
    param_2: *mut u8,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: *mut u32,
) {
    mech_trace::enter!(
        "1005e3fb", param_1, param_2, param_3, param_4, param_5, param_6
    );
    let mut local_a8 = [0i8; 128];
    let mut local_28: i32 = 1;
    let local_20: i32 = 0;
    // SAFETY: all pointers are the game's own structures, accessed at the
    // offsets the original uses; function pointers are the game's callbacks.
    unsafe {
        if param_1.is_null()
            || mech_trace::dr!("1005e3fb", param_1.add(0xc)) == 0
            || param_6.is_null()
        {
            return;
        }
        let local_18 = mech_trace::dr!("1005e3fb", param_6);
        let local_24 = mech_trace::dr!("1005e3fb", param_1) as *mut i32;
        if local_24.is_null() || param_2.is_null() {
            return;
        }
        if mech_trace::rd!(
            "1005e3fb",
            param_2.byte_add(8).cast::<i32>(),
            read_unaligned
        ) == 0
        {
            return;
        }
        let local_8 = ((mech_trace::gr!("1005e3fb", "10160be4", DAT_10160be4) as u32)
            .wrapping_add(0xffffffcf) as i32
            == param_3) as u32;
        let f14 = mech_trace::rd!(
            "1005e3fb",
            param_2.byte_add(0x14).cast::<u32>(),
            read_unaligned
        );
        let mut local_c: i32;
        if (mech_trace::dr!("1005e3fb", param_2) & 1) == 0 || f14 == 0 {
            local_c = mech_trace::rd!(
                "1005e3fb",
                param_2.byte_add(4).cast::<i32>(),
                read_unaligned
            );
        } else {
            let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(f14 as usize);
            local_c = mech_trace::cr!(
                "1005e3fb",
                "ind",
                f(mech_trace::rd!(
                    "1005e3fb",
                    param_2.byte_add(0xc).cast::<i32>(),
                    read_unaligned
                ))
            );
        }
        let local_14 = (mech_trace::dr!("1005e3fb", param_6.add(4)) == param_3 as u32) as u32;
        match local_18 {
            1 => {
                let f10 = mech_trace::rd!(
                    "1005e3fb",
                    param_2.byte_add(0x10).cast::<u32>(),
                    read_unaligned
                );
                if f10 != 0 {
                    let f: unsafe extern "C" fn(*mut u32, *mut u8) =
                        core::mem::transmute(f10 as usize);
                    mech_trace::cr!("1005e3fb", "ind", f(param_6, param_2));
                }
                let f14 = mech_trace::rd!(
                    "1005e3fb",
                    param_2.byte_add(0x14).cast::<u32>(),
                    read_unaligned
                );
                if f14 == 0 {
                    mech_trace::wr!(
                        "1005e3fb",
                        param_2.byte_add(4).cast::<i32>(),
                        write_unaligned,
                        0
                    );
                    local_c = mech_trace::rd!(
                        "1005e3fb",
                        param_2.byte_add(4).cast::<i32>(),
                        read_unaligned
                    );
                } else {
                    let f: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(f14 as usize);
                    let v = mech_trace::cr!(
                        "1005e3fb",
                        "ind",
                        f(mech_trace::rd!(
                            "1005e3fb",
                            param_2.byte_add(0xc).cast::<i32>(),
                            read_unaligned
                        ))
                    );
                    mech_trace::wr!(
                        "1005e3fb",
                        param_2.byte_add(4).cast::<u32>(),
                        write_unaligned,
                        v
                    );
                    local_c = mech_trace::rd!(
                        "1005e3fb",
                        param_2.byte_add(4).cast::<i32>(),
                        read_unaligned
                    );
                }
            }
            4 => {
                let f1c = mech_trace::rd!(
                    "1005e3fb",
                    param_2.byte_add(0x1c).cast::<u32>(),
                    read_unaligned
                );
                if f1c != 0 {
                    let f: unsafe extern "C" fn(i32, i32) = core::mem::transmute(f1c as usize);
                    mech_trace::cr!(
                        "1005e3fb",
                        "ind",
                        f(
                            mech_trace::rd!(
                                "1005e3fb",
                                param_2.byte_add(0xc).cast::<i32>(),
                                read_unaligned
                            ),
                            local_c,
                        )
                    );
                }
                local_28 = 0;
            }
            5 => {
                let f20 = mech_trace::rd!(
                    "1005e3fb",
                    param_2.byte_add(0x20).cast::<u32>(),
                    read_unaligned
                );
                if f20 != 0 {
                    let f: unsafe extern "C" fn(i32) = core::mem::transmute(f20 as usize);
                    mech_trace::cr!(
                        "1005e3fb",
                        "ind",
                        f(mech_trace::rd!(
                            "1005e3fb",
                            param_2.byte_add(0xc).cast::<i32>(),
                            read_unaligned
                        ))
                    );
                }
                local_28 = 0;
            }
            _ => {}
        }
        let pi_var5 = mech_trace::rd!(
            "1005e3fb",
            param_2.byte_add(8).cast::<usize>(),
            read_unaligned
        ) as *mut i32;
        let cnt = mech_trace::rd!("1005e3fb", pi_var5.add(1), read_unaligned);
        if 0 < cnt {
            local_c = cnt.wrapping_add(local_20).wrapping_add(local_c) % cnt;
            let src = mech_trace::rd!(
                "1005e3fb",
                pi_var5.offset((2 + local_c) as isize),
                read_unaligned
            ) as usize as *const i8;
            mech_trace::cr!(
                "1005e3fb",
                "strncpy",
                strncpy(local_a8.as_mut_ptr(), src, 0x7f)
            );
            local_a8[127] = 0;
            let first = mech_trace::rd!(
                "1005e3fb",
                param_2.byte_add(8).cast::<usize>(),
                read_unaligned
            ) as *const u32;
            let fp = mech_trace::rd!("1005e3fb", first, read_unaligned);
            if fp != 0 {
                let f: unsafe extern "C" fn(*mut i32, *mut u8, i32, i32, i32, *mut u32) -> i32 =
                    core::mem::transmute(fp as usize);
                let local_10 = mech_trace::cr!(
                    "1005e3fb",
                    "ind",
                    f(param_1, param_2, param_3, param_4, param_5, param_6)
                ) as usize as *const i8;
                if !local_10.is_null() {
                    let mut len: u32 = 0;
                    while local_a8[len as usize] != 0 {
                        len += 1;
                    }
                    let local_1c = 0x7fu32.wrapping_sub(len);
                    mech_trace::cr!(
                        "1005e3fb",
                        "strncat",
                        strncat(local_a8.as_mut_ptr(), local_10, local_1c)
                    );
                    local_a8[127] = 0;
                }
            }
            FUN_1005acaa(
                local_24,
                param_4,
                param_5,
                mech_trace::dr!("1005e3fb", param_1.add(0xc)),
                local_a8.as_mut_ptr(),
                0x100eebb0,
            );
        }
        if local_14 != 0 {
            let idx = (param_3 as u32).wrapping_mul(5).wrapping_add(6) as usize;
            if local_8 != 0
                && mech_trace::dr!("1005e3fb", param_6.wrapping_add(idx)) == 5
                && mech_trace::rd!(
                    "1005e3fb",
                    param_2.byte_add(0x1c).cast::<u32>(),
                    read_unaligned
                ) != 0
            {
                let f: unsafe extern "C" fn(i32, i32) = core::mem::transmute(mech_trace::rd!(
                    "1005e3fb",
                    param_2.byte_add(0x1c).cast::<u32>(),
                    read_unaligned
                )
                    as usize);
                mech_trace::cr!(
                    "1005e3fb",
                    "ind",
                    f(
                        mech_trace::rd!(
                            "1005e3fb",
                            param_2.byte_add(0xc).cast::<i32>(),
                            read_unaligned
                        ),
                        local_c,
                    )
                );
            }
            let f18 = mech_trace::rd!(
                "1005e3fb",
                param_2.byte_add(0x18).cast::<u32>(),
                read_unaligned
            );
            if f18 != 0 && local_28 != 0 {
                let f: unsafe extern "C" fn(i32, i32) = core::mem::transmute(f18 as usize);
                mech_trace::cr!(
                    "1005e3fb",
                    "ind",
                    f(
                        mech_trace::rd!(
                            "1005e3fb",
                            param_2.byte_add(0xc).cast::<i32>(),
                            read_unaligned
                        ),
                        local_c,
                    )
                );
            }
        }
        mech_trace::wr!(
            "1005e3fb",
            param_2.byte_add(4).cast::<i32>(),
            write_unaligned,
            local_c
        );
    }
}
