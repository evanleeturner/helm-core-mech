// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: i32;
    static mut DAT_1016aca0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_1002f3a7(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_1002fbc0(param_1: i32);
    fn __ftol() -> i64;
    fn FUN_1002f727(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002435e(param_1: i32) -> bool {
    mech_trace::enter!("1002435e", param_1);
    let base = param_1 as *mut u8;
    // SAFETY: param_1 points at the game's object; fields are read and written
    // at the original's offsets, and the callees and globals are the game's own.
    unsafe {
        let i_var1 =
            mech_trace::gr!("1002435e", "100b38fc", DAT_100b38fc).wrapping_sub(mech_trace::rd!(
                "1002435e",
                base.byte_add(0x1de).cast::<i32>(),
                read_unaligned
            ));
        if i_var1 < 0x5b {
            let i_var4 = mech_trace::rd!(
                "1002435e",
                base.byte_add(0x1f2).cast::<i32>(),
                read_unaligned
            );
            let i_var2 = mech_trace::cr!("1002435e", "10051f43", FUN_10051f43(10));
            FUN_1002f3a7(
                mech_trace::rd!(
                    "1002435e",
                    base.byte_add(0x1d2).cast::<i32>(),
                    read_unaligned
                ),
                mech_trace::rd!(
                    "1002435e",
                    base.byte_add(0x1ea).cast::<i32>(),
                    read_unaligned
                ),
                mech_trace::rd!(
                    "1002435e",
                    base.byte_add(0x1ee).cast::<i32>(),
                    read_unaligned
                )
                .wrapping_add(i_var2),
                i_var4,
            );
            FUN_1002fbc0(mech_trace::rd!(
                "1002435e",
                base.byte_add(0x1d2).cast::<i32>(),
                read_unaligned
            ));
            let l_var3 = mech_trace::cr!("1002435e", "__ftol", __ftol());
            let p = (&raw mut DAT_1016aca0).cast::<u8>();
            let v = mech_trace::rd!("1002435e", p.offset((l_var3 as i32) as isize), read);
            mech_trace::wr!(
                "1002435e",
                base.byte_add(0x1e6).cast::<u32>(),
                write_unaligned,
                u32::from(v)
            );
        } else {
            let p = base.byte_add(0x10).cast::<u16>();
            mech_trace::wr!(
                "1002435e",
                p,
                write_unaligned,
                mech_trace::rd!("1002435e", p, read_unaligned) & 0xfffe
            );
            FUN_1002f727(mech_trace::rd!(
                "1002435e",
                base.byte_add(0x1d2).cast::<i32>(),
                read_unaligned
            ));
        }
        mech_trace::rt!("1002435e", i_var1 < 0x5b)
    }
}
