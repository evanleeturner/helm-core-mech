// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100b4e8c: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(hHeap: *mut c_void, dwFlags: u32, dwBytes: u32) -> *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005d8e0() -> i32;
    fn FUN_1005db1f(param_1: i32) -> *mut i32;
    fn strchr(param_1: *const i8, str: i32) -> *mut i8;
    fn sscanf(param_1: *const i8, str: *const i8, ...) -> i32;
    fn atoi(param_1: *const i8) -> i32;
    fn FUN_10032c10(param_1: i32) -> i32;
    fn FUN_1000c8ee(param_1: i32) -> i32;
    fn FUN_1000c9e3(param_1: i32) -> *mut u8;
    fn FUN_1001d5cb(param_1: i32) -> i32;
    fn FUN_10006b9a(param_1: i32) -> i32;
    fn FUN_10006b20(param_1: i32) -> i32;
    fn FUN_10002570(param_1: i32, str: i32) -> i32;
    fn FUN_10003d60(param_1: i32, str: i32) -> i32;
    fn FUN_1002f3a7(param_1: i32, str: i32, param_3: i32, param_4: i32);
    fn FUN_1002fbc0(param_1: *mut i32);
    fn FUN_1002f6a8(param_1: *mut c_void, str: i32, param_3: i32, param_4: i32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10072e03(param_1: i32, str: *mut i8, param_3: i32) -> u32 {
    mech_trace::enter!("10072e03", param_1, str, param_3);
    // SAFETY: the callees and globals are the game's own; str points at the
    // caller's NUL-terminated buffer and the record at local_14 is the
    // game's 0x1c-byte per-thread block.
    unsafe {
        if param_1 == 0 {
            let local_14 = mech_trace::cr!(
                "10072e03",
                "HeapAlloc",
                HeapAlloc(
                    mech_trace::gr!("10072e03", "100adf74", DAT_100adf74),
                    1,
                    0x1c
                )
            ) as *mut i32;
            if local_14.is_null() {
                return mech_trace::rt!("10072e03", 0);
            }
            let i_var3 = mech_trace::cr!("10072e03", "1005d8e0", FUN_1005d8e0());
            let p = mech_trace::cr!("10072e03", "1005db1f", FUN_1005db1f(i_var3));
            mech_trace::wr!("10072e03", p, write_unaligned, local_14 as usize as i32);
            mech_trace::wr!("10072e03", local_14.add(5), write_unaligned, 0);
            mech_trace::wr!("10072e03", local_14.add(1), write_unaligned, 0);
            let mut local_8 = mech_trace::cr!("10072e03", "strchr", strchr(str, 0x3b));
            if !local_8.is_null() {
                mech_trace::dw!("10072e03", local_8, 0);
                local_8 = local_8.add(1);
                let mut local_2c: f32 = 0.0;
                let mut local_34: f32 = 0.0;
                let mut local_18: i32 = 0;
                let mut local_30: i32 = 0;
                mech_trace::cr!(
                    "10072e03",
                    "sscanf",
                    sscanf(
                        local_8,
                        (&raw mut DAT_100b4e8c) as *const i8,
                        &raw mut local_2c,
                        &raw mut local_34,
                        &raw mut local_18,
                        &raw mut local_30,
                    )
                );
                mech_trace::wr!(
                    "10072e03",
                    local_14.add(3),
                    write_unaligned,
                    local_2c as i32
                );
                mech_trace::wr!(
                    "10072e03",
                    local_14.add(4),
                    write_unaligned,
                    local_34 as i32
                );
                mech_trace::wr!("10072e03", local_14.add(5), write_unaligned, local_18);
                mech_trace::wr!("10072e03", local_14.add(2), write_unaligned, 0);
            }
            mech_trace::wr!("10072e03", local_14.add(6), write_unaligned, param_3);
            let i_var3 = mech_trace::cr!("10072e03", "atoi", atoi(str));
            let i_var3 = mech_trace::cr!("10072e03", "10032c10", FUN_10032c10(i_var3));
            let local_24 = mech_trace::cr!("10072e03", "1000c8ee", FUN_1000c8ee(i_var3));
            if local_24 == -1 {
                return mech_trace::rt!("10072e03", 0);
            }
            let puvar2 = mech_trace::cr!("10072e03", "1000c9e3", FUN_1000c9e3(local_24));
            mech_trace::wr!(
                "10072e03",
                local_14,
                write_unaligned,
                puvar2 as usize as i32
            );
        } else if param_1 == 1 {
            let i_var3 = mech_trace::cr!("10072e03", "1005d8e0", FUN_1005d8e0());
            let pp = mech_trace::cr!("10072e03", "1005db1f", FUN_1005db1f(i_var3));
            let local_14 = mech_trace::rd!("10072e03", pp, read_unaligned) as usize as *mut i32;
            if local_14.is_null() {
                return mech_trace::rt!("10072e03", 0);
            }
            if mech_trace::rd!("10072e03", local_14, read_unaligned) == 0
                || mech_trace::rd!("10072e03", local_14.add(5), read_unaligned) == 0
            {
                return mech_trace::rt!("10072e03", 0);
            }
            let inner =
                mech_trace::rd!("10072e03", local_14, read_unaligned) as usize as *const i32;
            if mech_trace::rd!("10072e03", inner, read_unaligned) == 0 {
                return mech_trace::rt!("10072e03", 0);
            }
            let i_var3 = mech_trace::cr!(
                "10072e03",
                "1001d5cb",
                FUN_1001d5cb(mech_trace::rd!("10072e03", inner, read_unaligned))
            );
            mech_trace::wr!("10072e03", local_14.add(1), write_unaligned, i_var3);
            let local_10 = mech_trace::cr!(
                "10072e03",
                "10006b9a",
                FUN_10006b9a(mech_trace::rd!("10072e03", local_14.add(2), read_unaligned))
            );
            let local_28 = mech_trace::cr!(
                "10072e03",
                "10006b20",
                FUN_10006b20(mech_trace::rd!("10072e03", local_14.add(2), read_unaligned))
            );
            let local_1c = mech_trace::cr!(
                "10072e03",
                "10002570",
                FUN_10002570(
                    param_3.wrapping_sub(mech_trace::rd!(
                        "10072e03",
                        local_14.add(6),
                        read_unaligned
                    )),
                    0xb5
                )
            );
            mech_trace::wr!("10072e03", local_14.add(6), write_unaligned, param_3);
            let mut local_20 = mech_trace::cr!(
                "10072e03",
                "10003d60",
                FUN_10003d60(
                    mech_trace::rd!("10072e03", local_14.add(4), read_unaligned),
                    local_1c
                )
            );
            local_20 >>= 0x10;
            let i_var3 = mech_trace::cr!("10072e03", "10003d60", FUN_10003d60(local_10, local_20));
            let mut local_c = mech_trace::cr!(
                "10072e03",
                "10003d60",
                FUN_10003d60(local_28.wrapping_neg(), local_20)
            );
            local_c >>= 0xd;
            FUN_1002f3a7(
                mech_trace::rd!("10072e03", local_14.add(1), read_unaligned),
                i_var3 >> 0xd,
                0,
                local_c,
            );
            FUN_1002fbc0(
                mech_trace::rd!("10072e03", local_14.add(1), read_unaligned) as usize as *mut i32,
            );
            local_20 = mech_trace::cr!(
                "10072e03",
                "10003d60",
                FUN_10003d60(
                    mech_trace::rd!("10072e03", local_14.add(3), read_unaligned),
                    local_1c
                )
            );
            FUN_1002f6a8(
                mech_trace::rd!("10072e03", local_14.add(1), read_unaligned) as usize
                    as *mut c_void,
                0,
                local_20,
                0,
                0,
            );
            FUN_1002fbc0(
                mech_trace::rd!("10072e03", local_14.add(1), read_unaligned) as usize as *mut i32,
            );
            let v =
                mech_trace::rd!("10072e03", local_14.add(2), read_unaligned).wrapping_add(local_20);
            mech_trace::wr!("10072e03", local_14.add(2), write_unaligned, v);
            mech_trace::wr!("10072e03", local_14.add(2), write_unaligned, v % 0x1680000);
        }
        mech_trace::rt!("10072e03", 1)
    }
}
