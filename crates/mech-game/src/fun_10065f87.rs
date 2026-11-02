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
    // SAFETY: the callee and globals are the game's own; source and out point
    // at caller-provided arrays of four i32 values, read and written in the original order.
    unsafe {
        let i_var1 = FUN_10003d60(DAT_101f52f4, source.read_unaligned());
        out.write_unaligned(i_var1);
        let i_var1 = FUN_10003d60(DAT_101f52d4, source.add(1).read_unaligned());
        out.add(1).write_unaligned(i_var1);
        let i_var1 = FUN_10003d60(DAT_101f52f4, source.add(2).read_unaligned());
        out.add(2).write_unaligned(i_var1);
        let i_var1 = FUN_10003d60(DAT_101f52d4, source.add(3).read_unaligned());
        out.add(3).write_unaligned(i_var1);
    }
    out
}
