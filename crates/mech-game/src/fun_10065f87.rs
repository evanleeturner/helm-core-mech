// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f52f4: i32;
    static mut DAT_101f52d4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(a: i32, b: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10065f87(
    _unused_arg1: u32,
    source: *mut i32,
    out: *mut i32,
) -> *mut i32 {
    mech_trace::enter!("10065f87", _unused_arg1, source, out);
    // SAFETY: the callee and globals are the game's own; source and out point
    // at caller-provided arrays of four i32 values, read and written in the original order.
    unsafe {
        let i_var1 = mech_trace::cr!(
            "10065f87",
            "10003d60",
            FUN_10003d60(
                mech_trace::gr!("10065f87", "101f52f4", DAT_101f52f4),
                mech_trace::rd!("10065f87", source, read_unaligned)
            )
        );
        mech_trace::wr!("10065f87", out, write_unaligned, i_var1);
        let i_var1 = mech_trace::cr!(
            "10065f87",
            "10003d60",
            FUN_10003d60(
                mech_trace::gr!("10065f87", "101f52d4", DAT_101f52d4),
                mech_trace::rd!("10065f87", source.add(1), read_unaligned)
            )
        );
        mech_trace::wr!("10065f87", out.add(1), write_unaligned, i_var1);
        let i_var1 = mech_trace::cr!(
            "10065f87",
            "10003d60",
            FUN_10003d60(
                mech_trace::gr!("10065f87", "101f52f4", DAT_101f52f4),
                mech_trace::rd!("10065f87", source.add(2), read_unaligned)
            )
        );
        mech_trace::wr!("10065f87", out.add(2), write_unaligned, i_var1);
        let i_var1 = mech_trace::cr!(
            "10065f87",
            "10003d60",
            FUN_10003d60(
                mech_trace::gr!("10065f87", "101f52d4", DAT_101f52d4),
                mech_trace::rd!("10065f87", source.add(3), read_unaligned)
            )
        );
        mech_trace::wr!("10065f87", out.add(3), write_unaligned, i_var1);
    }
    mech_trace::rt!("10065f87", out)
}
