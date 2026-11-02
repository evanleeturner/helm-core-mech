// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7bc8: u8;
    static mut DAT_100a7bcc: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10062d5f(param_1: i32, param_2: i32) -> u8;
    fn FUN_10044cc4(param_1: i32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100636e5(param_1: *mut i32) {
    mech_trace::enter!("100636e5", param_1);
    // SAFETY: param_1 points at the game's object; the offsets, globals and
    // callees are the game's own memory, accessed in the original's order.
    unsafe {
        let i_var2 = mech_trace::rd!("100636e5", param_1.add(0xf8), read_unaligned);
        mech_trace::cr!("100636e5", "10062d5f", FUN_10062d5f(param_1 as i32, 0));
        let i_var3 = mech_trace::rd!("100636e5", param_1.add(0xf8), read_unaligned);
        loop {
            let i_var4 = mech_trace::rd!("100636e5", param_1.add(0xf8), read_unaligned)
                .wrapping_mul(0x40)
                .wrapping_add(mech_trace::rd!(
                    "100636e5",
                    param_1.add(0x15),
                    read_unaligned
                ));
            let p = i_var4 as usize as *mut u8;
            let i_var1 = mech_trace::rd!("100636e5", p.byte_add(4).cast::<i32>(), read_unaligned)
                .wrapping_mul(0x68);
            if mech_trace::rd!("100636e5", p.byte_add(8).cast::<i32>(), read_unaligned) == 1 {
                mech_trace::wr!("100636e5", p.byte_add(8).cast::<i32>(), write_unaligned, 2);
                let a = (&raw mut DAT_100a7bcc).byte_offset(i_var1 as isize);
                mech_trace::wr!(
                    "100636e5",
                    p.byte_add(0x24).cast::<u32>(),
                    write_unaligned,
                    mech_trace::rd!("100636e5", a.cast::<u32>(), read_unaligned)
                );
                mech_trace::wr!(
                    "100636e5",
                    p.byte_add(0x10).cast::<u32>(),
                    write_unaligned,
                    0
                );
                mech_trace::cr!("100636e5", "10044cc4", FUN_10044cc4(i_var4, param_1));
                let b = (&raw mut DAT_100a7bc8).byte_offset(i_var1 as isize);
                if mech_trace::rd!("100636e5", b.cast::<i32>(), read_unaligned) == 0
                    || (mech_trace::rd!(
                        "100636e5",
                        param_1.add(0x110).cast::<u16>(),
                        read_unaligned
                    ) & 0x80)
                        == 0
                {
                    mech_trace::wr!(
                        "100636e5",
                        p.byte_add(0x1c).cast::<u32>(),
                        write_unaligned,
                        0
                    );
                    let v =
                        mech_trace::rd!("100636e5", p.byte_add(0x1c).cast::<u32>(), read_unaligned);
                    mech_trace::wr!(
                        "100636e5",
                        p.byte_add(0x18).cast::<u32>(),
                        write_unaligned,
                        v
                    );
                } else {
                    let base =
                        mech_trace::rd!("100636e5", param_1, read_unaligned) as usize as *const u8;
                    let v = mech_trace::rd!(
                        "100636e5",
                        base.byte_add(0xdc).cast::<u32>(),
                        read_unaligned
                    );
                    mech_trace::wr!(
                        "100636e5",
                        p.byte_add(0x18).cast::<u32>(),
                        write_unaligned,
                        v & 0xff
                    );
                    let v = mech_trace::rd!(
                        "100636e5",
                        base.byte_add(0xdc).cast::<u32>(),
                        read_unaligned
                    );
                    mech_trace::wr!(
                        "100636e5",
                        p.byte_add(0x1c).cast::<u32>(),
                        write_unaligned,
                        v & 0xf00
                    );
                }
            }
            mech_trace::cr!("100636e5", "10062d5f", FUN_10062d5f(param_1 as i32, 0));
            if mech_trace::rd!("100636e5", param_1.add(0xf8), read_unaligned) == i_var3 {
                break;
            }
        }
        mech_trace::wr!("100636e5", param_1.add(0xf8), write_unaligned, i_var2);
    }
}
