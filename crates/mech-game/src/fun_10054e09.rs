// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10054f6b(param_1: *mut i16, param_2: *mut u32) -> u8;
    fn FUN_10019e33(param_1: i32, param_2: *mut i32, param_3: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10054e09(param_1: *mut i16) {
    mech_trace::enter!("10054e09", param_1);
    let mut local_100: [u32; 60] = [0; 60];
    // SAFETY: param_1 points at the game's record: count, flags, then a table of
    // pointers at offset 12; each entry points at a record read at fixed offsets.
    // The output buffer is written where the original writes it.
    unsafe {
        let mut local_c: i32 = mech_trace::rd!("10054e09", param_1, read) as i32;
        let mut local_10: *const i32 = param_1.add(6).cast::<i32>();
        let mut local_104: *mut u32 = local_100.as_mut_ptr();
        let mut local_108: i32 = local_c;
        loop {
            if local_108 == 0 {
                break;
            }
            let local_8: i32 = mech_trace::rd!("10054e09", local_10, read_unaligned);
            local_10 = local_10.add(1);
            let base = local_8 as usize as *const u8;
            let u_var1: u32 =
                mech_trace::rd!("10054e09", base.byte_add(0xc).cast::<u32>(), read_unaligned);
            let u_var2: u32 = mech_trace::rd!(
                "10054e09",
                base.byte_add(0x10).cast::<u32>(),
                read_unaligned
            );
            if (u_var1 as i32) < -0x3fff
                || 0x3ffe < (u_var1 as i32)
                || (u_var2 as i32) < -0x3fff
                || 0x3ffe < (u_var2 as i32)
            {
                local_c = i32::from(mech_trace::cr!(
                    "10054e09",
                    "10054f6b",
                    FUN_10054f6b(param_1, local_100.as_mut_ptr())
                ));
                break;
            }
            mech_trace::wr!("10054e09", local_104, write_unaligned, u_var1);
            mech_trace::wr!("10054e09", local_104.add(1), write_unaligned, u_var2);
            mech_trace::wr!(
                "10054e09",
                local_104.add(3),
                write_unaligned,
                mech_trace::rd!(
                    "10054e09",
                    base.byte_add(0x14).cast::<u32>(),
                    read_unaligned
                )
            );
            mech_trace::wr!(
                "10054e09",
                local_104.add(4),
                write_unaligned,
                mech_trace::rd!(
                    "10054e09",
                    base.byte_add(0x18).cast::<u32>(),
                    read_unaligned
                )
            );
            mech_trace::wr!(
                "10054e09",
                local_104.add(2),
                write_unaligned,
                mech_trace::rd!(
                    "10054e09",
                    base.byte_add(0x2c).cast::<u32>(),
                    read_unaligned
                )
            );
            mech_trace::wr!(
                "10054e09",
                local_104.add(5),
                write_unaligned,
                mech_trace::rd!("10054e09", base.byte_add(8).cast::<u32>(), read_unaligned)
            );
            local_104 = local_104.add(6);
            local_108 = local_108.wrapping_sub(1);
        }
        if 0 < local_c {
            mech_trace::cr!(
                "10054e09",
                "10019e33",
                FUN_10019e33(
                    local_c,
                    local_100.as_mut_ptr().cast::<i32>(),
                    u32::from(mech_trace::rd!(
                        "10054e09",
                        param_1.add(1).cast::<u16>(),
                        read
                    )),
                )
            );
        }
    }
}
