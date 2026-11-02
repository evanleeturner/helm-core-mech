// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(a: i32, b: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10065ebf(frame: i32, source: i32, out: i32) -> i32 {
    let f = frame as *const u8;
    let s = source as *const u8;
    let o = out as *mut u8;
    // SAFETY: the pointers are the caller's rect structures, read and written
    // at the same offsets as the original; the callee is the game's own.
    unsafe {
        let rd =
            |p: *const u8, off: usize| -> i32 { p.byte_add(off).cast::<i32>().read_unaligned() };
        let wr = |off: usize, v: i32| {
            o.byte_add(off).cast::<i32>().write_unaligned(v);
        };
        let i_var1 = rd(f, 0xc).wrapping_sub(rd(f, 4));
        let height = rd(f, 0x10).wrapping_sub(rd(f, 8));
        let r = FUN_10003d60(rd(s, 4), i_var1);
        wr(4, r);
        let r = FUN_10003d60(rd(s, 8), height);
        wr(8, r);
        let r = FUN_10003d60(rd(s, 0xc), i_var1);
        wr(0xc, r);
        let r = FUN_10003d60(rd(s, 0x10), height);
        wr(0x10, r);
        wr(4, rd(o, 4).wrapping_add(rd(f, 4)));
        wr(8, rd(o, 8).wrapping_add(rd(f, 8)));
        wr(0xc, rd(o, 0xc).wrapping_add(rd(f, 4)));
        wr(0x10, rd(o, 0x10).wrapping_add(rd(f, 8)));
    }
    out
}
