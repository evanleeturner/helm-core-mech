// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c61b4: *mut i8;
    static mut DAT_100a50f0: i32;
    static mut DAT_10095a38: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005e200(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    param_8: i32,
    param_9: i32,
) {
    mech_trace::enter!(
        "1005e200", param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8, param_9
    );
    // SAFETY: the globals are the game's own; DAT_100c61b4 points at a
    // writable buffer with room for the 0x2c-byte record, written and read
    // where the original does.
    unsafe {
        let pc_var1 = mech_trace::gr!("1005e200", "100c61b4", DAT_100c61b4);
        mech_trace::wr!(
            "1005e200",
            pc_var1.cast::<i32>(),
            write_unaligned,
            mech_trace::gr!("1005e200", "10095a38", DAT_10095a38)
        );
        mech_trace::wr!("1005e200", pc_var1.byte_add(4), write, b',' as i8);
        mech_trace::wr!("1005e200", pc_var1.byte_add(5), write, 0);
        mech_trace::wr!("1005e200", pc_var1.byte_add(6), write, 0);
        mech_trace::wr!("1005e200", pc_var1.byte_add(7), write, 0);
        mech_trace::wr!(
            "1005e200",
            pc_var1.byte_add(8).cast::<i32>(),
            write_unaligned,
            param_1
        );
        mech_trace::wr!(
            "1005e200",
            pc_var1.byte_add(0xc).cast::<i32>(),
            write_unaligned,
            param_2
        );
        mech_trace::wr!(
            "1005e200",
            pc_var1.byte_add(0x10).cast::<i32>(),
            write_unaligned,
            param_3
        );
        mech_trace::wr!(
            "1005e200",
            pc_var1.byte_add(0x14).cast::<i32>(),
            write_unaligned,
            param_4.wrapping_shl(0x10)
        );
        mech_trace::wr!(
            "1005e200",
            pc_var1.byte_add(0x18).cast::<i32>(),
            write_unaligned,
            param_5.wrapping_shl(0x10)
        );
        mech_trace::wr!(
            "1005e200",
            pc_var1.byte_add(0x1c).cast::<i32>(),
            write_unaligned,
            param_6.wrapping_shl(0x10)
        );
        mech_trace::wr!(
            "1005e200",
            pc_var1.byte_add(0x20).cast::<i32>(),
            write_unaligned,
            param_7
        );
        mech_trace::wr!(
            "1005e200",
            pc_var1.byte_add(0x24).cast::<i32>(),
            write_unaligned,
            param_8
        );
        mech_trace::wr!(
            "1005e200",
            pc_var1.byte_add(0x28).cast::<i32>(),
            write_unaligned,
            param_9
        );
        let mut i_var2 = mech_trace::rd!(
            "1005e200",
            pc_var1.byte_add(4).cast::<i32>(),
            read_unaligned
        );
        if mech_trace::rd!(
            "1005e200",
            pc_var1.byte_add(4).cast::<i32>(),
            read_unaligned
        ) <= mech_trace::gr!("1005e200", "100a50f0", DAT_100a50f0)
        {
            i_var2 = mech_trace::gr!("1005e200", "100a50f0", DAT_100a50f0);
        }
        mech_trace::gw!("1005e200", "100a50f0", DAT_100a50f0, i_var2);
        let off = mech_trace::rd!(
            "1005e200",
            pc_var1.byte_add(4).cast::<i32>(),
            read_unaligned
        );
        mech_trace::gw!(
            "1005e200",
            "100c61b4",
            DAT_100c61b4,
            mech_trace::gr!("1005e200", "100c61b4", DAT_100c61b4)
                .wrapping_byte_offset(off as isize)
        );
    }
}
