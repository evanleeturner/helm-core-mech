// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7bc8: u8;
    static mut DAT_100a7bcc: u8;
    static mut DAT_100aa1ac: u32;
    static mut DAT_100f2fb0: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100635ce(param_1: *mut i32) {
    mech_trace::enter!("100635ce", param_1);
    // SAFETY: the globals are the game's own memory; param_1 and the
    // structures it points to are the caller's, accessed at the original's offsets.
    unsafe {
        let flags = &raw mut DAT_100f2fb0;
        let tab_c8 = &raw mut DAT_100a7bc8;
        let tab_cc = &raw mut DAT_100a7bcc;
        let mut local_10: i32 = 0;
        while local_10 < 0xe {
            let slot = flags
                .byte_add(local_10.wrapping_mul(4) as usize)
                .cast::<i32>();
            if mech_trace::rd!("100635ce", slot, read_unaligned) != 0 {
                mech_trace::wr!("100635ce", slot, write_unaligned, 0);
                let i_var2 = mech_trace::rd!("100635ce", param_1.add(0x15), read_unaligned)
                    .wrapping_add(local_10.wrapping_mul(0x40));
                let p = i_var2 as u32 as usize as *mut u8;
                let i_var1 =
                    mech_trace::rd!("100635ce", p.byte_add(4).cast::<i32>(), read_unaligned)
                        .wrapping_mul(0x68);
                if mech_trace::rd!("100635ce", p.byte_add(8).cast::<i32>(), read_unaligned) != -1
                    || mech_trace::gr!("100635ce", "100aa1ac", DAT_100aa1ac) != 0
                {
                    mech_trace::wr!("100635ce", p.byte_add(8).cast::<u32>(), write_unaligned, 2);
                    let v = mech_trace::rd!(
                        "100635ce",
                        tab_cc.byte_offset(i_var1 as isize).cast::<u32>(),
                        read_unaligned
                    );
                    mech_trace::wr!(
                        "100635ce",
                        p.byte_add(0x24).cast::<u32>(),
                        write_unaligned,
                        v
                    );
                    mech_trace::wr!(
                        "100635ce",
                        p.byte_add(0x10).cast::<u32>(),
                        write_unaligned,
                        0
                    );
                    let w = mech_trace::rd!(
                        "100635ce",
                        tab_c8.byte_offset(i_var1 as isize).cast::<i32>(),
                        read_unaligned
                    );
                    let f = mech_trace::rd!(
                        "100635ce",
                        param_1.add(0x110).cast::<u16>(),
                        read_unaligned
                    );
                    if w == 0 || (f & 0x80) == 0 {
                        mech_trace::wr!(
                            "100635ce",
                            p.byte_add(0x1c).cast::<u32>(),
                            write_unaligned,
                            0
                        );
                        let z = mech_trace::rd!(
                            "100635ce",
                            p.byte_add(0x1c).cast::<u32>(),
                            read_unaligned
                        );
                        mech_trace::wr!(
                            "100635ce",
                            p.byte_add(0x18).cast::<u32>(),
                            write_unaligned,
                            z
                        );
                    } else {
                        let q = (mech_trace::rd!("100635ce", param_1, read_unaligned)
                            .wrapping_add(0xdc)) as u32 as usize
                            as *const u32;
                        mech_trace::wr!(
                            "100635ce",
                            p.byte_add(0x18).cast::<u32>(),
                            write_unaligned,
                            mech_trace::rd!("100635ce", q, read_unaligned) & 0xff
                        );
                        let q2 = (mech_trace::rd!("100635ce", param_1, read_unaligned)
                            .wrapping_add(0xdc)) as u32 as usize
                            as *const u32;
                        mech_trace::wr!(
                            "100635ce",
                            p.byte_add(0x1c).cast::<u32>(),
                            write_unaligned,
                            mech_trace::rd!("100635ce", q2, read_unaligned) & 0xf00
                        );
                    }
                }
            }
            local_10 = local_10.wrapping_add(1);
        }
    }
}
