// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10027253(param_1: i32) {
    mech_trace::enter!("10027253", param_1);
    let base = param_1 as usize as *mut u8;
    let mut local_24: u32 = 0;
    // SAFETY: param_1 points at the game's own structure; every pointer
    // read from it is dereferenced where the original does.
    unsafe {
        let arr0 = mech_trace::rd!(
            "10027253",
            base.byte_add(0x144).cast::<*mut u32>(),
            read_unaligned
        );
        mech_trace::wr!("10027253", arr0, write_unaligned, 0);
        let mut local_14: i16 = 2;
        loop {
            if local_14 < 0 {
                return;
            }
            let mut local_c: *mut i16 = mech_trace::rd!(
                "10027253",
                base.byte_add(0x148)
                    .byte_offset((local_14 as isize) * 4)
                    .cast::<*mut i16>(),
                read_unaligned
            );
            if !local_c.is_null() {
                let mut local_1c: i16 = 1;
                let mut local_8: i16 = 0;
                let mut local_10: i16 = 0;
                let mut local_28: i16 = mech_trace::rd!("10027253", local_c, read_unaligned);
                while local_28 != 0 {
                    let key = mech_trace::rd!(
                        "10027253",
                        local_c.offset(local_1c as isize),
                        read_unaligned
                    ) as u16 as u32;
                    let cmp = mech_trace::rd!(
                        "10027253",
                        base.byte_add(0x176).cast::<i16>(),
                        read_unaligned
                    ) as i32 as u32;
                    local_24 = (key == cmp) as u32;
                    if local_24 != 0 {
                        local_8 = mech_trace::rd!(
                            "10027253",
                            local_c.offset(local_1c.wrapping_add(1) as isize),
                            read_unaligned
                        );
                        break;
                    }
                    local_10 = mech_trace::rd!(
                        "10027253",
                        local_c.offset(local_1c.wrapping_add(1) as isize),
                        read_unaligned
                    )
                    .wrapping_add(local_10);
                    local_1c = local_1c.wrapping_add(2);
                    local_28 = local_28.wrapping_sub(1);
                }
                if local_24 != 0 {
                    local_1c = 0;
                    let first = mech_trace::rd!("10027253", local_c, read_unaligned) as i32;
                    let off = first
                        .wrapping_mul(4)
                        .wrapping_add(2)
                        .wrapping_add((local_10 as i32).wrapping_mul(0xc))
                        as i16 as i32;
                    local_c = (off.wrapping_add(local_c as usize as i32)) as usize as *mut i16;
                    while local_1c < local_8
                        && mech_trace::rd!("10027253", local_c, read_unaligned) != 0
                    {
                        let mut local_20: i16 = 0;
                        local_24 = 0;
                        loop {
                            if local_24 != 0 {
                                break;
                            }
                            let arr = mech_trace::rd!(
                                "10027253",
                                base.byte_add(0x144).cast::<*mut u8>(),
                                read_unaligned
                            );
                            let e = mech_trace::rd!(
                                "10027253",
                                arr.byte_offset((local_20 as isize) * 4).cast::<*mut i16>(),
                                read_unaligned
                            );
                            if e.is_null() {
                                break;
                            }
                            let arr2 = mech_trace::rd!(
                                "10027253",
                                base.byte_add(0x144).cast::<*mut u8>(),
                                read_unaligned
                            );
                            let e2 = mech_trace::rd!(
                                "10027253",
                                arr2.byte_offset((local_20 as isize) * 4).cast::<*mut i16>(),
                                read_unaligned
                            );
                            if mech_trace::rd!("10027253", e2.byte_add(2), read_unaligned)
                                == mech_trace::rd!("10027253", local_c.add(1), read_unaligned)
                            {
                                let arr3 = mech_trace::rd!(
                                    "10027253",
                                    base.byte_add(0x144).cast::<*mut u8>(),
                                    read_unaligned
                                );
                                let e3 = mech_trace::rd!(
                                    "10027253",
                                    arr3.byte_offset((local_20 as isize) * 4).cast::<*mut i16>(),
                                    read_unaligned
                                );
                                if mech_trace::rd!("10027253", e3, read_unaligned)
                                    == mech_trace::rd!("10027253", local_c, read_unaligned)
                                {
                                    local_24 = 1;
                                }
                            }
                            local_20 = local_20.wrapping_add(1);
                        }
                        if local_24 == 0 {
                            let arr = mech_trace::rd!(
                                "10027253",
                                base.byte_add(0x144).cast::<*mut u8>(),
                                read_unaligned
                            );
                            mech_trace::wr!(
                                "10027253",
                                arr.byte_offset((local_20 as isize) * 4).cast::<*mut i16>(),
                                write_unaligned,
                                local_c
                            );
                            let arr = mech_trace::rd!(
                                "10027253",
                                base.byte_add(0x144).cast::<*mut u8>(),
                                read_unaligned
                            );
                            mech_trace::wr!(
                                "10027253",
                                arr.byte_add(4)
                                    .byte_offset((local_20 as isize) * 4)
                                    .cast::<u32>(),
                                write_unaligned,
                                0
                            );
                        }
                        local_1c = local_1c.wrapping_add(1);
                        local_c = local_c.add(6);
                    }
                }
            }
            local_14 = local_14.wrapping_sub(1);
        }
    }
}
