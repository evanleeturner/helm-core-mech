// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c61b4: *mut i8;
    static mut DAT_100a50f0: i32;
    static mut DAT_10095b50: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005e870(param_1: *mut i8, param_2: *mut i8, param_3: i32) {
    mech_trace::enter!("1005e870", param_1, param_2, param_3);
    // SAFETY: the globals are the game's own; param_1 and param_2 point at
    // NUL-terminated strings and the destination record is the game's buffer,
    // read and written where the original does.
    unsafe {
        let pc2: *mut i8 = mech_trace::gr!("1005e870", "100c61b4", DAT_100c61b4);
        mech_trace::wr!(
            "1005e870",
            pc2.cast::<u32>(),
            write_unaligned,
            mech_trace::gr!("1005e870", "10095b50", DAT_10095b50)
        );
        mech_trace::wr!(
            "1005e870",
            pc2.byte_add(4).cast::<u32>(),
            write_unaligned,
            (param_3 as u32).wrapping_add(0xa3) & 0xffff_fffc
        );

        // first string -> offset 0x14
        let mut len: u32 = 0;
        while mech_trace::dr!("1005e870", param_1.byte_add(len as usize)) != 0 {
            len = len.wrapping_add(1);
        }
        len = len.wrapping_add(1);
        let mut src = param_1 as *const i8;
        let mut dst = pc2.byte_add(0x14);
        let mut n = len >> 2;
        while n != 0 {
            mech_trace::wr!(
                "1005e870",
                dst.cast::<u32>(),
                write_unaligned,
                mech_trace::rd!("1005e870", src.cast::<u32>(), read_unaligned)
            );
            src = src.byte_add(4);
            dst = dst.byte_add(4);
            n -= 1;
        }
        let mut r = len & 3;
        while r != 0 {
            mech_trace::dw!("1005e870", dst, mech_trace::dr!("1005e870", src));
            src = src.byte_add(1);
            dst = dst.byte_add(1);
            r -= 1;
        }

        // second string -> offset 0x90
        let mut len: u32 = 0;
        while mech_trace::dr!("1005e870", param_2.byte_add(len as usize)) != 0 {
            len = len.wrapping_add(1);
        }
        len = len.wrapping_add(1);
        let mut src = param_2 as *const i8;
        let mut dst = pc2.byte_add(0x90);
        let mut n = len >> 2;
        while n != 0 {
            mech_trace::wr!(
                "1005e870",
                dst.cast::<u32>(),
                write_unaligned,
                mech_trace::rd!("1005e870", src.cast::<u32>(), read_unaligned)
            );
            src = src.byte_add(4);
            dst = dst.byte_add(4);
            n -= 1;
        }
        let mut r = len & 3;
        while r != 0 {
            mech_trace::dw!("1005e870", dst, mech_trace::dr!("1005e870", src));
            src = src.byte_add(1);
            dst = dst.byte_add(1);
            r -= 1;
        }

        let mut i_var3 = mech_trace::rd!("1005e870", pc2.byte_add(4).cast::<i32>(), read_unaligned);
        if i_var3 <= mech_trace::gr!("1005e870", "100a50f0", DAT_100a50f0) {
            i_var3 = mech_trace::gr!("1005e870", "100a50f0", DAT_100a50f0);
        }
        mech_trace::gw!("1005e870", "100a50f0", DAT_100a50f0, i_var3);
        mech_trace::gw!(
            "1005e870",
            "100c61b4",
            DAT_100c61b4,
            mech_trace::gr!("1005e870", "100c61b4", DAT_100c61b4).wrapping_byte_add(0xa0)
        );
    }
}
