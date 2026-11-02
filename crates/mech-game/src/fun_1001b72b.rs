// SPDX-License-Identifier: MIT

#[inline(always)]
unsafe fn rd(p: *const u8, off: usize) -> i32 {
    // SAFETY: the caller guarantees p points at a block at least off+4 bytes long.
    unsafe { mech_trace::rd!("1001b72b", p.byte_add(off).cast::<i32>(), read_unaligned) }
}

#[inline(always)]
unsafe fn calc(p1: *const u8, p2: *const i32, a: usize, b: usize, c: usize, add: usize) -> u32 {
    // SAFETY: both pointers refer to the caller's structures, read at the original's offsets.
    unsafe {
        let m1 = (mech_trace::rd!("1001b72b", p2.add(a), read_unaligned) as i64 as u64)
            .wrapping_mul(rd(p1, 0x2c) as i64 as u64);
        let m0 = (mech_trace::rd!("1001b72b", p2.add(b), read_unaligned) as i64 as u64)
            .wrapping_mul(rd(p1, 0x28) as i64 as u64);
        let m2 = (mech_trace::rd!("1001b72b", p2.add(c), read_unaligned) as i64 as u64)
            .wrapping_mul(rd(p1, 0x30) as i64 as u64);
        let v = m1.wrapping_add(m0).wrapping_add(m2);
        let lo = v as u32;
        let hi = (v >> 32) as u32;
        ((lo >> 0x1d) | hi.wrapping_mul(8))
            .wrapping_add(mech_trace::rd!("1001b72b", p2.add(add), read_unaligned) as u32)
            .wrapping_add(((lo >> 0x1c) & 1 != 0) as u32)
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001b72b(param_1: *mut u16, param_2: *mut i32) {
    mech_trace::enter!("1001b72b", param_1, param_2);
    // SAFETY: param_1 and param_2 point at the caller's structures; reads and
    // writes occur at the same offsets and in the same order as the original.
    unsafe {
        mech_trace::wr!(
            "1001b72b",
            param_1,
            write_unaligned,
            mech_trace::rd!("1001b72b", param_1, read_unaligned) & 0xfdff
        );
        let p1 = param_1 as *mut u8;
        let p2 = param_2 as *const i32;

        let r = calc(p1, p2, 1, 0, 2, 9);
        mech_trace::wr!(
            "1001b72b",
            p1.byte_add(0x34).cast::<u32>(),
            write_unaligned,
            r
        );

        let r = calc(p1, p2, 4, 3, 5, 10);
        mech_trace::wr!(
            "1001b72b",
            p1.byte_add(0x38).cast::<u32>(),
            write_unaligned,
            r
        );

        let r = calc(p1, p2, 7, 6, 8, 11);
        mech_trace::wr!(
            "1001b72b",
            p1.byte_add(0x3c).cast::<u32>(),
            write_unaligned,
            r
        );

        let c = p1.byte_add(0x50).cast::<u32>();
        mech_trace::wr!(
            "1001b72b",
            c,
            write_unaligned,
            mech_trace::rd!("1001b72b", c, read_unaligned).wrapping_add(1)
        );
    }
}
