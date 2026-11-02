// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ad6bc: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10022aaa() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100814bb(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004cd24(param_1: u32, param_2: i32) -> u32 {
    mech_trace::enter!("1004cd24", param_1, param_2);
    let _ = param_1;
    let p2 = param_2 as usize as *mut u8;
    // SAFETY: param_2 points at the caller's structure; the allocator, callee and
    // globals are the game's own, and the allocated block is written at the
    // offsets the original uses.
    unsafe {
        mech_trace::wr!(
            "1004cd24",
            p2.byte_add(0x20).cast::<u32>(),
            write_unaligned,
            0
        );
        let size = mech_trace::cr!("1004cd24", "10022aaa", FUN_10022aaa());
        let blk = mech_trace::cr!(
            "1004cd24",
            "100814bb",
            FUN_100814bb(
                size as i32,
                mech_trace::gr!("1004cd24", "100ad6bc", DAT_100ad6bc) as i32
            )
        );
        if blk == 0 {
            return mech_trace::rt!("1004cd24", 0);
        }
        let base = blk as u32;
        let b = base as usize as *mut u8;
        mech_trace::wr!(
            "1004cd24",
            b.byte_add(0x15 * 4).cast::<u32>(),
            write_unaligned,
            base.wrapping_add(0x4d3)
        );
        mech_trace::wr!(
            "1004cd24",
            b.byte_add(0x16 * 4).cast::<u32>(),
            write_unaligned,
            base.wrapping_add(0x853)
        );
        mech_trace::wr!(
            "1004cd24",
            b.byte_add(0x17 * 4).cast::<u32>(),
            write_unaligned,
            base.wrapping_add(0x993)
        );
        mech_trace::wr!("1004cd24", b.cast::<u32>(), write_unaligned, param_2 as u32);
        let mut i: i32 = 0;
        while i < 8 {
            mech_trace::wr!(
                "1004cd24",
                b.byte_add((i.wrapping_add(0xe6) as usize).wrapping_mul(4))
                    .cast::<u32>(),
                write_unaligned,
                0
            );
            i = i.wrapping_add(1);
        }
        mech_trace::wr!(
            "1004cd24",
            p2.byte_add(0x20).cast::<u32>(),
            write_unaligned,
            base
        );
        mech_trace::wr!(
            "1004cd24",
            p2.byte_add(0x24).cast::<u32>(),
            write_unaligned,
            0x4d3
        );
        let mut l10 = mech_trace::rd!(
            "1004cd24",
            b.byte_add(0x15 * 4).cast::<u32>(),
            read_unaligned
        ) as usize as *mut u32;
        let mut l8 = b.byte_add(0x19 * 4);
        i = 0;
        while i < 0xe {
            mech_trace::wr!("1004cd24", l10, write_unaligned, 0xffff_ffff);
            mech_trace::wr!("1004cd24", l10.add(1), write_unaligned, 0xffff_ffff);
            mech_trace::wr!("1004cd24", l10.add(2), write_unaligned, 0xffff_ffff);
            mech_trace::wr!("1004cd24", l10.add(4), write_unaligned, 0);
            mech_trace::wr!("1004cd24", l10.add(5), write_unaligned, 0);
            l10 = l10.wrapping_add(0x10);
            mech_trace::wr!(
                "1004cd24",
                l8.byte_add(2).cast::<u32>(),
                write_unaligned,
                0xffff_ffff
            );
            l8 = l8.wrapping_byte_add(0x3a);
            i = i.wrapping_add(1);
        }
        mech_trace::rt!("1004cd24", 1)
    }
}
