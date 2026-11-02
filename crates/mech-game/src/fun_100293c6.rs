// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: i16;
    static mut DAT_100eef40: [i32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d69b(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100293c6(param_1: i32) -> u32 {
    mech_trace::enter!("100293c6", param_1);
    // SAFETY: param_1 points at the caller's record, and the globals and the
    // callee are the game's own; every pointer is read where the original reads it.
    unsafe {
        let s_var2: i16 = mech_trace::rd!(
            "100293c6",
            ((param_1.wrapping_add(4)) as *const i16),
            read_unaligned
        );
        let mut local_c: i16 = 0;
        let mut local_10: *const i32 = (&raw mut DAT_100eef40).cast::<i32>();
        let mut ret: i32 = 0;

        loop {
            if mech_trace::gr!("100293c6", "100b38d8", DAT_100b38d8) <= local_c {
                return mech_trace::rt!(
                    "100293c6",
                    (((local_c as i32) >> 15).wrapping_shl(16) as u32) | 0xffff
                );
            }
            let p = mech_trace::rd!("100293c6", local_10, read_unaligned);
            if p != 0 {
                let flags = mech_trace::rd!(
                    "100293c6",
                    (p.wrapping_add(0x14) as *const u16),
                    read_unaligned
                );
                if (flags & 6) == 0 {
                    let v = mech_trace::rd!(
                        "100293c6",
                        (p.wrapping_add(0x17a) as *const u16),
                        read_unaligned
                    );
                    if (v as u32) == ((s_var2 as i32 as u32) | 0x200) {
                        let a = mech_trace::rd!(
                            "100293c6",
                            (p.wrapping_add(4) as *const i32),
                            read_unaligned
                        );
                        ret =
                            mech_trace::cr!("100293c6", "1004d69b", FUN_1004d69b(a, s_var2 as i32));
                        if ret == 0
                            && mech_trace::rd!(
                                "100293c6",
                                (p.wrapping_add(0x176) as *const i16),
                                read_unaligned
                            ) == 3
                        {
                            break;
                        }
                    }
                }
            }
            local_c = local_c.wrapping_add(1);
            local_10 = local_10.add(1);
        }
        let p = mech_trace::rd!("100293c6", local_10, read_unaligned);
        let hi = ((ret >> 16) as i16 as i32 as u32).wrapping_shl(16);
        let lo = mech_trace::rd!(
            "100293c6",
            (p.wrapping_add(4) as *const i32),
            read_unaligned
        ) as i16 as i32 as u32;
        mech_trace::rt!("100293c6", (hi | lo) | 0x200)
    }
}
