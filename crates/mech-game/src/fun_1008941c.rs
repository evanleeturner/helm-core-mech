// SPDX-License-Identifier: MIT

fn mul64(a: i32, b: i32) -> u64 {
    (a as i64).wrapping_mul(b as i64) as u64
}

fn acc(a0: i32, a1: i32, a2: i32, b0: i32, b1: i32, b2: i32) -> u64 {
    mul64(a1, b1)
        .wrapping_add(mul64(a0, b0))
        .wrapping_add(mul64(a2, b2))
}

fn fin(v: u64) -> i32 {
    let lo = v as u32;
    let hi = (v >> 32) as u32;
    ((lo >> 0x1d) | hi.wrapping_mul(8)).wrapping_add(((lo >> 0x1c) & 1 != 0) as u32) as i32
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008941c(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32) {
    mech_trace::enter!("1008941c", param_1, param_2, param_3);
    // SAFETY: the caller supplies three 3x3 i32 matrices; all inputs are read
    // before any output is written, as in the original.
    unsafe {
        let a = |i: usize| mech_trace::rd!("1008941c", param_1.add(i), read_unaligned);
        let b = |i: usize| mech_trace::rd!("1008941c", param_2.add(i), read_unaligned);

        let l1 = acc(a(0), a(1), a(2), b(0), b(3), b(6));
        let l2 = acc(a(0), a(1), a(2), b(1), b(4), b(7));
        let l3 = acc(a(0), a(1), a(2), b(2), b(5), b(8));
        let l4 = acc(a(3), a(4), a(5), b(0), b(3), b(6));
        let l5 = acc(a(3), a(4), a(5), b(1), b(4), b(7));
        let l6 = acc(a(3), a(4), a(5), b(2), b(5), b(8));
        let l7 = acc(a(6), a(7), a(8), b(0), b(3), b(6));
        let l8 = acc(a(6), a(7), a(8), b(1), b(4), b(7));
        let l9 = acc(a(6), a(7), a(8), b(2), b(5), b(8));

        mech_trace::wr!("1008941c", param_3, write_unaligned, fin(l1));
        mech_trace::wr!("1008941c", param_3.add(1), write_unaligned, fin(l2));
        mech_trace::wr!("1008941c", param_3.add(3), write_unaligned, fin(l4));
        mech_trace::wr!("1008941c", param_3.add(4), write_unaligned, fin(l5));
        mech_trace::wr!("1008941c", param_3.add(6), write_unaligned, fin(l7));
        mech_trace::wr!("1008941c", param_3.add(7), write_unaligned, fin(l8));
        mech_trace::wr!("1008941c", param_3.add(2), write_unaligned, fin(l3));
        mech_trace::wr!("1008941c", param_3.add(5), write_unaligned, fin(l6));
        mech_trace::wr!("1008941c", param_3.add(8), write_unaligned, fin(l9));
    }
}
