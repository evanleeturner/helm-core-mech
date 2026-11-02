// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008975d(param_1: *mut u32) {
    mech_trace::enter!("1008975d", param_1);
    // SAFETY: param_1 points at the caller's array of at least 12 u32 values,
    // written in the same order as the original.
    unsafe {
        mech_trace::wr!("1008975d", param_1.add(8), write_unaligned, 0x20000000);
        mech_trace::wr!(
            "1008975d",
            param_1.add(4),
            write_unaligned,
            mech_trace::rd!("1008975d", param_1.add(8), read_unaligned)
        );
        mech_trace::wr!(
            "1008975d",
            param_1,
            write_unaligned,
            mech_trace::rd!("1008975d", param_1.add(4), read_unaligned)
        );
        mech_trace::wr!("1008975d", param_1.add(9), write_unaligned, 0);
        mech_trace::wr!(
            "1008975d",
            param_1.add(6),
            write_unaligned,
            mech_trace::rd!("1008975d", param_1.add(9), read_unaligned)
        );
        mech_trace::wr!(
            "1008975d",
            param_1.add(3),
            write_unaligned,
            mech_trace::rd!("1008975d", param_1.add(6), read_unaligned)
        );
        mech_trace::wr!("1008975d", param_1.add(10), write_unaligned, 0);
        mech_trace::wr!(
            "1008975d",
            param_1.add(7),
            write_unaligned,
            mech_trace::rd!("1008975d", param_1.add(10), read_unaligned)
        );
        mech_trace::wr!(
            "1008975d",
            param_1.add(1),
            write_unaligned,
            mech_trace::rd!("1008975d", param_1.add(7), read_unaligned)
        );
        mech_trace::wr!("1008975d", param_1.add(11), write_unaligned, 0);
        mech_trace::wr!(
            "1008975d",
            param_1.add(5),
            write_unaligned,
            mech_trace::rd!("1008975d", param_1.add(11), read_unaligned)
        );
        mech_trace::wr!(
            "1008975d",
            param_1.add(2),
            write_unaligned,
            mech_trace::rd!("1008975d", param_1.add(5), read_unaligned)
        );
    }
}
