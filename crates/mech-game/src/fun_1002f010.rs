// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ad6c0: u32;
    static mut DAT_100ada48: u32;
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100814bb(param_1: i32, param_2: i32) -> i32;
    fn FUN_1002ff42(param_1: i32) -> i32;
    fn FUN_1008975d(param_1: *mut u32) -> u8;
}

unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut core::ffi::c_void, flags: u32, size: u32) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f010(param_1: i32, param_2: u8) -> *mut i32 {
    mech_trace::enter!("1002f010", param_1, param_2);
    // SAFETY: the callees and globals are the game's own; param_1 and the
    // allocated block are accessed at the same offsets as the original.
    unsafe {
        let local_8: *mut i32;
        if (param_2 & 2) == 0 {
            if (param_2 & 4) == 0 {
                return mech_trace::rt!("1002f010", core::ptr::null_mut());
            }
            local_8 = mech_trace::cr!(
                "1002f010",
                "HeapAlloc",
                HeapAlloc(
                    mech_trace::gr!("1002f010", "100adf74", DAT_100adf74),
                    1,
                    0x7c
                )
            ) as *mut i32;
            if local_8.is_null() {
                return mech_trace::rt!("1002f010", core::ptr::null_mut());
            }
            mech_trace::wr!("1002f010", local_8.add(0x1d), write_unaligned, 4);
        } else {
            local_8 = mech_trace::cr!(
                "1002f010",
                "100814bb",
                FUN_100814bb(
                    0x7c,
                    mech_trace::gr!("1002f010", "100ad6c0", DAT_100ad6c0) as i32
                )
            ) as *mut i32;
            if local_8.is_null() {
                return mech_trace::rt!("1002f010", core::ptr::null_mut());
            }
            mech_trace::wr!("1002f010", local_8.add(0x1d), write_unaligned, 2);
        }
        mech_trace::wr!("1002f010", local_8, write_unaligned, param_1);
        mech_trace::wr!("1002f010", local_8.add(1), write_unaligned, 0);
        mech_trace::wr!("1002f010", local_8.add(2), write_unaligned, 0);
        if param_1 != 0 {
            if (param_2 & 0x10) == 0 {
                let p = (param_1 as usize + 4) as *mut *mut i32;
                mech_trace::wr!(
                    "1002f010",
                    local_8.add(2),
                    write_unaligned,
                    mech_trace::rd!(
                        "1002f010",
                        ((param_1 as usize + 4) as *const i32),
                        read_unaligned
                    )
                );
                mech_trace::wr!("1002f010", p, write_unaligned, local_8);
            } else {
                let ivar1 = mech_trace::cr!("1002f010", "1002ff42", FUN_1002ff42(param_1));
                if ivar1 == 0 {
                    mech_trace::wr!(
                        "1002f010",
                        ((param_1 as usize + 4) as *mut *mut i32),
                        write_unaligned,
                        local_8
                    );
                } else {
                    mech_trace::wr!(
                        "1002f010",
                        ((ivar1 as usize + 8) as *mut *mut i32),
                        write_unaligned,
                        local_8
                    );
                }
            }
        }
        mech_trace::cr!(
            "1002f010",
            "1008975d",
            FUN_1008975d(local_8.add(3) as *mut u32)
        );
        if param_1 == 0 {
            mech_trace::cr!(
                "1002f010",
                "1008975d",
                FUN_1008975d(local_8.add(0xf) as *mut u32)
            );
        } else {
            let mut pivar2 = (param_1 as usize + 0x3c) as *const i32;
            let mut pivar3 = local_8.add(0xf);
            let mut ivar1: i32 = 0xc;
            while ivar1 != 0 {
                mech_trace::wr!(
                    "1002f010",
                    pivar3,
                    write_unaligned,
                    mech_trace::rd!("1002f010", pivar2, read_unaligned)
                );
                pivar2 = pivar2.add(1);
                pivar3 = pivar3.add(1);
                ivar1 = ivar1.wrapping_sub(1);
            }
        }
        mech_trace::wr!("1002f010", local_8.add(0x1b), write_unaligned, 0);
        let f = mech_trace::rd!("1002f010", local_8.add(0x1d), read_unaligned);
        mech_trace::wr!("1002f010", local_8.add(0x1d), write_unaligned, f | 1);
        mech_trace::wr!("1002f010", local_8.add(0x1c), write_unaligned, 0);
        mech_trace::wr!(
            "1002f010",
            local_8.add(0x1e),
            write_unaligned,
            mech_trace::gr!("1002f010", "100ada48", DAT_100ada48) as i32
        );
        mech_trace::gw!(
            "1002f010",
            "100ada48",
            DAT_100ada48,
            (mech_trace::gr!("1002f010", "100ada48", DAT_100ada48).wrapping_add(7) & 0x7f)
                .wrapping_add(0x40)
        );
        mech_trace::rt!("1002f010", local_8)
    }
}
