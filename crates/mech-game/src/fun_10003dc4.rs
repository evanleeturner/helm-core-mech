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
    // SAFETY: the callee is the game's own; from, to and out are addresses of
    // the caller's records, read and written at the same offsets as the original.
    unsafe {
        if param_5 != 2 {
            let a = ((to as u32).wrapping_add(4) as *const i32).read_unaligned();
            let b = ((from as u32).wrapping_add(4) as *const i32).read_unaligned();
            let d = FUN_10003d60(fraction, a.wrapping_sub(b));
            let b = ((from as u32).wrapping_add(4) as *const i32).read_unaligned();
            ((out as u32).wrapping_add(4) as *mut i32).write_unaligned(b.wrapping_add(d));

            let a = ((to as u32).wrapping_add(0xc) as *const i32).read_unaligned();
            let b = ((from as u32).wrapping_add(0xc) as *const i32).read_unaligned();
            let d = FUN_10003d60(fraction, a.wrapping_sub(b));
            let b = ((from as u32).wrapping_add(0xc) as *const i32).read_unaligned();
            ((out as u32).wrapping_add(0xc) as *mut i32).write_unaligned(b.wrapping_add(d));
        }
        if param_5 != 1 {
            let a = ((to as u32).wrapping_add(8) as *const i32).read_unaligned();
            let b = ((from as u32).wrapping_add(8) as *const i32).read_unaligned();
            let d = FUN_10003d60(fraction, a.wrapping_sub(b));
            let b = ((from as u32).wrapping_add(8) as *const i32).read_unaligned();
            ((out as u32).wrapping_add(8) as *mut i32).write_unaligned(b.wrapping_add(d));

            let a = ((to as u32).wrapping_add(0x10) as *const i32).read_unaligned();
            let b = ((from as u32).wrapping_add(0x10) as *const i32).read_unaligned();
            let d = FUN_10003d60(fraction, a.wrapping_sub(b));
            let b = ((from as u32).wrapping_add(0x10) as *const i32).read_unaligned();
            ((out as u32).wrapping_add(0x10) as *mut i32).write_unaligned(b.wrapping_add(d));
        }
    }
    out
}
