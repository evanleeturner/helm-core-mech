// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
#[inline(always)]
fn fix(a0: i32, b0: i32, a1: i32, b1: i32, a2: i32, b2: i32, add: u32) -> u32 {
    let s: u64 = ((a0 as i64).wrapping_mul(b0 as i64) as u64)
        .wrapping_add((a1 as i64).wrapping_mul(b1 as i64) as u64)
        .wrapping_add((a2 as i64).wrapping_mul(b2 as i64) as u64);
    let lo = s as u32;
    let hi = (s >> 32) as u32;
    ((lo >> 0x1d) | hi.wrapping_mul(8))
        .wrapping_add(add)
        .wrapping_add(((lo >> 0x1c) & 1 != 0) as u32)
}

#[inline(always)]
unsafe fn rd(p: *const i32, i: usize) -> i32 {
    // SAFETY: caller guarantees the pointed-to memory is readable.
    unsafe { mech_trace::rd!("1001b550", p.add(i), read_unaligned) }
}

#[inline(always)]
unsafe fn wr(p: *mut i32, i: usize, v: u32) {
    // SAFETY: caller guarantees the pointed-to memory is writable.
    unsafe { mech_trace::wr!("1001b550", p.add(i), write_unaligned, v as i32) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001b550(param_1: i32, param_2: *mut i32) {
    mech_trace::enter!("1001b550", param_1, param_2);
    let m = param_2 as *const i32;
    // SAFETY: param_1 is the address of the game's data block and param_2 a
    // 12-int matrix; all reads and writes follow the original's layout.
    unsafe {
        let mut pi2 = (param_1.wrapping_add(0x18)) as u32 as usize as *mut i32;
        let off = mech_trace::rd!(
            "1001b550",
            ((param_1 as usize as *const u8).byte_add(8) as *const i32),
            read_unaligned
        );
        let mut i_var3: u32 = (off as u32).wrapping_add(param_1 as u32);
        let mut local_c: i16 = mech_trace::rd!(
            "1001b550",
            ((param_1 as usize as *const u8).byte_add(4) as *const i16),
            read_unaligned
        );
        let mut local_18: i16 = mech_trace::rd!(
            "1001b550",
            ((param_1 as usize as *const u8).byte_add(6) as *const i16),
            read_unaligned
        );
        let p = pi2 as *const i32;
        let _ = p;
        loop {
            let q = pi2 as *const i32;
            let v = fix(
                rd(m, 1),
                rd(q, 1),
                rd(m, 0),
                rd(q, 0),
                rd(m, 2),
                rd(q, 2),
                rd(m, 9) as u32,
            );
            wr(pi2, 3, v);
            let v = fix(
                rd(m, 4),
                rd(q, 1),
                rd(m, 3),
                rd(q, 0),
                rd(m, 5),
                rd(q, 2),
                rd(m, 10) as u32,
            );
            wr(pi2, 4, v);
            let v = fix(
                rd(m, 7),
                rd(q, 1),
                rd(m, 6),
                rd(q, 0),
                rd(m, 8),
                rd(q, 2),
                rd(m, 11) as u32,
            );
            wr(pi2, 5, v);
            let v = fix(
                rd(m, 1),
                rd(q, 0xc),
                rd(m, 0),
                rd(q, 0xb),
                rd(m, 2),
                rd(q, 0xd),
                0,
            );
            wr(pi2, 0xe, v);
            let v = fix(
                rd(m, 4),
                rd(q, 0xc),
                rd(m, 3),
                rd(q, 0xb),
                rd(m, 5),
                rd(q, 0xd),
                0,
            );
            wr(pi2, 0xf, v);
            let v = fix(
                rd(m, 7),
                rd(q, 0xc),
                rd(m, 6),
                rd(q, 0xb),
                rd(m, 8),
                rd(q, 0xd),
                0,
            );
            wr(pi2, 0x10, v);
            pi2 = pi2.add(0x11);
            local_c = local_c.wrapping_sub(1);
            if local_c == 0 {
                break;
            }
        }
        loop {
            let b = i_var3 as usize as *mut i32;
            let q = b as *const i32;
            // offsets in ints: 8 -> 2, 0xc -> 3, 0x10 -> 4, 0x14 -> 5, 0x18 -> 6, 0x1c -> 7
            let v = fix(
                rd(m, 1),
                mech_trace::rd!("1001b550", q.byte_add(0xc), read_unaligned),
                rd(m, 0),
                mech_trace::rd!("1001b550", q.byte_add(8), read_unaligned),
                rd(m, 2),
                mech_trace::rd!("1001b550", q.byte_add(0x10), read_unaligned),
                0,
            );
            mech_trace::wr!("1001b550", b.byte_add(0x14), write_unaligned, v as i32);
            let v = fix(
                rd(m, 4),
                mech_trace::rd!("1001b550", q.byte_add(0xc), read_unaligned),
                rd(m, 3),
                mech_trace::rd!("1001b550", q.byte_add(8), read_unaligned),
                rd(m, 5),
                mech_trace::rd!("1001b550", q.byte_add(0x10), read_unaligned),
                0,
            );
            mech_trace::wr!("1001b550", b.byte_add(0x18), write_unaligned, v as i32);
            let v = fix(
                rd(m, 7),
                mech_trace::rd!("1001b550", q.byte_add(0xc), read_unaligned),
                rd(m, 6),
                mech_trace::rd!("1001b550", q.byte_add(8), read_unaligned),
                rd(m, 8),
                mech_trace::rd!("1001b550", q.byte_add(0x10), read_unaligned),
                0,
            );
            mech_trace::wr!("1001b550", b.byte_add(0x1c), write_unaligned, v as i32);
            i_var3 = i_var3.wrapping_add(0x24);
            local_18 = local_18.wrapping_sub(1);
            if local_18 == 0 {
                break;
            }
        }
    }
}
