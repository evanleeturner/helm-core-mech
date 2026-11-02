// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: u32;
    static mut DAT_100af098: *mut u8;
    static mut DAT_100c6d00: [u8; 0];
    static mut DAT_100c6d08: *mut u8;
    static mut DAT_100c6d20: *mut u8;
    static mut DAT_100c6d38: *mut u8;
    static mut DAT_100c6d50: [u8; 0];
    static mut DAT_100c7ad0: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003dce5(param_1: *mut u8) -> u8;
    fn FUN_1001c989(param_1: *mut c_void) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003e1b7() {
    mech_trace::enter!("1003e1b7");
    // SAFETY: the globals and callees are the game's own; the table at
    // DAT_100c6d50 is read at the same offsets the original uses.
    unsafe {
        if mech_trace::gr!("1003e1b7", "100aefc0", DAT_100aefc0) == 0 {
            if mech_trace::gr!("1003e1b7", "100af098", DAT_100af098)
                == (&raw mut DAT_100c6d00) as *mut u8
            {
                loop {
                    let p = mech_trace::gr!("1003e1b7", "100c6d08", DAT_100c6d08);
                    if p.is_null() {
                        break;
                    }
                    mech_trace::cr!(
                        "1003e1b7",
                        "1003dce5",
                        FUN_1003dce5(mech_trace::gr!("1003e1b7", "100c6d08", DAT_100c6d08))
                    );
                    mech_trace::cr!("1003e1b7", "1001c989", FUN_1001c989(p as *mut c_void));
                }
            }
        } else {
            let base = (&raw mut DAT_100c6d50) as *mut u8;
            let mut local_8: i32 = 0;
            while local_8 < 0xc {
                let mut local_c: i32 = 0;
                while local_c < 0xc {
                    let addr = base
                        .byte_add(local_c.wrapping_mul(0x18) as usize)
                        .byte_add(local_8.wrapping_mul(0x120) as usize);
                    while mech_trace::rd!("1003e1b7", addr.cast::<i32>(), read_unaligned) != 0 {
                        let p = mech_trace::rd!("1003e1b7", addr.cast::<u32>(), read_unaligned)
                            as *mut c_void;
                        mech_trace::cr!(
                            "1003e1b7",
                            "1003dce5",
                            FUN_1003dce5(mech_trace::rd!(
                                "1003e1b7",
                                addr.cast::<u32>(),
                                read_unaligned
                            ) as *mut u8)
                        );
                        mech_trace::cr!("1003e1b7", "1001c989", FUN_1001c989(p));
                    }
                    local_c = local_c.wrapping_add(1);
                }
                local_8 = local_8.wrapping_add(1);
            }
            loop {
                let p = mech_trace::gr!("1003e1b7", "100c6d20", DAT_100c6d20);
                if p.is_null() {
                    break;
                }
                mech_trace::cr!(
                    "1003e1b7",
                    "1003dce5",
                    FUN_1003dce5(mech_trace::gr!("1003e1b7", "100c6d20", DAT_100c6d20))
                );
                mech_trace::cr!("1003e1b7", "1001c989", FUN_1001c989(p as *mut c_void));
            }
        }
        if mech_trace::gr!("1003e1b7", "100af098", DAT_100af098)
            == (&raw mut DAT_100c6d00) as *mut u8
        {
            loop {
                let p = mech_trace::gr!("1003e1b7", "100c6d38", DAT_100c6d38);
                if p.is_null() {
                    break;
                }
                mech_trace::cr!(
                    "1003e1b7",
                    "1003dce5",
                    FUN_1003dce5(mech_trace::gr!("1003e1b7", "100c6d38", DAT_100c6d38))
                );
                mech_trace::cr!("1003e1b7", "1001c989", FUN_1001c989(p as *mut c_void));
            }
            loop {
                let p = mech_trace::gr!("1003e1b7", "100c7ad0", DAT_100c7ad0);
                if p.is_null() {
                    break;
                }
                mech_trace::cr!(
                    "1003e1b7",
                    "1003dce5",
                    FUN_1003dce5(mech_trace::gr!("1003e1b7", "100c7ad0", DAT_100c7ad0))
                );
                mech_trace::cr!("1003e1b7", "1001c989", FUN_1001c989(p as *mut c_void));
            }
        }
    }
}
