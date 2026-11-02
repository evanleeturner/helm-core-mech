// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf6c: *mut c_void;
    static mut DAT_100adf88: u32;
    static mut DAT_100b34d8: u32;
    static mut DAT_100b9a18: u32;
    static mut DAT_100b9a1c: u32;
    static mut DAT_100b9a20: *mut u8;
    static mut DAT_100b9a24: u32;
    static mut DAT_100b9a2c: u32;
    static mut DAT_100b9a30: [u8; 0];
    static mut DAT_100b9d54: u32;
    static mut DAT_100b9d58: u32;
    static mut DAT_100b9d5c: u32;
    static mut DAT_100c9240: u32;
    static mut DAT_100c9244: u32;
    static mut DAT_100c9248: u32;
    static mut DAT_100c924c: u32;
    static mut DAT_100c967c: u32;
    static mut DAT_100c9680: u32;
    static mut DAT_100b99f0: [u32; 0];
    static mut DAT_100b9a00: [u32; 0];
    static mut DAT_100b9e30: [i8; 0];
    static mut DAT_100b9eb0: [i8; 0];
    static mut DAT_100b9e08: [i8; 0];
    static mut DAT_100b9e88: [i8; 0];
    static mut DAT_100b9f10: [i8; 0];
    static mut DAT_100b9f38: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *mut i8) -> u8;
    fn FUN_10036b20(param_1: u16, param_2: i16) -> u8;
    fn FUN_10002110(param_1: *mut i8) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetWindowRect(hwnd: *mut c_void, rect: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10081fdc() {
    mech_trace::enter!("10081fdc");
    // SAFETY: all globals, callees and the pointers derived from them belong to
    // the game; reads, writes and calls happen in the original's order.
    unsafe {
        if mech_trace::gr!("10081fdc", "100b9a24", DAT_100b9a24) == 0
            && (mech_trace::gr!("10081fdc", "100b9d54", DAT_100b9d54) != 0
                || mech_trace::gr!("10081fdc", "100c967c", DAT_100c967c)
                    < mech_trace::gr!("10081fdc", "100b9d58", DAT_100b9d58)
                || mech_trace::gr!("10081fdc", "100c9680", DAT_100c9680)
                    < mech_trace::gr!("10081fdc", "100b9d5c", DAT_100b9d5c))
        {
            mech_trace::cr!(
                "10081fdc",
                "10002168",
                FUN_10002168((&raw mut DAT_100b9e08).cast::<i8>())
            );
            mech_trace::cr!("10081fdc", "10036b20", FUN_10036b20(0x80, 1));
            let local_8: *mut u8;
            if mech_trace::rd!(
                "10081fdc",
                ((mech_trace::gr!("10081fdc", "100b9a18", DAT_100b9a18) as usize + 4)
                    as *const i32),
                read_unaligned
            ) == 1
            {
                let tbl_a = (&raw mut DAT_100b99f0).cast::<u32>();
                let tbl_b = (&raw mut DAT_100b9a00).cast::<u32>();
                let mut local_10: i32 = 0;
                let mut cur: *mut u8 = core::ptr::null_mut();
                while local_10 < 6 {
                    cur = mech_trace::rd!(
                        "10081fdc",
                        tbl_b.offset(local_10 as isize),
                        read_unaligned
                    ) as usize as *mut u8;
                    if mech_trace::rd!("10081fdc", cur.byte_add(8).cast::<i32>(), read_unaligned)
                        != 0
                    {
                        let idx = mech_trace::rd!(
                            "10081fdc",
                            cur.byte_add(4).cast::<i32>(),
                            read_unaligned
                        );
                        let ent =
                            mech_trace::rd!("10081fdc", tbl_a.offset(idx as isize), read_unaligned);
                        if mech_trace::rd!(
                            "10081fdc",
                            ((ent as usize + 4) as *const i32),
                            read_unaligned
                        ) == 2
                        {
                            break;
                        }
                    }
                    local_10 += 1;
                }
                if local_10 == 6 {
                    mech_trace::cr!(
                        "10081fdc",
                        "10002110",
                        FUN_10002110((&raw mut DAT_100b9e30).cast::<i8>())
                    );
                    if mech_trace::gr!("10081fdc", "100adf88", DAT_100adf88) != 0 {
                        return;
                    }
                    mech_trace::cr!(
                        "10081fdc",
                        "10002168",
                        FUN_10002168((&raw mut DAT_100b9e88).cast::<i8>())
                    );
                    mech_trace::cr!("10081fdc", "10036b20", FUN_10036b20(0x80, 0));
                    return;
                }
                local_8 = cur;
            } else {
                if mech_trace::gr!("10081fdc", "100b9a20", DAT_100b9a20).is_null() {
                    mech_trace::cr!(
                        "10081fdc",
                        "10002110",
                        FUN_10002110((&raw mut DAT_100b9eb0).cast::<i8>())
                    );
                    if mech_trace::gr!("10081fdc", "100adf88", DAT_100adf88) != 0 {
                        return;
                    }
                    mech_trace::cr!(
                        "10081fdc",
                        "10002168",
                        FUN_10002168((&raw mut DAT_100b9f10).cast::<i8>())
                    );
                    mech_trace::cr!("10081fdc", "10036b20", FUN_10036b20(0x80, 0));
                    return;
                }
                local_8 = mech_trace::gr!("10081fdc", "100b9a20", DAT_100b9a20);
                mech_trace::cr!(
                    "10081fdc",
                    "GetWindowRect",
                    GetWindowRect(
                        mech_trace::gr!("10081fdc", "100adf6c", DAT_100adf6c),
                        (&raw mut DAT_100c9240).cast::<c_void>()
                    )
                );
                mech_trace::gw!(
                    "10081fdc",
                    "100c9248",
                    DAT_100c9248,
                    mech_trace::gr!("10081fdc", "100c9248", DAT_100c9248)
                        .wrapping_sub(mech_trace::gr!("10081fdc", "100c9240", DAT_100c9240))
                );
                mech_trace::gw!(
                    "10081fdc",
                    "100c924c",
                    DAT_100c924c,
                    mech_trace::gr!("10081fdc", "100c924c", DAT_100c924c)
                        .wrapping_sub(mech_trace::gr!("10081fdc", "100c9244", DAT_100c9244))
                );
            }
            let f1 = mech_trace::rd!(
                "10081fdc",
                ((mech_trace::gr!("10081fdc", "100b9a1c", DAT_100b9a1c) as usize + 0x14)
                    as *const usize),
                read_unaligned
            );
            let f1: unsafe extern "C" fn() = core::mem::transmute(f1);
            mech_trace::cr!("10081fdc", "ind", f1());
            mech_trace::gw!(
                "10081fdc",
                "100b9a1c",
                DAT_100b9a1c,
                local_8 as usize as u32
            );
            let f2 = mech_trace::rd!(
                "10081fdc",
                local_8.byte_add(0x10).cast::<usize>(),
                read_unaligned
            );
            let f2: unsafe extern "C" fn(u32, u32, u32) = core::mem::transmute(f2);
            mech_trace::cr!(
                "10081fdc",
                "ind",
                f2(
                    mech_trace::gr!("10081fdc", "100b9a2c", DAT_100b9a2c),
                    mech_trace::gr!("10081fdc", "100c967c", DAT_100c967c),
                    mech_trace::gr!("10081fdc", "100c9680", DAT_100c9680)
                )
            );
            let f3 = mech_trace::rd!(
                "10081fdc",
                ((mech_trace::gr!("10081fdc", "100b9a18", DAT_100b9a18) as usize + 0x14)
                    as *const usize),
                read_unaligned
            );
            let f3: unsafe extern "C" fn(i32, i32, *mut u8, i32) = core::mem::transmute(f3);
            mech_trace::cr!(
                "10081fdc",
                "ind",
                f3(0, 0x100, (&raw mut DAT_100b9a30).cast::<u8>(), 1)
            );
            mech_trace::gw!("10081fdc", "100b34d8", DAT_100b34d8, 1);
            if mech_trace::gr!("10081fdc", "100adf88", DAT_100adf88) == 0 {
                mech_trace::cr!(
                    "10081fdc",
                    "10002168",
                    FUN_10002168((&raw mut DAT_100b9f38).cast::<i8>())
                );
                mech_trace::cr!("10081fdc", "10036b20", FUN_10036b20(0x80, 0));
            }
        }
    }
}
