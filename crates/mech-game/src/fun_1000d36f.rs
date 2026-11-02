// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7584: i32;
    static mut DAT_101f64ac: u8;
    static mut DAT_101f64b0: u8;
    static mut DAT_101f64b4: u8;
    static mut DAT_101f64b8: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000d659() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d36f(param_1: i32, param_2: i32, param_3: i16) {
    mech_trace::enter!("1000d36f", param_1, param_2, param_3);
    // SAFETY: the globals and the callee are the game's own; the arrays are
    // indexed by record offsets exactly as the original does.
    unsafe {
        if -1 < param_1 && param_1 < mech_trace::gr!("1000d36f", "100a7584", DAT_100a7584) {
            let i_var1 = param_1.wrapping_mul(0x7c);
            let p = (&raw mut DAT_101f64ac)
                .byte_offset(i_var1 as isize)
                .cast::<u32>();
            mech_trace::wr!(
                "1000d36f",
                p,
                write_unaligned,
                mech_trace::rd!("1000d36f", p, read_unaligned) | 0x400
            );
            let p = (&raw mut DAT_101f64b0)
                .byte_offset(i_var1 as isize)
                .cast::<u32>();
            mech_trace::wr!(
                "1000d36f",
                p,
                write_unaligned,
                mech_trace::rd!("1000d36f", p, read_unaligned) | 0x200
            );
            mech_trace::wr!(
                "1000d36f",
                (&raw mut DAT_101f64b4)
                    .byte_offset(i_var1 as isize)
                    .cast::<i32>(),
                write_unaligned,
                param_2 as i16 as i32
            );
            mech_trace::wr!(
                "1000d36f",
                (&raw mut DAT_101f64b8)
                    .byte_offset(i_var1 as isize)
                    .cast::<i32>(),
                write_unaligned,
                param_3 as i32
            );
            if -1 < param_2 {
                let off = param_2.wrapping_mul(0x7c);
                let p = (&raw mut DAT_101f64ac)
                    .byte_offset(off as isize)
                    .cast::<u32>();
                mech_trace::wr!(
                    "1000d36f",
                    p,
                    write_unaligned,
                    mech_trace::rd!("1000d36f", p, read_unaligned) | 0x800
                );
                mech_trace::cr!("1000d36f", "1000d659", FUN_1000d659());
            }
        }
    }
}
