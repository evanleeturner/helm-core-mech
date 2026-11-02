// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: u32;
    static mut DAT_100b4838: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100228e4(param_1: i32) {
    mech_trace::enter!("100228e4", param_1);
    // SAFETY: param_1 is the address of the game's own structure; every
    // write lands at the offsets the original uses, and the globals are the
    // game's own memory.
    unsafe {
        let base = param_1 as usize as *mut u8;
        mech_trace::wr!(
            "100228e4",
            base.byte_add(0x54).cast::<i32>(),
            write_unaligned,
            param_1.wrapping_add(0x4d3)
        );
        mech_trace::wr!(
            "100228e4",
            base.byte_add(0x58).cast::<i32>(),
            write_unaligned,
            param_1.wrapping_add(0x853)
        );
        mech_trace::wr!(
            "100228e4",
            base.byte_add(0x5c).cast::<i32>(),
            write_unaligned,
            param_1.wrapping_add(0x993)
        );

        let mut local_10: *mut u32 = mech_trace::rd!(
            "100228e4",
            base.byte_add(0x54).cast::<i32>(),
            read_unaligned
        ) as usize as *mut u32;
        let mut local_8: i32 = param_1.wrapping_add(100);
        let mut local_18: i32 = 0;
        while local_18 < 0xe {
            mech_trace::wr!("100228e4", local_10, write_unaligned, 0xffffffff);
            mech_trace::wr!("100228e4", local_10.add(1), write_unaligned, 0xffffffff);
            mech_trace::wr!("100228e4", local_10.add(2), write_unaligned, 0xffffffff);
            mech_trace::wr!("100228e4", local_10.add(4), write_unaligned, 0);
            mech_trace::wr!("100228e4", local_10.add(5), write_unaligned, 0);
            if mech_trace::gr!("100228e4", "100aa1ac", DAT_100aa1ac) == 0
                || mech_trace::gr!("100228e4", "100b4838", DAT_100b4838) != 0xffffffff
            {
                mech_trace::wr!("100228e4", local_10.add(3), write_unaligned, 0);
            }
            mech_trace::wr!("100228e4", local_10.add(6), write_unaligned, 0xffffffff);
            mech_trace::wr!("100228e4", local_10.add(7), write_unaligned, 0);
            mech_trace::wr!("100228e4", local_10.add(9), write_unaligned, 0);
            mech_trace::wr!("100228e4", local_10.add(10), write_unaligned, 0xffffffff);
            mech_trace::wr!("100228e4", local_10.add(0xb), write_unaligned, 0);
            mech_trace::wr!("100228e4", local_10.add(0xc), write_unaligned, 0);
            mech_trace::wr!("100228e4", local_10.add(8), write_unaligned, 9999);
            local_10 = local_10.wrapping_add(0x10);
            let p = local_8 as usize as *mut u8;
            mech_trace::wr!(
                "100228e4",
                p.byte_add(2).cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            mech_trace::wr!("100228e4", p.byte_add(6).cast::<u32>(), write_unaligned, 0);
            local_8 = local_8.wrapping_add(0x3a);
            local_18 += 1;
        }

        let mut local_c: *mut u16 = mech_trace::rd!(
            "100228e4",
            base.byte_add(0x5c).cast::<i32>(),
            read_unaligned
        ) as usize as *mut u16;
        local_18 = 0;
        while local_18 < 0x19 {
            mech_trace::wr!("100228e4", local_c, write_unaligned, 0xffff);
            mech_trace::wr!("100228e4", local_c.add(1), write_unaligned, 0);
            mech_trace::wr!("100228e4", local_c.add(2), write_unaligned, 0);
            mech_trace::wr!("100228e4", local_c.add(3), write_unaligned, 0);
            mech_trace::wr!("100228e4", local_c.add(4), write_unaligned, 0);
            mech_trace::wr!("100228e4", local_c.add(5).cast::<u32>(), write_unaligned, 0);
            mech_trace::wr!("100228e4", local_c.add(7).cast::<u32>(), write_unaligned, 0);
            local_c = local_c.wrapping_add(0xf);
            local_18 += 1;
        }
    }
}
