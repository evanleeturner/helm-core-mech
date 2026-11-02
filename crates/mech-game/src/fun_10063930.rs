// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063528(param_1: i32, param_2: i32, param_3: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063930(param_1: i32) {
    mech_trace::enter!("10063930", param_1);
    // SAFETY: param_1 is the address of the game's own object; the offsets
    // read and written are the fields the original touches, and the callee
    // is the game's own function.
    unsafe {
        let p_flags = (param_1.wrapping_add(0x440)) as *mut u16;
        mech_trace::wr!(
            "10063930",
            p_flags,
            write_unaligned,
            mech_trace::rd!("10063930", p_flags, read_unaligned) ^ 0x4000
        );
        let idx = mech_trace::rd!(
            "10063930",
            ((param_1.wrapping_add(0x3e0)) as *const i32),
            read_unaligned
        );
        let mut local_c: i32 = idx.wrapping_add(1);
        let base = mech_trace::rd!(
            "10063930",
            ((param_1.wrapping_add(0x54)) as *const i32),
            read_unaligned
        );
        let v = mech_trace::rd!(
            "10063930",
            (base.wrapping_add(local_c.wrapping_mul(0x40)) as *const i32),
            read_unaligned
        );
        if v == -1 {
            local_c = 0;
        }
        let base2 = mech_trace::rd!(
            "10063930",
            ((param_1.wrapping_add(0x54)) as *const i32),
            read_unaligned
        );
        let idx2 = mech_trace::rd!(
            "10063930",
            ((param_1.wrapping_add(0x3e0)) as *const i32),
            read_unaligned
        );
        let arg = mech_trace::rd!(
            "10063930",
            (base2
                .wrapping_add(0xc)
                .wrapping_add(idx2.wrapping_mul(0x40)) as *const i32),
            read_unaligned
        );
        mech_trace::cr!("10063930", "10063528", FUN_10063528(param_1, local_c, arg));
    }
}
