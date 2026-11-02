// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093900: u8;
    static mut DAT_10093a18: u8;
    static mut DAT_10093b2c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100361e0(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) {
    mech_trace::enter!("100361e0", param_1, param_2, param_3, param_4, param_5);
    let mut param_3 = param_3;
    let mut param_4 = param_4;
    // SAFETY: the globals are the game's own memory; the table pointer stored
    // in DAT_10093b2c points at a game table whose fields are written where
    // the original does.
    unsafe {
        if -1 < param_1 {
            mech_trace::gw!(
                "100361e0",
                "10093b2c",
                DAT_10093b2c,
                (&raw mut DAT_10093900) as u32
            );
            if param_1 != 0 {
                mech_trace::gw!(
                    "100361e0",
                    "10093b2c",
                    DAT_10093b2c,
                    (&raw mut DAT_10093a18) as u32
                );
            }
        }
        if -1 < param_2 {
            mech_trace::wr!(
                "100361e0",
                (mech_trace::gr!("100361e0", "10093b2c", DAT_10093b2c) as *mut i32),
                write_unaligned,
                param_2
            );
        }
        if -1 < param_3 {
            if 2 < param_3 {
                param_3 = 3;
            }
            mech_trace::wr!(
                "100361e0",
                (mech_trace::gr!("100361e0", "10093b2c", DAT_10093b2c) as *mut u8)
                    .byte_add(8)
                    .cast::<i32>(),
                write_unaligned,
                param_3
            );
        }
        if -1 < param_4 {
            if 2 < param_4 {
                param_4 = 3;
            }
            mech_trace::wr!(
                "100361e0",
                (mech_trace::gr!("100361e0", "10093b2c", DAT_10093b2c) as *mut u8)
                    .byte_add(0xc)
                    .cast::<i32>(),
                write_unaligned,
                param_4
            );
        }
        if -1 < param_5 {
            mech_trace::wr!(
                "100361e0",
                (mech_trace::gr!("100361e0", "10093b2c", DAT_10093b2c) as *mut u8)
                    .byte_add(0x10)
                    .cast::<i32>(),
                write_unaligned,
                param_5
            );
        }
    }
}
