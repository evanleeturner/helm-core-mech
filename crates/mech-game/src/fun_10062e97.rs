// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7ba8: [u8; 0];
    static mut DAT_100ab30c: u32;
    static mut DAT_100adf38: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10062e97(param_1: *mut i32) {
    mech_trace::enter!("10062e97", param_1);
    // SAFETY: param_1 points at the game's object; the addresses computed from
    // its fields and the globals are the game's own memory, accessed as the
    // original does.
    unsafe {
        let mut b_var2 = false;
        let mut local_8: i32 = 0;
        if 0 < mech_trace::rd!("10062e97", param_1.add(0xf7), read_unaligned) {
            while !b_var2 {
                let v = mech_trace::rd!("10062e97", param_1.add(0xf8), read_unaligned);
                mech_trace::wr!(
                    "10062e97",
                    param_1.add(0xf8),
                    write_unaligned,
                    v.wrapping_add(1)
                );
                let v = mech_trace::rd!("10062e97", param_1.add(0xf8), read_unaligned);
                mech_trace::wr!("10062e97", param_1.add(0xf8), write_unaligned, v % 0xe);
                let a = mech_trace::rd!("10062e97", param_1.add(0xf8), read_unaligned)
                    .wrapping_mul(0x40)
                    .wrapping_add(4)
                    .wrapping_add(mech_trace::rd!(
                        "10062e97",
                        param_1.add(0x15),
                        read_unaligned
                    ));
                let i_var1 = mech_trace::rd!("10062e97", (a as *const i32), read_unaligned);
                if -1 < i_var1 {
                    if mech_trace::gr!("10062e97", "100ab30c", DAT_100ab30c) != 0 {
                        let base = mech_trace::rd!("10062e97", param_1, read_unaligned);
                        let w = mech_trace::rd!(
                            "10062e97",
                            (base.wrapping_add(4) as *const u32),
                            read_unaligned
                        );
                        if w == mech_trace::gr!("10062e97", "100adf38", DAT_100adf38) {
                            return;
                        }
                    }
                    let a = mech_trace::rd!("10062e97", param_1.add(0xf8), read_unaligned)
                        .wrapping_mul(0x40)
                        .wrapping_add(mech_trace::rd!(
                            "10062e97",
                            param_1.add(0x15),
                            read_unaligned
                        ));
                    if mech_trace::rd!("10062e97", (a as *const i32), read_unaligned) == -1 {
                        mech_trace::wr!("10062e97", param_1.add(0xf8), write_unaligned, 0);
                    }
                    let a = mech_trace::rd!("10062e97", param_1.add(0xf8), read_unaligned)
                        .wrapping_mul(0x40)
                        .wrapping_add(8)
                        .wrapping_add(mech_trace::rd!(
                            "10062e97",
                            param_1.add(0x15),
                            read_unaligned
                        ));
                    let mut cond = false;
                    if mech_trace::rd!("10062e97", (a as *const i32), read_unaligned) != -1 {
                        let p = (&raw mut DAT_100a7ba8)
                            .cast::<u8>()
                            .byte_offset(i_var1.wrapping_mul(0x68) as isize);
                        if (mech_trace::rd!("10062e97", p, read) & 4) != 0 {
                            cond = true;
                        }
                    }
                    if !cond {
                        local_8 = local_8.wrapping_add(1);
                        if local_8 == 0xe {
                            cond = true;
                        }
                    }
                    if cond {
                        b_var2 = true;
                    }
                }
            }
        }
    }
}
