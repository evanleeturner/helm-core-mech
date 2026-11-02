// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40f8: i32;
    static mut DAT_100a40e4: i32;
    static mut DAT_100b38fc: i32;
    static mut DAT_100b3904: i32;
    static mut DAT_101e8db0: [u8; 0];
    static mut DAT_101e8db4: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002d8f(
        param_1: u32,
        param_2: *mut c_void,
        param_3: u16,
        param_4: i32,
        param_5: i32,
    ) -> u32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_set_sample_type_12(param_1: u32, param_2: u32, param_3: u32);
    fn _AIL_set_sample_playback_rate_8(param_1: u32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10002ba7(
    param_1: i32,
    param_2: u32,
    param_3: u32,
    param_4: *mut c_void,
    param_5: i32,
    param_6: i32,
    param_7: u32,
    param_8: i32,
    param_9: i32,
    param_10: u16,
) -> u32 {
    mech_trace::enter!(
        "10002ba7", param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8,
        param_9, param_10
    );
    let mut param_10 = param_10;
    let mut param_8 = param_8;
    // SAFETY: globals, tables and callees are the game's own memory; offsets
    // mirror the original's pointer arithmetic.
    unsafe {
        let u_var3: u32;
        if mech_trace::gr!("10002ba7", "100a40f8", DAT_100a40f8) == 0 {
            u_var3 = 0xfffffffe;
        } else if param_5 == 0 {
            u_var3 = 0xfffffff5;
        } else {
            let t0 = (&raw mut DAT_101e8db0)
                .cast::<u8>()
                .byte_add((param_3 as usize).wrapping_mul(8));
            let t4 = (&raw mut DAT_101e8db4)
                .cast::<u8>()
                .byte_add((param_3 as usize).wrapping_mul(8));
            if (param_3 as i32) < 1 {
                if param_4.is_null() {
                    return mech_trace::rt!("10002ba7", 0xfffffffa);
                }
                param_10 = (param_10 & 0xff00) | 0x32;
            } else if mech_trace::rd!("10002ba7", t0.cast::<i16>(), read_unaligned) == -1 {
                param_10 = (param_10 & 0xff00) | 0x32;
            } else {
                param_10 = (mech_trace::rd!("10002ba7", t0.cast::<i16>(), read_unaligned) as u16)
                    | (param_10 & 0xff00);
            }
            u_var3 = mech_trace::cr!(
                "10002ba7",
                "10002d8f",
                FUN_10002d8f(param_3, param_4, param_10, -1, param_9)
            );
            if (u_var3 as i32) > -1 {
                let at = |off: u32| -> usize {
                    (mech_trace::gr!("10002ba7", "100a40f8", DAT_100a40f8) as u32)
                        .wrapping_add(off)
                        .wrapping_add(u_var3.wrapping_mul(4)) as usize
                };
                let u_var2 = mech_trace::rd!("10002ba7", (at(4) as *const u32), read_unaligned);
                _AIL_set_sample_type_12(u_var2, 0, 0);
                if (param_3 as i32) < 1
                    || mech_trace::rd!("10002ba7", t4.cast::<i16>(), read_unaligned) == -1
                {
                    param_8 = mech_trace::gr!("10002ba7", "100a40e4", DAT_100a40e4);
                } else {
                    param_8 = mech_trace::rd!("10002ba7", t4.cast::<i16>(), read_unaligned) as i32;
                }
                _AIL_set_sample_playback_rate_8(u_var2, param_8);
                mech_trace::wr!(
                    "10002ba7",
                    (at(0x3a8) as *mut i32),
                    write_unaligned,
                    mech_trace::gr!("10002ba7", "100b38fc", DAT_100b38fc)
                );
                if mech_trace::gr!("10002ba7", "100b3904", DAT_100b3904) < param_1 {
                    let p = at(0x3a8) as *mut i32;
                    mech_trace::wr!(
                        "10002ba7",
                        p,
                        write_unaligned,
                        mech_trace::rd!("10002ba7", p, read_unaligned).wrapping_add(param_1)
                    );
                }
                mech_trace::wr!(
                    "10002ba7",
                    (at(0x428) as *mut u32),
                    write_unaligned,
                    param_7
                );
                mech_trace::wr!("10002ba7", (at(1000) as *mut u32), write_unaligned, param_2);
                let i_var4 =
                    mech_trace::cr!("10002ba7", "10003d60", FUN_10003d60(param_5, param_6));
                mech_trace::wr!("10002ba7", (at(0x468) as *mut i32), write_unaligned, i_var4);
                mech_trace::wr!("10002ba7", (at(0x48) as *mut u32), write_unaligned, 1);
            }
        }
        mech_trace::rt!("10002ba7", u_var3)
    }
}
