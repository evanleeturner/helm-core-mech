// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(segment: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002628c(
    segment: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    length: i32,
) {
    mech_trace::enter!(
        "1002628c", segment, param_2, param_3, param_4, param_5, param_6, param_7, length
    );
    // SAFETY: segment points at the caller's array of at least 14 ints; the
    // callee is the game's own function.
    unsafe {
        mech_trace::wr!("1002628c", segment, write_unaligned, param_2);
        mech_trace::wr!("1002628c", segment.add(1), write_unaligned, param_3);
        mech_trace::wr!("1002628c", segment.add(2), write_unaligned, param_4);
        mech_trace::wr!("1002628c", segment.add(9), write_unaligned, param_5);
        mech_trace::wr!("1002628c", segment.add(10), write_unaligned, param_6);
        mech_trace::wr!("1002628c", segment.add(11), write_unaligned, param_7);
        let i_var1 = mech_trace::cr!("1002628c", "10003d60", FUN_10003d60(length, param_5));
        mech_trace::wr!("1002628c", segment.add(6), write_unaligned, i_var1);
        let i_var1 = mech_trace::cr!("1002628c", "10003d60", FUN_10003d60(length, param_6));
        mech_trace::wr!("1002628c", segment.add(7), write_unaligned, i_var1);
        let i_var1 = mech_trace::cr!("1002628c", "10003d60", FUN_10003d60(length, param_7));
        mech_trace::wr!("1002628c", segment.add(8), write_unaligned, i_var1);
        mech_trace::wr!(
            "1002628c",
            segment.add(3),
            write_unaligned,
            mech_trace::rd!("1002628c", segment.add(6), read_unaligned).wrapping_add(param_2)
        );
        mech_trace::wr!(
            "1002628c",
            segment.add(4),
            write_unaligned,
            mech_trace::rd!("1002628c", segment.add(7), read_unaligned).wrapping_add(param_3)
        );
        mech_trace::wr!(
            "1002628c",
            segment.add(5),
            write_unaligned,
            mech_trace::rd!("1002628c", segment.add(8), read_unaligned).wrapping_add(param_4)
        );
        mech_trace::wr!("1002628c", segment.add(0xc), write_unaligned, length);
        mech_trace::wr!("1002628c", segment.add(0xd), write_unaligned, 1);
    }
}
