// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10045f36(param_1: i32, param_2: i32) {
    mech_trace::enter!("10045f36", param_1, param_2);
    // SAFETY: param_1 and param_2 are addresses of the caller's structures;
    // fields are read and written at the same offsets as the original.
    unsafe {
        mech_trace::wr!(
            "10045f36",
            ((param_1.wrapping_add(0xa4)) as *mut i32),
            write_unaligned,
            param_2
        );
        mech_trace::wr!(
            "10045f36",
            ((param_1.wrapping_add(0xb4)) as *mut i16),
            write_unaligned,
            mech_trace::rd!(
                "10045f36",
                ((param_2.wrapping_add(4)) as *const u32),
                read_unaligned
            ) as i16
        );
        mech_trace::wr!(
            "10045f36",
            ((param_1.wrapping_add(0xb6)) as *mut i16),
            write_unaligned,
            mech_trace::rd!(
                "10045f36",
                ((param_2.wrapping_add(8)) as *const u32),
                read_unaligned
            ) as i16
        );
        let a = mech_trace::rd!(
            "10045f36",
            ((param_2.wrapping_add(0xc)) as *const u32),
            read_unaligned
        ) as u16;
        let b = mech_trace::rd!(
            "10045f36",
            ((param_2.wrapping_add(4)) as *const u32),
            read_unaligned
        ) as u16;
        mech_trace::wr!(
            "10045f36",
            ((param_1.wrapping_add(0xb8)) as *mut i16),
            write_unaligned,
            a.wrapping_sub(b).wrapping_add(1) as i16
        );
        let c = mech_trace::rd!(
            "10045f36",
            ((param_2.wrapping_add(0x10)) as *const u32),
            read_unaligned
        ) as u16;
        let d = mech_trace::rd!(
            "10045f36",
            ((param_2.wrapping_add(8)) as *const u32),
            read_unaligned
        ) as u16;
        mech_trace::wr!(
            "10045f36",
            ((param_1.wrapping_add(0xba)) as *mut i16),
            write_unaligned,
            c.wrapping_sub(d).wrapping_add(1) as i16
        );
    }
}
