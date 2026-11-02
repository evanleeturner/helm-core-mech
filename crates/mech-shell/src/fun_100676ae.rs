// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a8a81: [u8; 4];
    static mut DAT_100a8a85: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10067837(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100676ae(param_1: i32, param_2: u8, param_3: i32) {
    mech_trace::enter!("100676ae", param_1, param_2, param_3);
    // SAFETY: the globals and callee are the game's own; the byte pointer
    // read from the global is walked exactly as the original walks it.
    unsafe {
        let mut p: *mut u8 = mech_trace::rd!(
            "100676ae",
            (&raw const DAT_100a8a81).cast::<*mut u8>(),
            read_unaligned
        );
        mech_trace::wr!(
            "100676ae",
            (&raw mut DAT_100a8a85).cast::<*mut u8>(),
            write_unaligned,
            p
        );
        mech_trace::cr!("100676ae", "10067837", FUN_10067837(0, 0, param_3));
        let mut local_8: i32 = 5;
        if param_1 != 0 {
            let mut b2: u8 = mech_trace::rd!("100676ae", p, read);
            p = p.byte_add(1);
            let mut i5: i32 = param_1.wrapping_sub(1);
            let mut i3: i32 = ((b2 as u32) << 8) as i32;
            let mut u4: u32 = 0;
            // 0=outer top, 1=A, 2=B, 3=DD, 4=AA, 5=end
            let mut state: u32 = 0;
            loop {
                match state {
                    0 => {
                        if b2 == param_2 {
                            state = 3;
                        } else {
                            state = 1;
                        }
                    }
                    1 => {
                        local_8 = 1;
                        if i5 == 0 {
                            break;
                        }
                        b2 = mech_trace::rd!("100676ae", p, read);
                        p = p.byte_add(1);
                        i5 = i5.wrapping_sub(1);
                        let b1 = b2 ^ ((i3 as u32) >> 8) as u8;
                        u4 = ((b2 as u32) << 8) | b1 as u32;
                        if b1 == 0 {
                            state = 4;
                        } else if b2 == param_2 {
                            mech_trace::wr!(
                                "100676ae",
                                (&raw mut DAT_100a8a85).cast::<*mut u8>(),
                                write_unaligned,
                                p
                            );
                            i3 = mech_trace::cr!(
                                "100676ae",
                                "10067837",
                                FUN_10067837(1, 1, param_3)
                            );
                            state = 3;
                        } else {
                            state = 2;
                        }
                    }
                    2 => {
                        let mut next: u32 = 3;
                        loop {
                            let p7 = p;
                            i3 = i5;
                            p = p7;
                            if i3 == 0 {
                                next = 5;
                                break;
                            }
                            b2 = mech_trace::rd!("100676ae", p7, read);
                            p = p7.byte_add(1);
                            let mut b1 = b2 ^ (u4 >> 8) as u8;
                            if b2 == param_2 {
                                mech_trace::wr!(
                                    "100676ae",
                                    (&raw mut DAT_100a8a85).cast::<*mut u8>(),
                                    write_unaligned,
                                    p
                                );
                                i5 = i3;
                                i3 = mech_trace::cr!(
                                    "100676ae",
                                    "10067837",
                                    FUN_10067837(1, 1, param_3)
                                );
                                next = 3;
                                break;
                            }
                            u4 = ((b2 as u32) << 8) | b1 as u32;
                            i5 = i3.wrapping_sub(1);
                            if b1 == 0 {
                                if i3.wrapping_sub(1) == 0 {
                                    next = 5;
                                    break;
                                }
                                b1 = mech_trace::rd!("100676ae", p, read);
                                p = p7.byte_add(2);
                                mech_trace::wr!(
                                    "100676ae",
                                    (&raw mut DAT_100a8a85).cast::<*mut u8>(),
                                    write_unaligned,
                                    p
                                );
                                if b1 == param_2 {
                                    i3 = mech_trace::cr!(
                                        "100676ae",
                                        "10067837",
                                        FUN_10067837(1, 1, param_3)
                                    );
                                    next = 3;
                                    break;
                                }
                                u4 = ((b1 as u32) << 8) | (b1 ^ b2) as u32;
                                i5 = i3.wrapping_sub(2);
                                if (b1 ^ b2) == 0 {
                                    u4 = mech_trace::cr!(
                                        "100676ae",
                                        "10067837",
                                        FUN_10067837(1, 3, param_3)
                                    ) as u32;
                                    next = 4;
                                    break;
                                }
                            }
                        }
                        state = next;
                    }
                    3 => {
                        loop {
                            local_8 = 3;
                            if i5 == 0 {
                                state = 5;
                                break;
                            }
                            b2 = mech_trace::rd!("100676ae", p, read);
                            p = p.byte_add(1);
                            i5 = i5.wrapping_sub(1);
                            b2 ^= ((i3 as u32) >> 8) as u8;
                            i3 = ((((i3 as u32) >> 8) << 8) | b2 as u32) as i32;
                            if b2 != 0 {
                                state = 6;
                                break;
                            }
                        }
                        if state == 5 {
                            break;
                        }
                        mech_trace::wr!(
                            "100676ae",
                            (&raw mut DAT_100a8a85).cast::<*mut u8>(),
                            write_unaligned,
                            p
                        );
                        i3 = mech_trace::cr!("100676ae", "10067837", FUN_10067837(3, 1, param_3));
                        state = 1;
                    }
                    4 => {
                        loop {
                            local_8 = 2;
                            if i5 == 0 {
                                state = 5;
                                break;
                            }
                            b2 = mech_trace::rd!("100676ae", p, read);
                            p = p.byte_add(1);
                            i5 = i5.wrapping_sub(1);
                            b2 ^= (u4 >> 8) as u8;
                            u4 = (((u4 >> 8) & 0xff_ffff) << 8) | b2 as u32;
                            if b2 != 0 {
                                state = 6;
                                break;
                            }
                        }
                        if state == 5 {
                            break;
                        }
                        mech_trace::wr!(
                            "100676ae",
                            (&raw mut DAT_100a8a85).cast::<*mut u8>(),
                            write_unaligned,
                            p
                        );
                        i3 = mech_trace::cr!("100676ae", "10067837", FUN_10067837(2, 1, param_3));
                        b2 = ((i3 as u32) >> 8) as u8;
                        state = 0;
                    }
                    _ => break,
                }
            }
        }
        mech_trace::wr!(
            "100676ae",
            (&raw mut DAT_100a8a85).cast::<*mut u8>(),
            write_unaligned,
            p
        );
        mech_trace::cr!("100676ae", "10067837", FUN_10067837(local_8, 0, param_3));
        mech_trace::cr!("100676ae", "10067837", FUN_10067837(4, 0, param_3));
    }
}
