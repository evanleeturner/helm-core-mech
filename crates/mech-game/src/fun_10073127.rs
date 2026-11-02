// SPDX-License-Identifier: MIT

use core::ffi::c_void;
use core::mem::MaybeUninit;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100b4e98: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut c_void;
    fn FUN_1005d8e0() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005db1f(param_1: i32) -> i32;
    fn _strchr(s: *mut i8, c: i32) -> *mut i8;
    fn _sscanf(src: *mut i8, format: *mut i8, ...) -> i32;
    fn FUN_1007b9d2(param_1: u32, s: *mut i8) -> i32;
    fn _atoi(s: *mut i8) -> i32;
    fn FUN_10032c10(param_1: i32) -> i32;
    fn FUN_1000c8ee(param_1: i32) -> u32;
    fn FUN_1000c9e3(param_1: i32) -> *mut u8;
    fn FUN_10003c3e(param_1: i32);
    fn FUN_1001d5cb(param_1: i32) -> i32;
    fn FUN_1000393d(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10073127(param_1: i32, str: *mut i8) -> u32 {
    let mut local_7c: MaybeUninit<[i8; 100]> = MaybeUninit::uninit();
    // SAFETY: all callees and globals are the game's own; pointers come from
    // the game's heap allocation and its per-thread slot, accessed as the
    // original does.
    unsafe {
        if param_1 == 0 {
            let local_c = HeapAlloc(DAT_100adf74, 1, 0x1e) as *mut i32;
            if local_c.is_null() {
                return 0;
            }
            let iv = FUN_1005d8e0();
            let local_18 = FUN_1005db1f(iv as i32) as *mut i32;
            local_18.write_unaligned(local_c as i32);
            local_c.add(1).write_unaligned(-1);
            local_c.add(2).write_unaligned(0);
            local_c.add(6).write_unaligned(1);
            let mut local_8 = _strchr(str, 0x3b);
            if !local_8.is_null() {
                *local_8 = 0;
                local_8 = local_8.add(1);
                _sscanf(
                    local_8,
                    &raw mut DAT_100b4e98 as *mut i8,
                    local_c,
                    local_7c.as_mut_ptr() as *mut i8,
                    local_c.add(5),
                );
                let v = local_c.read_unaligned();
                local_c.write_unaligned(v.wrapping_mul(100));
            }
            let iv = FUN_1007b9d2(10, local_7c.as_mut_ptr() as *mut i8);
            local_c.add(7).cast::<i16>().write_unaligned(iv as i16);
            let mut local_14 = _atoi(str);
            local_14 = FUN_10032c10(local_14);
            let local_10 = FUN_1000c8ee(local_14) as i32;
            if local_10 == -1 {
                return 0;
            }
            let p = FUN_1000c9e3(local_10);
            local_c.add(3).write_unaligned(p as i32);
        } else if param_1 == 1 {
            let iv = FUN_1005d8e0();
            let local_18 = FUN_1005db1f(iv as i32) as *mut i32;
            let local_c = local_18.read_unaligned() as *mut i32;
            if local_c.is_null() {
                return 0;
            }
            let c3 = local_c.add(3).read_unaligned();
            if c3 == 0 || local_c.add(5).read_unaligned() == 0 {
                FUN_10003c3e(local_c as i32);
                return 0;
            }
            if (c3 as *const i32).read_unaligned() == 0 {
                FUN_10003c3e(local_c as i32);
                return 0;
            }
            let c3 = local_c.add(3).read_unaligned();
            let r = FUN_1001d5cb((c3 as *const i32).read_unaligned());
            local_c.add(4).write_unaligned(r);
            FUN_1000393d(local_c);
        } else if param_1 == 2 {
            let iv = FUN_1005d8e0();
            let local_18 = FUN_1005db1f(iv as i32) as *mut i32;
            let local_c = local_18.read_unaligned() as *mut i32;
            if local_c.is_null() {
                return 1;
            }
            FUN_10003c3e(local_c as i32);
        }
    }
    1
}
