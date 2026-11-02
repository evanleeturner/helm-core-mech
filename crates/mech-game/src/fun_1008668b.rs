// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100b0a08: u32;
    static mut DAT_100ba7bc: u32;
    static mut DAT_100c9050: u32;
    static mut DAT_100c9054: *mut u16;
    static mut DAT_100ba8c8: [i8; 0];
    static mut DAT_100ba920: [i8; 0];
    static mut DAT_100ba8ec: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008628d(param_1: u32, param_2: *mut i32) -> *mut u16;
    fn FUN_10085fb3(param_1: i32) -> u8;
    fn FUN_10086336(param_1: *mut u16) -> u8;
    fn FUN_1005be24(param_1: *const i8, param_2: i32, param_3: i32, param_4: i32) -> u32;
    fn FUN_1001e8a8(param_1: i32, param_2: *mut i8, param_3: u16) -> i32;
    fn FUN_1001ec56(param_1: i32, param_2: *mut i8, param_3: u16, param_4: *mut c_void) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16 {
    // SAFETY: the globals and callees are the game's own; param_3 points at
    // the caller's key bytes and cache entries are read and written where
    // the original does.
    unsafe {
        if (param_2 as i32) < 0 {
            return core::ptr::null_mut();
        }
        let mut p = FUN_1008628d(param_2, param_3);
        if p.is_null() {
            if 999 < DAT_100c9050 {
                if DAT_100c9054.is_null() {
                    if DAT_100b0a08 == 0 {
                        FUN_1005be24((&raw mut DAT_100ba8ec) as *const i8, 1, 0x712, 100);
                    } else {
                        FUN_1005be24((&raw mut DAT_100ba8c8) as *const i8, 1, 0x712, 100);
                    }
                    return core::ptr::null_mut();
                }
                FUN_10086336(DAT_100c9054);
            }
            let u_var3 = param_2 as u16;
            let mut i_var2 = FUN_1001e8a8(param_1, param_3 as *mut i8, u_var3);
            if i_var2 < 1 {
                return core::ptr::null_mut();
            }
            loop {
                p = HeapAlloc(DAT_100adf74, 1, i_var2.wrapping_add(0x14) as u32) as *mut u16;
                if !p.is_null() {
                    break;
                }
                if DAT_100c9054.is_null() {
                    FUN_1005be24((&raw mut DAT_100ba920) as *const i8, 1, 0x712, 100);
                    return core::ptr::null_mut();
                }
                FUN_10086336(DAT_100c9054);
            }
            i_var2 = FUN_1001ec56(
                param_1,
                param_3 as *mut i8,
                u_var3,
                p.add(10) as *mut c_void,
            ) as i32;
            if i_var2 == -1 {
                return core::ptr::null_mut();
            }
            let b = param_3 as *const i8;
            let sum = (b.read() as i32)
                .wrapping_add(b.add(1).read() as i32)
                .wrapping_add(b.add(2).read() as i32)
                .wrapping_add(b.add(3).read() as i32)
                .wrapping_add(param_2 as i32);
            let idx = sum % 0x3f1;
            let slot = DAT_100ba7bc.wrapping_add((idx as u32).wrapping_mul(4)) as *mut u32;
            let old = slot.read_unaligned();
            p.add(4).cast::<u32>().write_unaligned(old);
            slot.write_unaligned(p as u32);
            p.add(1).write(1);
            p.write(u_var3);
            p.add(2)
                .cast::<i32>()
                .write_unaligned(param_3.read_unaligned());
            p.add(6).write(0);
            p.add(7).write(0);
            p.add(8).write(0);
            p.add(9).write(0);
            DAT_100c9050 = DAT_100c9050.wrapping_add(1);
            p.add(10)
        } else {
            FUN_10085fb3(p as i32);
            p.add(10)
        }
    }
}
