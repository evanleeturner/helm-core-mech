// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009025c: u32;
    static mut DAT_10090260: u8;
    static mut DAT_100a6798: i32;
    static mut DAT_100a67a4: i32;
    static mut DAT_100a67a8: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10035e90();
    fn FUN_100361c0();
    fn FUN_10036260();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100361e0(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_1000ad30(param_1: i32, param_2: i32) -> i32;
    fn FUN_1000b9a0(param_1: i32) -> u32;
    fn FUN_10035f30(param_1: i32, param_2: *mut i8, param_3: *mut i8, param_4: *mut i8) -> u32;
    fn FUN_1000ae80(param_1: *mut i8) -> *mut i8;
}

unsafe fn strncpy_local(dest: *mut i8, src: *const i8, count: usize) {
    // SAFETY: caller guarantees dest holds count bytes and src is readable
    // up to its terminator or count bytes.
    unsafe {
        let mut i = 0usize;
        while i < count {
            let c = mech_trace::dr!("1001d650", src.add(i));
            mech_trace::dw!("1001d650", dest.add(i), c);
            i += 1;
            if c == 0 {
                break;
            }
        }
        while i < count {
            mech_trace::dw!("1001d650", dest.add(i), 0);
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001d650() {
    mech_trace::enter!("1001d650");
    // SAFETY: callees and globals are the game's own; item pointers come
    // from the game's collections and are accessed at the original offsets.
    unsafe {
        let mut local_38: [i8; 23] = [0; 23];
        let mut local_20: [i8; 10] = [0; 10];

        let mut i_var5: i32 = 1;
        FUN_10035e90();
        FUN_100361c0();
        let mut i_var6: i32 = 0;
        FUN_100361e0(0, -1, 3, 0, -1);
        let mut i_var4: i32 = i_var5;
        if 0 < mech_trace::rd!(
            "1001d650",
            ((mech_trace::gr!("1001d650", "100a67a4", DAT_100a67a4).wrapping_add(4)) as *const i32),
            read_unaligned
        ) {
            loop {
                let pc2 = mech_trace::cr!(
                    "1001d650",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1001d650", "100a67a4", DAT_100a67a4),
                        i_var6
                    )
                ) as *mut i8;
                i_var5 = i_var4;
                let key =
                    mech_trace::rd!("1001d650", pc2.byte_add(0x20).cast::<i32>(), read_unaligned);
                if key != -1 {
                    let i_var3 = mech_trace::cr!(
                        "1001d650",
                        "1000ad30",
                        FUN_1000ad30(mech_trace::gr!("1001d650", "100a6798", DAT_100a6798), key)
                    );
                    mech_trace::cr!("1001d650", "1000b9a0", FUN_1000b9a0(i_var3));
                    mech_trace::dw!("1001d650", ((i_var3.wrapping_add(0x84b)) as *mut u8), 1);
                    strncpy_local(
                        local_20.as_mut_ptr(),
                        (i_var3.wrapping_add(0x823)) as *const i8,
                        8,
                    );
                    local_20[9] = 0;
                    strncpy_local(local_38.as_mut_ptr(), pc2, 0x15);
                    local_38[22] = 0;
                    if mech_trace::dr!("1001d650", pc2.byte_add(0x2064)) == 1 {
                        i_var4 = 0;
                    } else {
                        i_var5 = i_var4.wrapping_add(1);
                    }
                    mech_trace::cr!(
                        "1001d650",
                        "10035f30",
                        FUN_10035f30(
                            i_var4,
                            local_20.as_mut_ptr(),
                            local_38.as_mut_ptr(),
                            core::ptr::null_mut(),
                        )
                    );
                }
                i_var6 = i_var6.wrapping_add(1);
                i_var4 = i_var5;
                if !(i_var6
                    < mech_trace::rd!(
                        "1001d650",
                        ((mech_trace::gr!("1001d650", "100a67a4", DAT_100a67a4).wrapping_add(4))
                            as *const i32),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
        i_var4 = 0;
        if 0 < mech_trace::rd!(
            "1001d650",
            ((mech_trace::gr!("1001d650", "100a67a8", DAT_100a67a8).wrapping_add(4)) as *const i32),
            read_unaligned
        ) {
            loop {
                let mut pc2 = mech_trace::cr!(
                    "1001d650",
                    "10018060",
                    FUN_10018060(
                        mech_trace::gr!("1001d650", "100a67a8", DAT_100a67a8),
                        i_var4
                    )
                ) as *mut i8;
                i_var6 = i_var5;
                if mech_trace::dr!("1001d650", pc2.byte_add(0x2c)) == 1 && i_var5 < 4 {
                    strncpy_local(local_38.as_mut_ptr(), pc2, 0x15);
                    local_38[22] = 0;
                    pc2 = mech_trace::cr!("1001d650", "1000ae80", FUN_1000ae80(pc2.byte_add(0x4d)));
                    if !pc2.is_null() {
                        mech_trace::dw!("1001d650", pc2.byte_add(0x828), b'M' as i8);
                        mech_trace::dw!("1001d650", pc2.byte_add(0x829), b'E' as i8);
                        mech_trace::dw!("1001d650", pc2.byte_add(0x82a), b'R' as i8);
                        let mut p = pc2.byte_add(0x823);
                        while mech_trace::dr!("1001d650", p) != 0 {
                            p = p.add(1);
                        }
                        mech_trace::wr!(
                            "1001d650",
                            p.cast::<u32>(),
                            write_unaligned,
                            mech_trace::gr!("1001d650", "1009025c", DAT_1009025c)
                        );
                        mech_trace::dw!(
                            "1001d650",
                            p.add(4),
                            mech_trace::gr!("1001d650", "10090260", DAT_10090260) as i8
                        );
                        mech_trace::cr!("1001d650", "1000b9a0", FUN_1000b9a0(pc2 as i32));
                        i_var6 = i_var5.wrapping_add(1);
                        mech_trace::cr!(
                            "1001d650",
                            "10035f30",
                            FUN_10035f30(
                                i_var5,
                                pc2.byte_add(0x823),
                                local_38.as_mut_ptr(),
                                core::ptr::null_mut(),
                            )
                        );
                    }
                }
                i_var4 = i_var4.wrapping_add(1);
                i_var5 = i_var6;
                if !(i_var4
                    < mech_trace::rd!(
                        "1001d650",
                        ((mech_trace::gr!("1001d650", "100a67a8", DAT_100a67a8).wrapping_add(4))
                            as *const i32),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
        FUN_10036260();
    }
}
