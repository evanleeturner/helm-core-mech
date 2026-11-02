// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af17c: u32;
    static mut DAT_100f3170: u8;
    static mut DAT_100f3174: u8;
    static mut DAT_10113170: u8;
    static mut DAT_10113172: u8;
    static mut DAT_10113176: u8;
    static mut DAT_10113178: u8;
    static mut DAT_100af0ac: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005f9f7(
    param_1: i32,
    param_2: *mut u16,
    param_3: *mut u16,
) -> *mut u16 {
    mech_trace::enter!("1005f9f7", param_1, param_2, param_3);
    if param_1 >= 0x200 {
        return mech_trace::rt!("1005f9f7", core::ptr::null_mut());
    }
    // SAFETY: the globals and tables are the game's own memory; param_2 and
    // param_3 point at caller-provided u16 slots, accessed as the original does.
    unsafe {
        let i_var1 = param_1.wrapping_mul(0xe);
        let a = mech_trace::rd!(
            "1005f9f7",
            (&raw mut DAT_10113176)
                .byte_offset(i_var1 as isize)
                .cast::<i16>(),
            read_unaligned
        );
        if a == 0 {
            return mech_trace::rt!("1005f9f7", core::ptr::null_mut());
        }
        let b = mech_trace::rd!(
            "1005f9f7",
            (&raw mut DAT_10113178)
                .byte_offset(i_var1 as isize)
                .cast::<i16>(),
            read_unaligned
        );
        if b < 0 {
            return mech_trace::rt!("1005f9f7", core::ptr::null_mut());
        }
        let x = mech_trace::rd!(
            "1005f9f7",
            (&raw mut DAT_10113170)
                .byte_offset(i_var1 as isize)
                .cast::<i16>(),
            read_unaligned
        ) as i32;
        let y = mech_trace::rd!(
            "1005f9f7",
            (&raw mut DAT_10113172)
                .byte_offset(i_var1 as isize)
                .cast::<i16>(),
            read_unaligned
        ) as i32;
        let i_var1 = x.wrapping_mul(0x100).wrapping_add(y.wrapping_mul(8));
        let s = mech_trace::rd!(
            "1005f9f7",
            (&raw mut DAT_100f3170)
                .byte_offset(i_var1 as isize)
                .cast::<i16>(),
            read_unaligned
        );
        if s < 1 {
            return mech_trace::rt!("1005f9f7", core::ptr::null_mut());
        }
        let mut local_c = mech_trace::rd!(
            "1005f9f7",
            (&raw mut DAT_100f3174)
                .byte_offset(i_var1 as isize)
                .cast::<*mut u16>(),
            read_unaligned
        );
        if local_c.is_null() {
            let s2 = mech_trace::rd!(
                "1005f9f7",
                (&raw mut DAT_100f3170)
                    .byte_offset(i_var1 as isize)
                    .cast::<i16>(),
                read_unaligned
            );
            local_c = mech_trace::cr!(
                "1005f9f7",
                "1008668b",
                FUN_1008668b(
                    mech_trace::gr!("1005f9f7", "100af17c", DAT_100af17c) as i32,
                    s2 as i32 as u32,
                    mech_trace::gr!("1005f9f7", "100af0ac", DAT_100af0ac) as *mut i32,
                )
            );
            if local_c.is_null() {
                return mech_trace::rt!("1005f9f7", core::ptr::null_mut());
            }
        }
        mech_trace::wr!(
            "1005f9f7",
            param_2,
            write_unaligned,
            mech_trace::rd!("1005f9f7", local_c, read_unaligned)
        );
        mech_trace::wr!(
            "1005f9f7",
            param_3,
            write_unaligned,
            mech_trace::rd!("1005f9f7", local_c.add(1), read_unaligned)
        );
        mech_trace::rt!("1005f9f7", local_c.add(2))
    }
}
