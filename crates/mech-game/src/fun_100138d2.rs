// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ef030: u8;
    static mut DAT_101eb530: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004bbc3(param_1: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100138d2(param_1: i32, param_2: i32) {
    mech_trace::enter!("100138d2", param_1, param_2);
    // SAFETY: the globals and callee are the game's own; param_1 points at the
    // caller's record, read at the same offsets and order as the original.
    unsafe {
        let mut local_14: i32 = 0;
        let mut local_8: *const u8 = (param_1.wrapping_add(6)) as *const u8;
        let slot = (&raw mut DAT_101eb530)
            .byte_offset(param_2.wrapping_mul(4) as isize)
            .cast::<i32>();
        let src = mech_trace::rd!(
            "100138d2",
            ((param_1.wrapping_add(2)) as *const i32),
            read_unaligned
        );
        if mech_trace::rd!("100138d2", slot, read_unaligned) < src {
            let src2 = mech_trace::rd!(
                "100138d2",
                ((param_1.wrapping_add(2)) as *const i32),
                read_unaligned
            );
            mech_trace::wr!("100138d2", slot, write_unaligned, src2);
            let mut local_18: u8 = mech_trace::rd!("100138d2", local_8, read);
            let mut local_10: u32 = 0;
            while (local_10 as i32) < 0xfe {
                if (local_18 & 0x80) != 0 {
                    let flags = mech_trace::rd!(
                        "100138d2",
                        (&raw mut DAT_100ef030)
                            .byte_add(local_10.wrapping_mul(0x40) as usize)
                            .cast::<u16>(),
                        read_unaligned
                    );
                    if (flags & 4) == 0 {
                        mech_trace::cr!("100138d2", "1004bbc3", FUN_1004bbc3(local_10));
                    }
                }
                local_18 = ((local_18 as u32).wrapping_shl(1)) as u8;
                local_14 = local_14.wrapping_add(1);
                if local_14 == 8 {
                    local_14 = 0;
                    local_8 = local_8.byte_add(1);
                    local_18 = mech_trace::rd!("100138d2", local_8, read);
                }
                local_10 = local_10.wrapping_add(1);
            }
        }
    }
}
