// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a365c: i32;
    static mut DAT_100a3660: i32;
    static mut DAT_100a3664: u32;
    static mut DAT_100a3668: u8;
    static mut DAT_100a3674: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100578d0() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100575d0(param_1: i32) -> i32 {
    mech_trace::enter!("100575d0", param_1);
    // SAFETY: the globals and handler table are the game's own memory; the
    // handler records and entry array are read and written where the original does.
    unsafe {
        if mech_trace::gr!("100575d0", "100a3664", DAT_100a3664) == 0 {
            if param_1 == 0 {
                // fall through to the loop without clearing
            } else {
                mech_trace::gw!("100575d0", "100a365c", DAT_100a365c, 0);
            }
        } else if param_1 == 0 {
            return mech_trace::rt!(
                "100575d0",
                mech_trace::gr!("100575d0", "100a365c", DAT_100a365c)
            );
        } else {
            mech_trace::gw!("100575d0", "100a365c", DAT_100a365c, 0);
        }
        let mut cursor: *const u32 = (&raw mut DAT_100a3668) as *const u32;
        loop {
            let handler = mech_trace::rd!("100575d0", cursor, read_unaligned);
            if handler != 0 {
                let f1 = mech_trace::rd!("100575d0", (handler as *const u32), read_unaligned);
                let count = mech_trace::cr!(
                    "100575d0",
                    "ind",
                    core::mem::transmute::<u32, unsafe extern "C" fn() -> i32>(f1)()
                );
                if count != 0 {
                    let mut device_index: i32 = 0;
                    if 0 < count {
                        loop {
                            let record_index =
                                mech_trace::gr!("100575d0", "100a365c", DAT_100a365c);
                            mech_trace::gw!(
                                "100575d0",
                                "100a365c",
                                DAT_100a365c,
                                mech_trace::gr!("100575d0", "100a365c", DAT_100a365c)
                                    .wrapping_add(1)
                            );
                            if mech_trace::gr!("100575d0", "100a3660", DAT_100a3660)
                                < mech_trace::gr!("100575d0", "100a365c", DAT_100a365c)
                            {
                                let r = mech_trace::cr!("100575d0", "100578d0", FUN_100578d0());
                                if r == 0 {
                                    return mech_trace::rt!("100575d0", 0);
                                }
                            }
                            let f2 = mech_trace::rd!(
                                "100575d0",
                                ((handler.wrapping_add(4)) as *const u32),
                                read_unaligned
                            );
                            let off = (record_index as u32).wrapping_mul(0x78);
                            let base = mech_trace::gr!("100575d0", "100a3664", DAT_100a3664);
                            let r = mech_trace::cr!(
                                "100575d0",
                                "ind",
                                core::mem::transmute::<u32, unsafe extern "C" fn(i32, u32) -> i32>(
                                    f2
                                )(
                                    device_index, base.wrapping_add(off).wrapping_add(4)
                                )
                            );
                            if r == 0 {
                                let dst = mech_trace::gr!("100575d0", "100a3664", DAT_100a3664)
                                    .wrapping_add(off)
                                    as *mut u32;
                                mech_trace::wr!(
                                    "100575d0",
                                    dst,
                                    write_unaligned,
                                    mech_trace::rd!("100575d0", cursor, read_unaligned)
                                );
                            } else {
                                mech_trace::gw!(
                                    "100575d0",
                                    "100a365c",
                                    DAT_100a365c,
                                    mech_trace::gr!("100575d0", "100a365c", DAT_100a365c)
                                        .wrapping_sub(1)
                                );
                            }
                            device_index = device_index.wrapping_add(1);
                            if !(device_index < count) {
                                break;
                            }
                        }
                    }
                }
            }
            cursor = cursor.add(1);
            if !(cursor < (&raw mut DAT_100a3674) as *const u32) {
                break;
            }
        }
        mech_trace::rt!(
            "100575d0",
            mech_trace::gr!("100575d0", "100a365c", DAT_100a365c)
        )
    }
}
