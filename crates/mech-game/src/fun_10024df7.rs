// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f3a7(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
    fn FUN_1002f727(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10024df7(param_1: i32) -> bool {
    mech_trace::enter!("10024df7", param_1);
    // SAFETY: param_1 is the caller's object address; the globals and callees
    // are the game's own, accessed in the original's order.
    unsafe {
        let base = param_1 as usize as *mut u8;
        let i_var1 = (mech_trace::gr!("10024df7", "100b38fc", DAT_100b38fc) as i32).wrapping_sub(
            mech_trace::rd!(
                "10024df7",
                base.byte_add(0x1de).cast::<i32>(),
                read_unaligned
            ),
        );
        if i_var1 < 0x2e {
            let a = mech_trace::rd!(
                "10024df7",
                base.byte_add(0x1d2).cast::<i32>(),
                read_unaligned
            );
            let b = mech_trace::rd!(
                "10024df7",
                base.byte_add(0x1ea).cast::<i32>(),
                read_unaligned
            );
            let c = mech_trace::rd!(
                "10024df7",
                base.byte_add(0x1ee).cast::<i32>(),
                read_unaligned
            );
            let d = mech_trace::rd!(
                "10024df7",
                base.byte_add(0x1f2).cast::<i32>(),
                read_unaligned
            );
            mech_trace::cr!("10024df7", "1002f3a7", FUN_1002f3a7(a, b, c, d));
            let p = mech_trace::rd!(
                "10024df7",
                base.byte_add(0x1d2).cast::<i32>(),
                read_unaligned
            );
            mech_trace::cr!("10024df7", "1002fbc0", FUN_1002fbc0(p as usize as *mut i32));
            mech_trace::wr!(
                "10024df7",
                base.byte_add(0x1e6).cast::<u32>(),
                write_unaligned,
                3
            );
        } else {
            let p = base.byte_add(0x10).cast::<u16>();
            mech_trace::wr!(
                "10024df7",
                p,
                write_unaligned,
                mech_trace::rd!("10024df7", p, read_unaligned) & 0xfffe
            );
            let q = mech_trace::rd!(
                "10024df7",
                base.byte_add(0x1d2).cast::<i32>(),
                read_unaligned
            );
            mech_trace::cr!("10024df7", "1002f727", FUN_1002f727(q));
        }
        mech_trace::rt!("10024df7", i_var1 < 0x2e)
    }
}
