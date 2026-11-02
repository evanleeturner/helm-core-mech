// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100261f0(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    mech_trace::enter!(
        "100261f0", param_1, param_2, param_3, param_4, param_5, param_6, param_7
    );
    // SAFETY: param_1 points at the caller's buffer of at least 14 ints,
    // written in the same order as the original.
    unsafe {
        mech_trace::wr!("100261f0", param_1, write_unaligned, param_2);
        mech_trace::wr!("100261f0", param_1.add(1), write_unaligned, param_3);
        mech_trace::wr!("100261f0", param_1.add(2), write_unaligned, param_4);
        mech_trace::wr!("100261f0", param_1.add(3), write_unaligned, param_5);
        mech_trace::wr!("100261f0", param_1.add(4), write_unaligned, param_6);
        mech_trace::wr!("100261f0", param_1.add(5), write_unaligned, param_7);
        mech_trace::wr!(
            "100261f0",
            param_1.add(6),
            write_unaligned,
            (param_5 as u32).wrapping_sub(param_2 as u32) as i32
        );
        mech_trace::wr!(
            "100261f0",
            param_1.add(7),
            write_unaligned,
            (param_6 as u32).wrapping_sub(param_3 as u32) as i32
        );
        mech_trace::wr!(
            "100261f0",
            param_1.add(8),
            write_unaligned,
            (param_7 as u32).wrapping_sub(param_4 as u32) as i32
        );
        mech_trace::wr!("100261f0", param_1.add(0xc), write_unaligned, 0);
        let v = mech_trace::rd!("100261f0", param_1.add(0xc), read_unaligned);
        mech_trace::wr!("100261f0", param_1.add(0xb), write_unaligned, v);
        let v = mech_trace::rd!("100261f0", param_1.add(0xb), read_unaligned);
        mech_trace::wr!("100261f0", param_1.add(10), write_unaligned, v);
        let v = mech_trace::rd!("100261f0", param_1.add(10), read_unaligned);
        mech_trace::wr!("100261f0", param_1.add(9), write_unaligned, v);
        mech_trace::wr!("100261f0", param_1.add(0xd), write_unaligned, 0);
    }
}
