// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
    static mut DAT_10123120: u32;
    static mut DAT_10123240: u8;
    static mut DAT_10123244: u8;
    static mut DAT_10123248: u8;
    static mut DAT_10123278: u8;
    static mut DAT_101232ee: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004fd60(param_1: i32) {
    mech_trace::enter!("1004fd60", param_1);
    // SAFETY: all globals are the game's own memory; the offsets are
    // computed exactly as the original does.
    unsafe {
        mech_trace::gw!(
            "1004fd60",
            "10123120",
            DAT_10123120,
            mech_trace::gr!("1004fd60", "100b38fc", DAT_100b38fc) / 0xb5
        );
        let i_var1 = param_1.wrapping_mul(0x3c0a);
        let pi_var2 = (&raw mut DAT_10123240).byte_offset(i_var1 as isize);
        let mut local_c: i32 = 0;
        while local_c < mech_trace::rd!("1004fd60", pi_var2.cast::<i32>(), read_unaligned) {
            let elem = pi_var2.byte_offset(local_c.wrapping_mul(0x13f) as isize);
            let pc_var3 = elem.byte_add(0x3a);
            if mech_trace::rd!(
                "1004fd60",
                elem.byte_add(0x3c).cast::<i32>(),
                read_unaligned
            ) == 0x40000
                && (mech_trace::rd!("1004fd60", pc_var3.cast::<i8>(), read)) == 5
            {
                let a = mech_trace::rd!(
                    "1004fd60",
                    elem.byte_add(0xe3).cast::<i16>(),
                    read_unaligned
                ) as i32;
                let b = mech_trace::rd!(
                    "1004fd60",
                    elem.byte_add(0xe5).cast::<i16>(),
                    read_unaligned
                ) as i32;
                let idx = a.wrapping_mul(0x3c0a).wrapping_add(b.wrapping_mul(0x13f));
                let p = (&raw mut DAT_101232ee).byte_offset(idx as isize);
                if mech_trace::rd!("1004fd60", p, read) == 0 {
                    mech_trace::wr!("1004fd60", p, write, 1);
                } else {
                    mech_trace::wr!("1004fd60", p, write, 0);
                }
            }
            mech_trace::wr!("1004fd60", pc_var3.cast::<i8>(), write, 3);
            mech_trace::wr!(
                "1004fd60",
                elem.byte_add(0xa1).cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            mech_trace::wr!(
                "1004fd60",
                elem.byte_add(0xa5).cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            local_c = local_c.wrapping_add(1);
        }
        mech_trace::wr!(
            "1004fd60",
            (&raw mut DAT_10123278).byte_offset(i_var1 as isize),
            write,
            0
        );
        mech_trace::wr!(
            "1004fd60",
            (&raw mut DAT_10123244)
                .byte_offset(i_var1 as isize)
                .cast::<u32>(),
            write_unaligned,
            mech_trace::gr!("1004fd60", "10123120", DAT_10123120)
        );
        mech_trace::wr!(
            "1004fd60",
            (&raw mut DAT_10123248)
                .byte_offset(i_var1 as isize)
                .cast::<u32>(),
            write_unaligned,
            0xffffffff
        );
    }
}
