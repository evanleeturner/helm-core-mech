// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1002637d(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002650e(param_1: *mut i32, param_2: i32) {
    mech_trace::enter!("1002650e", param_1, param_2);
    // SAFETY: param_1 points at the caller's record of at least 0xd ints; the
    // callees are the game's own functions.
    unsafe {
        if mech_trace::rd!("1002650e", param_1.add(0xd), read_unaligned) == 0 {
            mech_trace::cr!(
                "1002650e",
                "1002637d",
                FUN_1002637d(param_1 as usize as i32)
            );
        }
        if mech_trace::rd!("1002650e", param_1.add(0xd), read_unaligned) != 0 {
            let mut i_var1 = mech_trace::cr!(
                "1002650e",
                "10003d60",
                FUN_10003d60(
                    param_2,
                    mech_trace::rd!("1002650e", param_1.add(9), read_unaligned)
                )
            );
            mech_trace::wr!(
                "1002650e",
                param_1.add(3),
                write_unaligned,
                mech_trace::rd!("1002650e", param_1, read_unaligned).wrapping_add(i_var1)
            );
            i_var1 = mech_trace::cr!(
                "1002650e",
                "10003d60",
                FUN_10003d60(
                    param_2,
                    mech_trace::rd!("1002650e", param_1.add(10), read_unaligned)
                )
            );
            mech_trace::wr!(
                "1002650e",
                param_1.add(4),
                write_unaligned,
                mech_trace::rd!("1002650e", param_1.add(1), read_unaligned).wrapping_add(i_var1)
            );
            i_var1 = mech_trace::cr!(
                "1002650e",
                "10003d60",
                FUN_10003d60(
                    param_2,
                    mech_trace::rd!("1002650e", param_1.add(0xb), read_unaligned)
                )
            );
            mech_trace::wr!(
                "1002650e",
                param_1.add(5),
                write_unaligned,
                mech_trace::rd!("1002650e", param_1.add(2), read_unaligned).wrapping_add(i_var1)
            );
            mech_trace::wr!(
                "1002650e",
                param_1.add(6),
                write_unaligned,
                mech_trace::rd!("1002650e", param_1.add(3), read_unaligned)
                    .wrapping_sub(mech_trace::rd!("1002650e", param_1, read_unaligned))
            );
            mech_trace::wr!(
                "1002650e",
                param_1.add(7),
                write_unaligned,
                mech_trace::rd!("1002650e", param_1.add(4), read_unaligned)
                    .wrapping_sub(mech_trace::rd!("1002650e", param_1.add(1), read_unaligned))
            );
            mech_trace::wr!(
                "1002650e",
                param_1.add(8),
                write_unaligned,
                mech_trace::rd!("1002650e", param_1.add(5), read_unaligned)
                    .wrapping_sub(mech_trace::rd!("1002650e", param_1.add(2), read_unaligned))
            );
            mech_trace::wr!("1002650e", param_1.add(0xc), write_unaligned, param_2);
        }
    }
}
