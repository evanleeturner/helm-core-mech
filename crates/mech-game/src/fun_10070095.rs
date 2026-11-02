// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b4068: u8;
    static mut DAT_100b4250: u8;
    static mut DAT_100b47cc: u8;
    static mut DAT_100b47c0: u8;
    static mut DAT_100b47d8: u8;
}

unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10070095(param_1: u32, param_2: *mut i8) -> u32 {
    mech_trace::enter!("10070095", param_1, param_2);
    let _ = param_1;
    // SAFETY: param_2 points at the caller's record, written at the original's
    // offsets; the globals and sprintf are the game's own.
    unsafe {
        mech_trace::wr!(
            "10070095",
            param_2.byte_add(0x58).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10070095",
            param_2.byte_add(0x5c).cast::<u32>(),
            write_unaligned,
            0x79
        );
        mech_trace::cr!(
            "10070095",
            "sprintf",
            sprintf(param_2, (&raw mut DAT_100b47c0).cast::<i8>() as *const i8)
        );
        mech_trace::cr!(
            "10070095",
            "sprintf",
            sprintf(
                param_2.byte_add(0xc),
                (&raw mut DAT_100b47cc).cast::<i8>() as *const i8,
            )
        );
        mech_trace::cr!(
            "10070095",
            "sprintf",
            sprintf(
                param_2.byte_add(0x4c),
                (&raw mut DAT_100b47d8).cast::<i8>() as *const i8,
            )
        );
        mech_trace::wr!(
            "10070095",
            param_2.byte_add(0x60).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10070095",
            param_2.byte_add(100).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10070095",
            param_2.byte_add(0x68).cast::<u32>(),
            write_unaligned,
            (&raw mut DAT_100b4250) as u32
        );
        mech_trace::wr!(
            "10070095",
            param_2.byte_add(0x6c).cast::<u32>(),
            write_unaligned,
            (&raw mut DAT_100b4068) as u32
        );
        mech_trace::wr!(
            "10070095",
            param_2.byte_add(0x70).cast::<u32>(),
            write_unaligned,
            0
        );
    }
    mech_trace::rt!("10070095", 0)
}
