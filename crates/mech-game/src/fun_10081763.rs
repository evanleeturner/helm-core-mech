// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b98a4: i32;
    static mut DAT_100c8280: u8;
    static mut DAT_100c8284: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100108a0(param_1: u32, param_2: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10081763(param_1: i32, param_2: i32) -> bool {
    mech_trace::enter!("10081763", param_1, param_2);
    // SAFETY: the globals and the callee are the game's own; the table at
    // DAT_100c8280 is accessed at the same 0x14-byte strides as the original.
    unsafe {
        let mut local_c: i32 = 0;
        while local_c < mech_trace::gr!("10081763", "100b98a4", DAT_100b98a4)
            && mech_trace::rd!(
                "10081763",
                (&raw mut DAT_100c8280)
                    .byte_add(local_c.wrapping_mul(0x14) as usize)
                    .cast::<i32>(),
                read_unaligned
            ) != param_2
        {
            local_c = local_c.wrapping_add(1);
        }
        let i_var1 = local_c.wrapping_mul(0x14) as usize;
        if local_c == mech_trace::gr!("10081763", "100b98a4", DAT_100b98a4) {
            mech_trace::wr!(
                "10081763",
                (&raw mut DAT_100c8280).byte_add(i_var1).cast::<i32>(),
                write_unaligned,
                param_2
            );
            mech_trace::wr!(
                "10081763",
                (&raw mut DAT_100c8284).byte_add(i_var1).cast::<u32>(),
                write_unaligned,
                0
            );
            mech_trace::gw!(
                "10081763",
                "100b98a4",
                DAT_100b98a4,
                mech_trace::gr!("10081763", "100b98a4", DAT_100b98a4).wrapping_add(1)
            );
            if 0x20 < mech_trace::gr!("10081763", "100b98a4", DAT_100b98a4) {
                mech_trace::cr!(
                    "10081763",
                    "100108a0",
                    FUN_100108a0(0x44, core::ptr::null_mut())
                );
            }
        }
        let p = (&raw mut DAT_100c8284).byte_add(i_var1).cast::<i32>();
        mech_trace::wr!(
            "10081763",
            p,
            write_unaligned,
            mech_trace::rd!("10081763", p, read_unaligned).wrapping_add(param_1)
        );
        mech_trace::rt!(
            "10081763",
            mech_trace::rd!("10081763", p, read_unaligned) < 0xffdd
        )
    }
}
