// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100cded0: u8;
    static mut DAT_100cded4: u8;
    static mut DAT_100cded8: u8;
    static mut DAT_100cdedc: u8;
    static mut DAT_100cdee0: u8;
    static mut DAT_100cdee4: u8;
    static mut DAT_100cdee8: u8;
    static mut DAT_100cdeec: u8;
    static mut DAT_100cdef0: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007c97d(param_1: i32) {
    mech_trace::enter!("1007c97d", param_1);
    let i_var1 = param_1.wrapping_mul(0x24) as isize;
    // SAFETY: the globals are the game's own memory; each is the base of a
    // table whose entry at the 0x24-byte stride is read and written where
    // the original does.
    unsafe {
        mech_trace::wr!(
            "1007c97d",
            (&raw mut DAT_100cded0)
                .wrapping_byte_offset(i_var1)
                .cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "1007c97d",
            (&raw mut DAT_100cded4)
                .wrapping_byte_offset(i_var1)
                .cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "1007c97d",
            (&raw mut DAT_100cdee0)
                .wrapping_byte_offset(i_var1)
                .cast::<u32>(),
            write_unaligned,
            0
        );
        let v = mech_trace::rd!(
            "1007c97d",
            (&raw mut DAT_100cdee0)
                .wrapping_byte_offset(i_var1)
                .cast::<u32>(),
            read_unaligned
        );
        mech_trace::wr!(
            "1007c97d",
            (&raw mut DAT_100cdedc)
                .wrapping_byte_offset(i_var1)
                .cast::<u32>(),
            write_unaligned,
            v
        );
        let v = mech_trace::rd!(
            "1007c97d",
            (&raw mut DAT_100cdedc)
                .wrapping_byte_offset(i_var1)
                .cast::<u32>(),
            read_unaligned
        );
        mech_trace::wr!(
            "1007c97d",
            (&raw mut DAT_100cded8)
                .wrapping_byte_offset(i_var1)
                .cast::<u32>(),
            write_unaligned,
            v
        );
        mech_trace::wr!(
            "1007c97d",
            (&raw mut DAT_100cdeec)
                .wrapping_byte_offset(i_var1)
                .cast::<u32>(),
            write_unaligned,
            0
        );
        let v = mech_trace::rd!(
            "1007c97d",
            (&raw mut DAT_100cdeec)
                .wrapping_byte_offset(i_var1)
                .cast::<u32>(),
            read_unaligned
        );
        mech_trace::wr!(
            "1007c97d",
            (&raw mut DAT_100cdee8)
                .wrapping_byte_offset(i_var1)
                .cast::<u32>(),
            write_unaligned,
            v
        );
        let v = mech_trace::rd!(
            "1007c97d",
            (&raw mut DAT_100cdee8)
                .wrapping_byte_offset(i_var1)
                .cast::<u32>(),
            read_unaligned
        );
        mech_trace::wr!(
            "1007c97d",
            (&raw mut DAT_100cdee4)
                .wrapping_byte_offset(i_var1)
                .cast::<u32>(),
            write_unaligned,
            v
        );
        mech_trace::wr!(
            "1007c97d",
            (&raw mut DAT_100cdef0)
                .wrapping_byte_offset(i_var1)
                .cast::<u32>(),
            write_unaligned,
            0
        );
    }
}
