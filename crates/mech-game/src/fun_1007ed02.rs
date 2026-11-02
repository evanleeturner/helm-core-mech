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
pub unsafe extern "C" fn FUN_1007ed02(param_1: u32, param_2: i32) -> u32 {
    mech_trace::enter!("1007ed02", param_1, param_2);
    let _ = param_1;
    let p2 = param_2 as usize as *mut u8;
    // SAFETY: the callees and globals are the game's own; param_2 points at
    // the caller's structure and the allocation returned by the game's
    // allocator is large enough for the offsets the original uses.
    unsafe {
        mech_trace::wr!(
            "1007ed02",
            p2.byte_add(0x20).cast::<u32>(),
            write_unaligned,
            0
        );
        let i_var1 = mech_trace::cr!("1007ed02", "10022aaa", FUN_10022aaa()) as i32;
        let base = mech_trace::cr!(
            "1007ed02",
            "100814bb",
            FUN_100814bb(
                i_var1,
                mech_trace::gr!("1007ed02", "100ad6bc", DAT_100ad6bc) as i32
            )
        ) as usize as *mut u8;
        if base.is_null() {
            return mech_trace::rt!("1007ed02", 0);
        }
        let b = base as usize as i32;
        mech_trace::wr!(
            "1007ed02",
            base.byte_add(0x15 * 4).cast::<i32>(),
            write_unaligned,
            b.wrapping_add(0x4d3)
        );
        mech_trace::wr!(
            "1007ed02",
            base.byte_add(0x16 * 4).cast::<i32>(),
            write_unaligned,
            b.wrapping_add(0x853)
        );
        mech_trace::wr!(
            "1007ed02",
            base.byte_add(0x17 * 4).cast::<i32>(),
            write_unaligned,
            b.wrapping_add(0x993)
        );
        mech_trace::wr!("1007ed02", base.cast::<i32>(), write_unaligned, param_2);
        let mut i: i32 = 0;
        while i < 8 {
            mech_trace::wr!(
                "1007ed02",
                base.byte_add(((i as usize).wrapping_add(0xe6)).wrapping_mul(4))
                    .cast::<i32>(),
                write_unaligned,
                0
            );
            i = i.wrapping_add(1);
        }
        mech_trace::wr!(
            "1007ed02",
            p2.byte_add(0x20).cast::<i32>(),
            write_unaligned,
            b
        );
        mech_trace::wr!(
            "1007ed02",
            p2.byte_add(0x24).cast::<u32>(),
            write_unaligned,
            0x4d3
        );
        let mut local_10 = mech_trace::rd!(
            "1007ed02",
            base.byte_add(0x15 * 4).cast::<i32>(),
            read_unaligned
        ) as usize as *mut u8;
        let mut local_8 = base.byte_add(0x19 * 4);
        i = 0;
        while i < 0xe {
            mech_trace::wr!(
                "1007ed02",
                local_10.cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            mech_trace::wr!(
                "1007ed02",
                local_10.byte_add(4).cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            mech_trace::wr!(
                "1007ed02",
                local_10.byte_add(8).cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            mech_trace::wr!(
                "1007ed02",
                local_10.byte_add(16).cast::<u32>(),
                write_unaligned,
                0
            );
            mech_trace::wr!(
                "1007ed02",
                local_10.byte_add(20).cast::<u32>(),
                write_unaligned,
                0
            );
            local_10 = local_10.byte_add(0x40);
            mech_trace::wr!(
                "1007ed02",
                local_8.byte_add(2).cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            local_8 = local_8.byte_add(0x3a);
            i = i.wrapping_add(1);
        }
        mech_trace::rt!("1007ed02", 1)
    }
}
