// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100b06ec: i32;
    static mut DAT_100b06f0: i32;
    static mut DAT_101ec78c: [u8; 0];
    static mut DAT_101ec794: [u8; 0];
    static mut DAT_101ec7a0: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004bb42() -> u32 {
    mech_trace::enter!("1004bb42");
    // SAFETY: the globals and the table they address are the game's own
    // memory; entries are read and written at the original's offsets.
    unsafe {
        let idx = mech_trace::gr!("1004bb42", "100b06ec", DAT_100b06ec);
        let off = idx.wrapping_mul(0x60) as isize;
        mech_trace::rt!(
            "1004bb42",
            if idx < 1
                || mech_trace::rd!(
                    "1004bb42",
                    (&raw mut DAT_101ec78c)
                        .cast::<u8>()
                        .wrapping_byte_offset(off)
                        .cast::<i32>(),
                    read_unaligned
                ) == 0
                || mech_trace::rd!(
                    "1004bb42",
                    (&raw mut DAT_101ec794)
                        .cast::<u8>()
                        .wrapping_byte_offset(off)
                        .cast::<u32>(),
                    read_unaligned
                ) != mech_trace::gr!("1004bb42", "100adf38", DAT_100adf38)
            {
                mech_trace::gw!("1004bb42", "100b06f0", DAT_100b06f0, -1);
                0
            } else {
                mech_trace::gw!(
                    "1004bb42",
                    "100b06f0",
                    DAT_100b06f0,
                    mech_trace::gr!("1004bb42", "100b06ec", DAT_100b06ec)
                );
                let off2 = mech_trace::gr!("1004bb42", "100b06ec", DAT_100b06ec).wrapping_mul(0x60)
                    as isize;
                mech_trace::wr!(
                    "1004bb42",
                    (&raw mut DAT_101ec7a0)
                        .cast::<u8>()
                        .wrapping_byte_offset(off2)
                        .cast::<u32>(),
                    write_unaligned,
                    1
                );
                1
            }
        )
    }
}
