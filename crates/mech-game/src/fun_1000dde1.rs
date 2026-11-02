// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7584: i32;
    static mut DAT_101f64bc: [i8; 1];
    static mut DAT_101f64c0: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000dde1(
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut u32,
    param_4: *mut u32,
) {
    mech_trace::enter!("1000dde1", param_1, param_2, param_3, param_4);
    // SAFETY: the globals and callee are the game's own; the table entries and
    // the pointers passed in are valid for the reads and writes the original makes.
    unsafe {
        if mech_trace::gr!("1000dde1", "100a7584", DAT_100a7584) < param_1 || param_1 < 0 {
            mech_trace::wr!("1000dde1", param_4, write_unaligned, 0);
            mech_trace::wr!(
                "1000dde1",
                param_3,
                write_unaligned,
                mech_trace::rd!("1000dde1", param_4, read_unaligned)
            );
            mech_trace::wr!(
                "1000dde1",
                param_2,
                write_unaligned,
                mech_trace::rd!("1000dde1", param_3, read_unaligned)
            );
        } else {
            let off = (param_1 as u32).wrapping_mul(0x7c) as usize;
            let i_var1 = mech_trace::rd!(
                "1000dde1",
                (&raw mut DAT_101f64bc)
                    .cast::<u8>()
                    .byte_add(off)
                    .cast::<i32>(),
                read_unaligned
            );
            if i_var1 == 0 {
                let v = mech_trace::rd!(
                    "1000dde1",
                    (&raw mut DAT_101f64c0)
                        .cast::<u8>()
                        .byte_add(off)
                        .cast::<i32>(),
                    read_unaligned
                );
                if v == 0 {
                    mech_trace::wr!("1000dde1", param_4, write_unaligned, 0);
                    mech_trace::wr!(
                        "1000dde1",
                        param_3,
                        write_unaligned,
                        mech_trace::rd!("1000dde1", param_4, read_unaligned)
                    );
                    mech_trace::wr!(
                        "1000dde1",
                        param_2,
                        write_unaligned,
                        mech_trace::rd!("1000dde1", param_3, read_unaligned)
                    );
                } else {
                    let v2 = mech_trace::rd!(
                        "1000dde1",
                        (&raw mut DAT_101f64c0)
                            .cast::<u8>()
                            .byte_add(off)
                            .cast::<i32>(),
                        read_unaligned
                    );
                    FUN_1002f322(v2, param_2, param_3, param_4);
                }
            } else {
                let p = i_var1 as usize as *const u8;
                mech_trace::wr!(
                    "1000dde1",
                    param_2,
                    write_unaligned,
                    mech_trace::rd!("1000dde1", p.byte_add(0x34).cast::<u32>(), read_unaligned)
                );
                mech_trace::wr!(
                    "1000dde1",
                    param_3,
                    write_unaligned,
                    mech_trace::rd!("1000dde1", p.byte_add(0x38).cast::<u32>(), read_unaligned)
                );
                mech_trace::wr!(
                    "1000dde1",
                    param_4,
                    write_unaligned,
                    mech_trace::rd!("1000dde1", p.byte_add(0x3c).cast::<u32>(), read_unaligned)
                );
            }
        }
    }
}
