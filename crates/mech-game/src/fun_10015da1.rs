// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100660e1(param_1: i32, param_2: *mut i32, param_3: *mut i32);
}

unsafe fn rd(base: i32, off: i32) -> i32 {
    // SAFETY: caller guarantees base points at the game's object.
    unsafe {
        mech_trace::rd!(
            "10015da1",
            (base.wrapping_add(off) as *const i32),
            read_unaligned
        )
    }
}

unsafe fn wr(base: i32, off: i32, v: i32) {
    // SAFETY: caller guarantees base points at the game's object.
    unsafe {
        mech_trace::wr!(
            "10015da1",
            (base.wrapping_add(off) as *mut i32),
            write_unaligned,
            v
        )
    }
}

unsafe fn rs(base: i32, off: i32) -> i32 {
    // SAFETY: caller guarantees base points at the game's object.
    unsafe {
        mech_trace::rd!(
            "10015da1",
            (base.wrapping_add(off) as *const i16),
            read_unaligned
        ) as i32
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10015da1(param_1: i32) {
    mech_trace::enter!("10015da1", param_1);
    // SAFETY: param_1 is the game's object; the callee is the game's own.
    unsafe {
        wr(param_1, 0x5c, 0x1e7a);
        wr(param_1, 0x60, 0x8dc9);
        wr(param_1, 0x54, 0);
        wr(param_1, 0x58, 0);
        FUN_100660e1(
            rd(param_1, 0xa4),
            param_1.wrapping_add(0x5c) as *mut i32,
            param_1.wrapping_add(0x5c) as *mut i32,
        );
        FUN_100660e1(
            rd(param_1, 0xa4),
            param_1.wrapping_add(0x54) as *mut i32,
            param_1.wrapping_add(0x54) as *mut i32,
        );
        wr(param_1, 0x64, 0);
        let t = rd(param_1, 0x60);
        let v = (rs(param_1, 0xba) as u32)
            .wrapping_sub((t as u32).wrapping_add((t / 2) as u32))
            .wrapping_add(0xFFFFFFFD);
        wr(param_1, 0x68, v as i32);
        wr(param_1, 0x6c, rd(param_1, 0x5c));
        wr(param_1, 0x70, rs(param_1, 0xba).wrapping_add(-1));
        wr(param_1, 0x54, rd(param_1, 0x64).wrapping_add(1));
        wr(param_1, 0x58, rs(param_1, 0xba).wrapping_add(-2));
        let w = (rd(param_1, 0x70) as u32)
            .wrapping_sub(rd(param_1, 0x68) as u32)
            .wrapping_add(0xFFFFFFFE);
        wr(param_1, 0x60, w as i32);
    }
}
