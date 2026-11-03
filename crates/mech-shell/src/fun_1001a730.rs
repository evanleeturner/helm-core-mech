// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a67a4: i32;
    static mut DAT_100a6798: i32;
    static mut DAT_1008f980: i32;
    static mut DAT_1008f964: i32;
    static mut DAT_1008fb74: [i8; 1];
    static mut DAT_1008fb88: [i8; 1];
    static mut DAT_1008fb84: [i8; 1];
    static mut DAT_1008fb94: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100366c0();
    fn FUN_10018060(a: i32, b: i32) -> i32;
    fn FUN_1000ad30(a: i32, b: i32) -> i32;
    fn FUN_1005bbb0(a: *mut i8) -> *mut i8;
    fn _strncmp(a: *const i8, b: *const i8, n: u32) -> i32;
    fn FUN_1001a480(a: *mut i8, b: *mut i8, c: *mut i8, d: i32, e: i32, f: i32, g: i32, h: i32);
    fn FUN_10036780(a: *const i8);
    fn FID_conflict___wfopen(a: *const i8, b: *const i8) -> *mut u8;
    fn FUN_1006f790(buf: *mut u8, size: u32, count: u32, file: *mut u8) -> u32;
    fn _fclose(file: *mut u8) -> i32;
    fn _sprintf(dest: *mut i8, fmt: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001a730(param_1: i32, param_2: i32, param_3: i32, param_4: *mut i32) {
    mech_trace::enter!("1001a730", param_1, param_2, param_3, param_4);
    let mut matched: i32 = -1;
    // SAFETY: the globals, callees and record buffers belong to the game;
    // pointers are read and written exactly where the original does.
    unsafe {
        if 0 < param_1 {
            let mut outer: i32 = 0;
            FUN_100366c0();
            let mut passed: i32 = 0;
            let mut cursor = param_2.wrapping_add(4) as *mut i8;
            loop {
                if -1
                    < mech_trace::rd!("1001a730", cursor.byte_sub(4).cast::<i32>(), read_unaligned)
                {
                    let mut i3: i32 = 0;
                    if 0 < mech_trace::rd!(
                        "1001a730",
                        ((mech_trace::gr!("1001a730", "100a67a4", DAT_100a67a4).wrapping_add(4))
                            as *const i32),
                        read_unaligned
                    ) {
                        loop {
                            let i1 = mech_trace::cr!(
                                "1001a730",
                                "10018060",
                                FUN_10018060(
                                    mech_trace::gr!("1001a730", "100a67a4", DAT_100a67a4),
                                    i3
                                )
                            );
                            if mech_trace::rd!(
                                "1001a730",
                                ((i1.wrapping_add(0x20)) as *const i32),
                                read_unaligned
                            ) != -1
                            {
                                let i2 = mech_trace::cr!(
                                    "1001a730",
                                    "1000ad30",
                                    FUN_1000ad30(
                                        mech_trace::gr!("1001a730", "100a6798", DAT_100a6798),
                                        mech_trace::rd!(
                                            "1001a730",
                                            ((i1.wrapping_add(0x20)) as *const i32),
                                            read_unaligned
                                        ),
                                    )
                                );
                                let n: u32 = 8;
                                let entry = mech_trace::cr!(
                                    "1001a730",
                                    "1005bbb0",
                                    FUN_1005bbb0(i2.wrapping_add(0x823) as *mut i8)
                                );
                                let rec =
                                    mech_trace::cr!("1001a730", "1005bbb0", FUN_1005bbb0(cursor));
                                let r = mech_trace::cr!(
                                    "1001a730",
                                    "_strncmp",
                                    _strncmp(rec, entry, n)
                                );
                                if r == 0 {
                                    matched = mech_trace::rd!(
                                        "1001a730",
                                        ((i1.wrapping_add(0x24)) as *const i32),
                                        read_unaligned
                                    );
                                    break;
                                }
                            }
                            i3 = i3.wrapping_add(1);
                            if !(i3
                                < mech_trace::rd!(
                                    "1001a730",
                                    ((mech_trace::gr!("1001a730", "100a67a4", DAT_100a67a4)
                                        .wrapping_add(4))
                                        as *const i32),
                                    read_unaligned
                                ))
                            {
                                break;
                            }
                        }
                    }
                    FUN_1001a480(
                        cursor,
                        cursor.wrapping_add(0x10),
                        cursor.wrapping_add(0x26),
                        outer,
                        0,
                        0,
                        matched,
                        -1,
                    );
                    passed = passed.wrapping_add(1);
                    if param_1 <= passed {
                        break;
                    }
                }
                cursor = cursor.wrapping_add(0x40);
                outer = outer.wrapping_add(1);
                if !(outer < 3) {
                    break;
                }
            }
            FUN_10036780((&raw mut DAT_1008fb74).cast::<i8>());
        }
        if 0 < param_3 {
            let mut outer: i32 = 1;
            let mut dif_bytes = [0u8; 0x17];
            let mut name = [0i8; 16];
            loop {
                let i3 = mech_trace::gr!("1001a730", "1008f980", DAT_1008f980);
                let mut i1 = (mech_trace::gr!("1001a730", "1008f980", DAT_1008f980) as u32)
                    .wrapping_sub(2) as i32;
                let f = mech_trace::cr!(
                    "1001a730",
                    "FID_conflict___wfopen",
                    FID_conflict___wfopen(
                        (&raw mut DAT_1008fb88).cast::<i8>(),
                        (&raw mut DAT_1008fb84).cast::<i8>(),
                    )
                );
                if !f.is_null() {
                    mech_trace::cr!(
                        "1001a730",
                        "1006f790",
                        FUN_1006f790(dif_bytes.as_mut_ptr(), 0x17, 1, f)
                    );
                    mech_trace::cr!("1001a730", "_fclose", _fclose(f));
                    if dif_bytes[5] as i8 == 0 {
                        i1 = (i3 as u32).wrapping_sub(1) as i32;
                    } else if dif_bytes[5] as i8 == 2 {
                        i1 = (i3 as u32).wrapping_sub(3) as i32;
                    }
                }
                if -1 < mech_trace::gr!("1001a730", "1008f964", DAT_1008f964) {
                    i1 = mech_trace::gr!("1001a730", "1008f964", DAT_1008f964);
                }
                let mut i3: i32 = 0;
                FUN_100366c0();
                let mut passed: i32 = 0;
                let mut ec = param_4;
                loop {
                    if -1 < mech_trace::rd!("1001a730", ec, read_unaligned) {
                        FUN_1001a480(
                            ec.wrapping_add(1).cast::<i8>(),
                            ec.wrapping_add(5).cast::<i8>(),
                            ec.byte_add(0x2a).cast::<i8>(),
                            i3,
                            outer,
                            i1,
                            -1,
                            -1,
                        );
                        passed = passed.wrapping_add(1);
                        if param_3 <= passed {
                            break;
                        }
                    }
                    ec = ec.wrapping_add(0x10);
                    i3 = i3.wrapping_add(1);
                    if !(i3 < 3) {
                        break;
                    }
                }
                let next = outer.wrapping_add(1);
                mech_trace::cr!(
                    "1001a730",
                    "_sprintf",
                    _sprintf(
                        name.as_mut_ptr(),
                        (&raw mut DAT_1008fb94).cast::<i8>(),
                        outer,
                    )
                );
                FUN_10036780(name.as_ptr());
                outer = next;
                if !(next < 6) {
                    break;
                }
            }
        }
    }
}
