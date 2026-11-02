// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(a: i32, b: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100663e5(source: i32, out: i32, scale_x: i32, scale_y: i32) -> i32 {
    let s = source as u32 as *const i32;
    let o = out as u32 as *mut i32;
    // SAFETY: source and out are caller-provided addresses of rect structures;
    // reads and writes happen in the same order as the original, and the
    // callee is the game's own function.
    unsafe {
        let s4 = s.byte_add(4).read_unaligned();
        let s8 = s.byte_add(8).read_unaligned();
        let sc = s.byte_add(0xc).read_unaligned();
        let s10 = s.byte_add(0x10).read_unaligned();

        let mut center_x = sc.wrapping_sub(s4).wrapping_add(1) >> 1;
        center_x = center_x.wrapping_add(s4);
        let mut center_y = s10.wrapping_sub(s8).wrapping_add(1) >> 1;
        center_y = center_y.wrapping_add(s8);

        o.byte_add(4).write_unaligned(s4.wrapping_sub(center_x));
        o.byte_add(8).write_unaligned(s8.wrapping_sub(center_y));
        o.byte_add(0xc).write_unaligned(sc.wrapping_sub(center_x));
        o.byte_add(0x10).write_unaligned(s10.wrapping_sub(center_y));

        let v = FUN_10003d60(o.byte_add(4).read_unaligned(), scale_x);
        o.byte_add(4).write_unaligned(v);
        let v = FUN_10003d60(o.byte_add(8).read_unaligned(), scale_y);
        o.byte_add(8).write_unaligned(v);
        let v = FUN_10003d60(o.byte_add(0xc).read_unaligned(), scale_x);
        o.byte_add(0xc).write_unaligned(v);
        let v = FUN_10003d60(o.byte_add(0x10).read_unaligned(), scale_y);
        o.byte_add(0x10).write_unaligned(v);

        let v = o.byte_add(4).read_unaligned().wrapping_add(center_x);
        o.byte_add(4).write_unaligned(v);
        let v = o.byte_add(8).read_unaligned().wrapping_add(center_y);
        o.byte_add(8).write_unaligned(v);
        let v = o.byte_add(0xc).read_unaligned().wrapping_add(center_x);
        o.byte_add(0xc).write_unaligned(v);
        let v = o.byte_add(0x10).read_unaligned().wrapping_add(center_y);
        o.byte_add(0x10).write_unaligned(v);
    }
    out
}
