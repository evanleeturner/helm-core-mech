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
pub unsafe extern "C" fn FUN_10066004(
    _unused_arg1: u32,
    source: *mut i32,
    out: *mut i32,
) -> *mut i32 {
    mech_trace::enter!("10066004", _unused_arg1, source, out);
    // SAFETY: the callee and the globals are the game's own; source and out
    // point at the caller's two-int buffers, read and written where the
    // original does.
    unsafe {
        let i_var1 = mech_trace::cr!(
            "10066004",
            "10003d60",
            FUN_10003d60(
                mech_trace::gr!("10066004", "101f52f4", DAT_101f52f4),
                mech_trace::rd!("10066004", source, read_unaligned)
            )
        );
        mech_trace::wr!("10066004", out, write_unaligned, i_var1);
        let i_var1 = mech_trace::cr!(
            "10066004",
            "10003d60",
            FUN_10003d60(
                mech_trace::gr!("10066004", "101f52d4", DAT_101f52d4),
                mech_trace::rd!("10066004", source.add(1), read_unaligned)
            )
        );
        mech_trace::wr!("10066004", out.add(1), write_unaligned, i_var1);
        mech_trace::rt!("10066004", out)
    }
}
