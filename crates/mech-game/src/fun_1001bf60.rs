// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f6e8(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001bf60(
    param_1: i32,
    param_2: *mut i32,
    param_3: *mut i32,
    param_4: *mut i32,
    param_5: *mut i32,
    param_6: *mut i32,
    param_7: *mut i32,
) {
    mech_trace::enter!(
        "1001bf60", param_1, param_2, param_3, param_4, param_5, param_6, param_7
    );
    // SAFETY: all pointers are the caller's; param_1 and the structures it
    // links to are the game's own memory, read at the original's offsets.
    unsafe {
        let i_var1 = mech_trace::rd!(
            "1001bf60",
            (param_1.wrapping_add(0x1c) as *const i32),
            read_unaligned
        );
        if i_var1 == 0 {
            mech_trace::wr!("1001bf60", param_7, write_unaligned, 0);
            mech_trace::wr!(
                "1001bf60",
                param_6,
                write_unaligned,
                mech_trace::rd!("1001bf60", param_7, read_unaligned)
            );
            mech_trace::wr!(
                "1001bf60",
                param_5,
                write_unaligned,
                mech_trace::rd!("1001bf60", param_6, read_unaligned)
            );
            mech_trace::wr!(
                "1001bf60",
                param_4,
                write_unaligned,
                mech_trace::rd!("1001bf60", param_5, read_unaligned)
            );
            mech_trace::wr!(
                "1001bf60",
                param_3,
                write_unaligned,
                mech_trace::rd!("1001bf60", param_4, read_unaligned)
            );
            mech_trace::wr!(
                "1001bf60",
                param_2,
                write_unaligned,
                mech_trace::rd!("1001bf60", param_3, read_unaligned)
            );
        } else {
            if mech_trace::rd!(
                "1001bf60",
                (param_1.wrapping_add(0x50) as *const i32),
                read_unaligned
            ) != mech_trace::rd!(
                "1001bf60",
                (i_var1.wrapping_add(0x10) as *const i32),
                read_unaligned
            ) {
                let u_var2 = mech_trace::rd!(
                    "1001bf60",
                    (param_1.wrapping_add(0x20) as *const u32),
                    read_unaligned
                );
                mech_trace::wr!(
                    "1001bf60",
                    (param_1.wrapping_add(0x20) as *mut i32),
                    write_unaligned,
                    i_var1
                );
                mech_trace::cr!("1001bf60", "1002f6e8", FUN_1002f6e8(param_1));
                mech_trace::wr!(
                    "1001bf60",
                    (param_1.wrapping_add(0x20) as *mut u32),
                    write_unaligned,
                    u_var2
                );
            }
            let mut local_c: i32 = i_var1.wrapping_add(0x18);
            mech_trace::wr!("1001bf60", param_6, write_unaligned, 0x7fffffff);
            mech_trace::wr!(
                "1001bf60",
                param_4,
                write_unaligned,
                mech_trace::rd!("1001bf60", param_6, read_unaligned)
            );
            mech_trace::wr!(
                "1001bf60",
                param_2,
                write_unaligned,
                mech_trace::rd!("1001bf60", param_4, read_unaligned)
            );
            mech_trace::wr!("1001bf60", param_7, write_unaligned, -0x7fffffff);
            mech_trace::wr!(
                "1001bf60",
                param_5,
                write_unaligned,
                mech_trace::rd!("1001bf60", param_7, read_unaligned)
            );
            mech_trace::wr!(
                "1001bf60",
                param_3,
                write_unaligned,
                mech_trace::rd!("1001bf60", param_5, read_unaligned)
            );
            let mut local_10: i32 = mech_trace::rd!(
                "1001bf60",
                (i_var1.wrapping_add(4) as *const i16),
                read_unaligned
            ) as i32;
            loop {
                if local_10 == 0 {
                    break;
                }
                let a = mech_trace::rd!(
                    "1001bf60",
                    (local_c.wrapping_add(0xc) as *const i32),
                    read_unaligned
                );
                let b = mech_trace::rd!(
                    "1001bf60",
                    (local_c.wrapping_add(0x10) as *const i32),
                    read_unaligned
                );
                let c = mech_trace::rd!(
                    "1001bf60",
                    (local_c.wrapping_add(0x14) as *const i32),
                    read_unaligned
                );
                if mech_trace::rd!("1001bf60", param_3, read_unaligned) < a {
                    mech_trace::wr!("1001bf60", param_3, write_unaligned, a);
                }
                if mech_trace::rd!("1001bf60", param_5, read_unaligned) < b {
                    mech_trace::wr!("1001bf60", param_5, write_unaligned, b);
                }
                if mech_trace::rd!("1001bf60", param_7, read_unaligned) < c {
                    mech_trace::wr!("1001bf60", param_7, write_unaligned, c);
                }
                if a < mech_trace::rd!("1001bf60", param_2, read_unaligned) {
                    mech_trace::wr!("1001bf60", param_2, write_unaligned, a);
                }
                if b < mech_trace::rd!("1001bf60", param_4, read_unaligned) {
                    mech_trace::wr!("1001bf60", param_4, write_unaligned, b);
                }
                if c < mech_trace::rd!("1001bf60", param_6, read_unaligned) {
                    mech_trace::wr!("1001bf60", param_6, write_unaligned, c);
                }
                local_c = local_c.wrapping_add(0x44);
                local_10 = local_10.wrapping_sub(1);
            }
        }
    }
}
