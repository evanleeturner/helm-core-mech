// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FUN_10089643(param_1: *mut u32, param_2: *mut u32) {
    mech_trace::enter!("10089643", param_1, param_2);
    // SAFETY: both pointers refer to caller-provided arrays of at least nine
    // u32 values; reads and writes follow the original's order.
    unsafe {
        mech_trace::wr!(
            "10089643",
            param_2,
            write_unaligned,
            mech_trace::rd!("10089643", param_1, read_unaligned)
        );
        mech_trace::wr!(
            "10089643",
            param_2.add(4),
            write_unaligned,
            mech_trace::rd!("10089643", param_1.add(4), read_unaligned)
        );
        mech_trace::wr!(
            "10089643",
            param_2.add(8),
            write_unaligned,
            mech_trace::rd!("10089643", param_1.add(8), read_unaligned)
        );
        let u_var1 = mech_trace::rd!("10089643", param_1.add(1), read_unaligned);
        mech_trace::wr!(
            "10089643",
            param_2.add(1),
            write_unaligned,
            mech_trace::rd!("10089643", param_1.add(3), read_unaligned)
        );
        mech_trace::wr!("10089643", param_2.add(3), write_unaligned, u_var1);
        let u_var1 = mech_trace::rd!("10089643", param_1.add(6), read_unaligned);
        mech_trace::wr!(
            "10089643",
            param_2.add(6),
            write_unaligned,
            mech_trace::rd!("10089643", param_1.add(2), read_unaligned)
        );
        mech_trace::wr!("10089643", param_2.add(2), write_unaligned, u_var1);
        let u_var1 = mech_trace::rd!("10089643", param_1.add(7), read_unaligned);
        mech_trace::wr!(
            "10089643",
            param_2.add(7),
            write_unaligned,
            mech_trace::rd!("10089643", param_1.add(5), read_unaligned)
        );
        mech_trace::wr!("10089643", param_2.add(5), write_unaligned, u_var1);
    }
}
