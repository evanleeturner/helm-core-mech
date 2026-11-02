// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(from: i32, to: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10003dc4(
    from: i32,
    to: i32,
    out: i32,
    fraction: i32,
    param_5: i32,
) -> i32 {
    mech_trace::enter!("10003dc4", from, to, out, fraction, param_5);
    // SAFETY: the callee is the game's own; from, to and out are addresses of
    // the caller's records, read and written at the same offsets as the original.
    unsafe {
        if param_5 != 2 {
            let a = mech_trace::rd!(
                "10003dc4",
                ((to as u32).wrapping_add(4) as *const i32),
                read_unaligned
            );
            let b = mech_trace::rd!(
                "10003dc4",
                ((from as u32).wrapping_add(4) as *const i32),
                read_unaligned
            );
            let d = mech_trace::cr!(
                "10003dc4",
                "10003d60",
                FUN_10003d60(fraction, a.wrapping_sub(b))
            );
            let b = mech_trace::rd!(
                "10003dc4",
                ((from as u32).wrapping_add(4) as *const i32),
                read_unaligned
            );
            mech_trace::wr!(
                "10003dc4",
                ((out as u32).wrapping_add(4) as *mut i32),
                write_unaligned,
                b.wrapping_add(d)
            );

            let a = mech_trace::rd!(
                "10003dc4",
                ((to as u32).wrapping_add(0xc) as *const i32),
                read_unaligned
            );
            let b = mech_trace::rd!(
                "10003dc4",
                ((from as u32).wrapping_add(0xc) as *const i32),
                read_unaligned
            );
            let d = mech_trace::cr!(
                "10003dc4",
                "10003d60",
                FUN_10003d60(fraction, a.wrapping_sub(b))
            );
            let b = mech_trace::rd!(
                "10003dc4",
                ((from as u32).wrapping_add(0xc) as *const i32),
                read_unaligned
            );
            mech_trace::wr!(
                "10003dc4",
                ((out as u32).wrapping_add(0xc) as *mut i32),
                write_unaligned,
                b.wrapping_add(d)
            );
        }
        if param_5 != 1 {
            let a = mech_trace::rd!(
                "10003dc4",
                ((to as u32).wrapping_add(8) as *const i32),
                read_unaligned
            );
            let b = mech_trace::rd!(
                "10003dc4",
                ((from as u32).wrapping_add(8) as *const i32),
                read_unaligned
            );
            let d = mech_trace::cr!(
                "10003dc4",
                "10003d60",
                FUN_10003d60(fraction, a.wrapping_sub(b))
            );
            let b = mech_trace::rd!(
                "10003dc4",
                ((from as u32).wrapping_add(8) as *const i32),
                read_unaligned
            );
            mech_trace::wr!(
                "10003dc4",
                ((out as u32).wrapping_add(8) as *mut i32),
                write_unaligned,
                b.wrapping_add(d)
            );

            let a = mech_trace::rd!(
                "10003dc4",
                ((to as u32).wrapping_add(0x10) as *const i32),
                read_unaligned
            );
            let b = mech_trace::rd!(
                "10003dc4",
                ((from as u32).wrapping_add(0x10) as *const i32),
                read_unaligned
            );
            let d = mech_trace::cr!(
                "10003dc4",
                "10003d60",
                FUN_10003d60(fraction, a.wrapping_sub(b))
            );
            let b = mech_trace::rd!(
                "10003dc4",
                ((from as u32).wrapping_add(0x10) as *const i32),
                read_unaligned
            );
            mech_trace::wr!(
                "10003dc4",
                ((out as u32).wrapping_add(0x10) as *mut i32),
                write_unaligned,
                b.wrapping_add(d)
            );
        }
    }
    mech_trace::rt!("10003dc4", out)
}
