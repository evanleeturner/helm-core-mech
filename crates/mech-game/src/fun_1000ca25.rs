// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f64a0: [u8; 0];
    static mut DAT_101f64c0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000c8ee(param_1: i32) -> i32;
    fn FUN_1000c95b(param_1: i32) -> i32;
    fn FUN_10089cc9(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> i32;
    fn FUN_100897ef(param_1: *mut i32, param_2: *mut u32);
    fn FUN_1002f010(param_1: i32, param_2: i32) -> *mut i32;
    fn FUN_1000d659();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn FUN_1000ca25(
    param_1: i32,
    param_2: u32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    param_8: i32,
    param_9: i32,
    param_10: i32,
    param_11: i32,
    param_12: i32,
    param_13: i32,
    param_14: i32,
    param_15: u32,
    param_16: u32,
    param_17: u32,
) -> i32 {
    mech_trace::enter!(
        "1000ca25", param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8,
        param_9, param_10, param_11, param_12, param_13, param_14, param_15, param_16, param_17
    );
    let _ = param_14;
    let mut local_3c: [i32; 12] = [0; 12];
    // SAFETY: the callees and globals are the game's own; local_c points into
    // the game's entry table, written at the same offsets as the original.
    unsafe {
        let mut local_8 = mech_trace::cr!("1000ca25", "1000c8ee", FUN_1000c8ee(param_1));
        if local_8 == -1 {
            local_8 = mech_trace::cr!("1000ca25", "1000c95b", FUN_1000c95b(param_1));
        }
        if local_8 != -1 {
            let local_c: *mut u32 = (&raw mut DAT_101f64a0)
                .cast::<u8>()
                .byte_add((local_8 as u32).wrapping_mul(0x7c) as usize)
                .cast::<u32>();
            mech_trace::cr!(
                "1000ca25",
                "10089cc9",
                FUN_10089cc9(
                    local_3c.as_mut_ptr(),
                    param_6,
                    param_7,
                    param_8,
                    param_9,
                    param_10,
                    param_11,
                )
            );
            FUN_100897ef(local_3c.as_mut_ptr(), local_c.add(0x12));
            mech_trace::wr!("1000ca25", local_c.add(9), write_unaligned, param_3 as u32);
            mech_trace::wr!("1000ca25", local_c.add(10), write_unaligned, param_4 as u32);
            mech_trace::wr!("1000ca25", local_c.add(11), write_unaligned, param_5 as u32);
            mech_trace::wr!("1000ca25", local_c.add(12), write_unaligned, param_6 as u32);
            mech_trace::wr!("1000ca25", local_c.add(13), write_unaligned, param_7 as u32);
            mech_trace::wr!("1000ca25", local_c.add(14), write_unaligned, param_8 as u32);
            mech_trace::wr!("1000ca25", local_c.add(15), write_unaligned, param_9 as u32);
            mech_trace::wr!(
                "1000ca25",
                local_c.add(16),
                write_unaligned,
                param_10 as u32
            );
            mech_trace::wr!(
                "1000ca25",
                local_c.add(17),
                write_unaligned,
                param_11 as u32
            );
            mech_trace::wr!("1000ca25", local_c, write_unaligned, param_2);
            mech_trace::wr!("1000ca25", local_c.add(2), write_unaligned, param_13 as u32);
            mech_trace::wr!("1000ca25", local_c.add(1), write_unaligned, param_12 as u32);
            mech_trace::wr!("1000ca25", local_c.add(7), write_unaligned, 0);
            if param_13 == -2 {
                mech_trace::wr!("1000ca25", local_c.add(8), write_unaligned, 0);
            } else if param_13 == -1 {
                let p = mech_trace::cr!("1000ca25", "1002f010", FUN_1002f010(0, 10));
                mech_trace::wr!("1000ca25", local_c.add(8), write_unaligned, p as u32);
            } else {
                let v = mech_trace::rd!(
                    "1000ca25",
                    (&raw mut DAT_101f64c0)
                        .cast::<u8>()
                        .byte_add((param_13 as u32).wrapping_mul(0x7c) as usize)
                        .cast::<i32>(),
                    read_unaligned
                );
                let p = mech_trace::cr!("1000ca25", "1002f010", FUN_1002f010(v, 10));
                mech_trace::wr!("1000ca25", local_c.add(8), write_unaligned, p as u32);
            }
            let v3 = mech_trace::rd!("1000ca25", local_c.add(3), read_unaligned);
            mech_trace::wr!(
                "1000ca25",
                local_c.add(3),
                write_unaligned,
                v3 | (param_15 & 0x1ff)
            );
            let v3 = mech_trace::rd!("1000ca25", local_c.add(3), read_unaligned);
            mech_trace::wr!(
                "1000ca25",
                local_c.add(3),
                write_unaligned,
                v3 | ((param_16 & 0xf).wrapping_shl(0xc))
            );
            mech_trace::wr!("1000ca25", local_c.add(4), write_unaligned, param_17);
            FUN_1000d659();
        }
        mech_trace::rt!("1000ca25", local_8)
    }
}
