// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f26f8: u8;
    static mut DAT_101f26fc: u8;
    static mut DAT_101f2700: u8;
    static mut DAT_101f2704: u8;
    static mut DAT_101f2708: u8;
    static mut DAT_101f2710: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10049a38(param_1: i32) {
    mech_trace::enter!("10049a38", param_1);
    let i_var1 = param_1.wrapping_mul(0x2c);
    // SAFETY: the globals are the game's own tables; their addresses are the
    // bases of arrays indexed by the byte offset, read and written in the
    // original's order.
    unsafe {
        let p2700 = (&raw mut DAT_101f2700)
            .byte_add(i_var1 as usize)
            .cast::<u32>();
        let p26fc = (&raw mut DAT_101f26fc)
            .byte_add(i_var1 as usize)
            .cast::<u32>();
        let p26f8 = (&raw mut DAT_101f26f8)
            .byte_add(i_var1 as usize)
            .cast::<u32>();
        let p2704 = (&raw mut DAT_101f2704)
            .byte_add(i_var1 as usize)
            .cast::<u32>();
        let p2708 = (&raw mut DAT_101f2708)
            .byte_add(i_var1 as usize)
            .cast::<u32>();
        let p2710 = (&raw mut DAT_101f2710)
            .byte_add(i_var1 as usize)
            .cast::<u32>();
        mech_trace::wr!("10049a38", p2700, write_unaligned, 0);
        mech_trace::wr!(
            "10049a38",
            p26fc,
            write_unaligned,
            mech_trace::rd!("10049a38", p2700, read_unaligned)
        );
        mech_trace::wr!(
            "10049a38",
            p26f8,
            write_unaligned,
            mech_trace::rd!("10049a38", p26fc, read_unaligned)
        );
        mech_trace::wr!("10049a38", p2704, write_unaligned, 0);
        mech_trace::wr!("10049a38", p2708, write_unaligned, 0);
        mech_trace::wr!("10049a38", p2710, write_unaligned, 0);
    }
}
