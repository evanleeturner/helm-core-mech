// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009bffc: i8;
    static mut DAT_1009c000: i8;
    static mut DAT_1009c004: i8;
    static mut DAT_1009c048: [i8; 1];
    static mut DAT_1009c04c: u16;
    static mut DAT_1009c038: [i8; 1];
    static mut DAT_1009c040: [i8; 1];
}

unsafe extern "C" {
    fn sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn atoi(param_1: *const i8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1004f4c0(param_1: i32, param_2: *mut i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004f4f0(
    param_1: i32,
    param_2: *mut i8,
    param_3: *mut i32,
    param_4: i32,
    param_5: *mut i32,
    param_6: *mut i32,
) -> u32 {
    mech_trace::enter!(
        "1004f4f0", param_1, param_2, param_3, param_4, param_5, param_6
    );
    // SAFETY: all pointers are the caller's buffers and the game's own globals;
    // reads and writes happen in the same order as the original.
    unsafe {
        if param_4 <= mech_trace::dr!("1004f4f0", param_3) {
            return mech_trace::rt!("1004f4f0", 0);
        }
        if mech_trace::gr!("1004f4f0", "1009c000", DAT_1009c000) == 1 {
            mech_trace::gw!("1004f4f0", "1009c000", DAT_1009c000, 0);
            mech_trace::cr!(
                "1004f4f0",
                "sprintf",
                sprintf(
                    param_2,
                    (&raw const DAT_1009c038).cast::<i8>(),
                    mech_trace::dr!("1004f4f0", param_6)
                )
            );
            return mech_trace::rt!("1004f4f0", 4);
        }
        if mech_trace::gr!("1004f4f0", "1009c004", DAT_1009c004) == 1 {
            mech_trace::gw!("1004f4f0", "1009c004", DAT_1009c004, 0);
            mech_trace::cr!(
                "1004f4f0",
                "sprintf",
                sprintf(
                    param_2,
                    (&raw const DAT_1009c040).cast::<i8>(),
                    mech_trace::dr!("1004f4f0", param_6)
                )
            );
            return mech_trace::rt!("1004f4f0", 4);
        }
        let src = (&raw const DAT_1009c048).cast::<i8>();
        let mut n: usize = 0;
        while mech_trace::dr!("1004f4f0", src.add(n)) != 0 {
            n += 1;
        }
        n += 1;
        let mut k = 0usize;
        while k < n {
            mech_trace::dw!(
                "1004f4f0",
                param_2.add(k),
                mech_trace::dr!("1004f4f0", src.add(k))
            );
            k += 1;
        }

        let mut i_var5: i32 = 0;
        let mut i_var7: i32 = mech_trace::dr!("1004f4f0", param_3);
        while i_var7 < param_4 {
            loop {
                let b = mech_trace::dr!("1004f4f0", (param_1.wrapping_add(i_var7) as *const u8));
                if b == 0x20 || b == 0x5c {
                    break;
                }
                i_var7 = i_var7.wrapping_add(1);
                mech_trace::dw!(
                    "1004f4f0",
                    param_2.offset(i_var5 as isize),
                    (b & 0x7f) as i8
                );
                i_var5 = i_var5.wrapping_add(1);
                if i_var7 >= param_4 {
                    break;
                }
            }
            if param_4 <= i_var7 {
                break;
            }
            let c = mech_trace::dr!("1004f4f0", (param_1.wrapping_add(i_var7) as *const i8));
            if (c == 0x5c || c == 0x20) && 0 < i_var5 {
                mech_trace::dw!("1004f4f0", param_2.offset(i_var5 as isize), 0);
                mech_trace::dw!("1004f4f0", param_3, i_var7);
                let mut cnt = mech_trace::cr!(
                    "1004f4f0",
                    "1004f4c0",
                    FUN_1004f4c0(param_1, param_3, param_4)
                );
                if 0 < cnt {
                    loop {
                        let mut p = param_2;
                        while mech_trace::dr!("1004f4f0", p) != 0 {
                            p = p.add(1);
                        }
                        cnt = cnt.wrapping_sub(1);
                        mech_trace::wr!(
                            "1004f4f0",
                            p.cast::<u16>(),
                            write_unaligned,
                            mech_trace::gr!("1004f4f0", "1009c04c", DAT_1009c04c)
                        );
                        if cnt == 0 {
                            break;
                        }
                    }
                }
                return mech_trace::rt!("1004f4f0", 4);
            }
            if c != 0x5c {
                return mech_trace::rt!("1004f4f0", 4);
            }
            let sel = mech_trace::dr!(
                "1004f4f0",
                (param_1.wrapping_add(1).wrapping_add(i_var7) as *const u8)
            );
            let a2 = mech_trace::dr!(
                "1004f4f0",
                (param_1.wrapping_add(i_var7).wrapping_add(2) as *const i8)
            );
            let a3 = mech_trace::dr!(
                "1004f4f0",
                (param_1.wrapping_add(i_var7).wrapping_add(3) as *const i8)
            );
            match sel {
                0x41 | 0x61 => {
                    let local_8: [i8; 4] = [a2, a3, 0, 0];
                    mech_trace::gw!("1004f4f0", "1009bffc", DAT_1009bffc, 1);
                    mech_trace::dw!(
                        "1004f4f0",
                        param_5,
                        mech_trace::cr!("1004f4f0", "atoi", atoi(local_8.as_ptr()))
                    );
                    mech_trace::dw!("1004f4f0", param_3, i_var7.wrapping_add(4));
                    mech_trace::cr!(
                        "1004f4f0",
                        "1004f4c0",
                        FUN_1004f4c0(param_1, param_3, param_4)
                    );
                    return mech_trace::rt!("1004f4f0", 6);
                }
                0x42 | 0x62 => {
                    let a4 = mech_trace::dr!(
                        "1004f4f0",
                        (param_1.wrapping_add(i_var7).wrapping_add(4) as *const i8)
                    );
                    let local_8: [i8; 4] = [a2, a3, a4, 0];
                    mech_trace::gw!("1004f4f0", "1009c000", DAT_1009c000, 1);
                    mech_trace::dw!(
                        "1004f4f0",
                        param_6,
                        mech_trace::cr!("1004f4f0", "atoi", atoi(local_8.as_ptr()))
                    );
                    mech_trace::dw!("1004f4f0", param_3, i_var7.wrapping_add(5));
                    mech_trace::cr!(
                        "1004f4f0",
                        "1004f4c0",
                        FUN_1004f4c0(param_1, param_3, param_4)
                    );
                    return mech_trace::rt!("1004f4f0", 7);
                }
                0x43 | 0x63 => {
                    mech_trace::dw!("1004f4f0", param_3, i_var7.wrapping_add(2));
                    mech_trace::cr!(
                        "1004f4f0",
                        "1004f4c0",
                        FUN_1004f4c0(param_1, param_3, param_4)
                    );
                    return mech_trace::rt!("1004f4f0", 2);
                }
                0x47 | 0x67 => {
                    let a4 = mech_trace::dr!(
                        "1004f4f0",
                        (param_1.wrapping_add(i_var7).wrapping_add(4) as *const i8)
                    );
                    let local_8: [i8; 4] = [a2, a3, a4, 0];
                    mech_trace::gw!("1004f4f0", "1009c004", DAT_1009c004, 1);
                    mech_trace::dw!(
                        "1004f4f0",
                        param_6,
                        mech_trace::cr!("1004f4f0", "atoi", atoi(local_8.as_ptr()))
                    );
                    mech_trace::dw!("1004f4f0", param_3, i_var7.wrapping_add(5));
                    mech_trace::cr!(
                        "1004f4f0",
                        "1004f4c0",
                        FUN_1004f4c0(param_1, param_3, param_4)
                    );
                    return mech_trace::rt!("1004f4f0", 8);
                }
                0x4c | 0x6c => {
                    mech_trace::dw!("1004f4f0", param_3, i_var7.wrapping_add(2));
                    mech_trace::cr!(
                        "1004f4f0",
                        "1004f4c0",
                        FUN_1004f4c0(param_1, param_3, param_4)
                    );
                    return mech_trace::rt!("1004f4f0", 9);
                }
                0x4e | 0x6e => {
                    mech_trace::dw!("1004f4f0", param_3, i_var7.wrapping_add(2));
                    mech_trace::cr!(
                        "1004f4f0",
                        "1004f4c0",
                        FUN_1004f4c0(param_1, param_3, param_4)
                    );
                    return mech_trace::rt!("1004f4f0", 1);
                }
                0x53 | 0x73 => {
                    mech_trace::dw!("1004f4f0", param_3, i_var7.wrapping_add(2));
                    mech_trace::cr!(
                        "1004f4f0",
                        "1004f4c0",
                        FUN_1004f4c0(param_1, param_3, param_4)
                    );
                    return mech_trace::rt!("1004f4f0", 3);
                }
                0x54 | 0x74 => {
                    mech_trace::dw!("1004f4f0", param_3, i_var7.wrapping_add(2));
                    mech_trace::cr!(
                        "1004f4f0",
                        "1004f4c0",
                        FUN_1004f4c0(param_1, param_3, param_4)
                    );
                    return mech_trace::rt!("1004f4f0", 5);
                }
                _ => {
                    mech_trace::dw!("1004f4f0", param_2.offset(i_var5 as isize), 0x5c);
                    i_var7 = i_var7.wrapping_add(2);
                    i_var5 = i_var5.wrapping_add(1);
                }
            }
        }
        mech_trace::dw!("1004f4f0", param_2.offset(i_var5 as isize), 0);
        mech_trace::dw!("1004f4f0", param_3, i_var7);
        mech_trace::rt!("1004f4f0", 0)
    }
}
