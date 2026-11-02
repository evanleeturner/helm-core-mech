// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009fcbc: u8;
    static mut DAT_1009fdac: u8;
    static mut DAT_1009fd8c: u8;
    static mut DAT_1009fd5c: u8;
    static mut DAT_1009fda0: u8;
    static mut DAT_1009fd7c: u8;
    static mut DAT_1009fd3c: u8;
    static mut DAT_1009fcb0: u8;
    static mut DAT_1009fd2c: u8;
    static mut DAT_1009fc90: u8;
    static mut DAT_1009fd08: u8;
    static mut DAT_1009fd1c: u8;
    static mut DAT_1009fcf4: u8;
    static mut DAT_1009fcdc: u8;
    static mut DAT_1009fc9c: u8;
    static mut DAT_1009fd48: u8;
    static mut DAT_1009fd68: u8;
    static mut DAT_1009fcd0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10053500(param_1: *mut c_void, param_2: u32, param_3: i32) -> u8;
    fn FUN_10053590(param_1: *mut c_void, param_2: *mut i8, param_3: i32) -> u8;
    fn FUN_10053630(param_1: *mut c_void, param_2: *mut i8, param_3: *mut u32) -> u8;
}

unsafe fn streq(mut a: *const u8, mut b: *const u8) -> bool {
    // SAFETY: both pointers address NUL-terminated strings; reads stop at
    // the first difference or terminator.
    unsafe {
        loop {
            let x = *a;
            if x != *b {
                return false;
            }
            if x == 0 {
                return true;
            }
            a = a.add(1);
            b = b.add(1);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10053780(param_1: *mut c_void, param_2: *mut u8, param_3: *mut i32) {
    // SAFETY: the globals and callees are the game's own; param_2 is a
    // NUL-terminated string and param_3 points at the caller's int record.
    unsafe {
        if param_3.add(4).read_unaligned() < 0 {
            return;
        }
        let kind = param_3.read_unaligned();
        if kind == 0 {
            FUN_10053500(param_1, param_2 as usize as u32, param_3 as usize as i32);
        } else if kind == 1 {
            FUN_10053590(param_1, param_2.cast::<i8>(), param_3 as usize as i32);
        } else if kind == 2 {
            FUN_10053630(param_1, param_2.cast::<i8>(), param_3.cast::<u32>());
        }

        if streq(param_2, &raw mut DAT_1009fc90) {
            FUN_10053780(param_1, &raw mut DAT_1009fc9c, param_3);
            return;
        }
        if streq(param_2, &raw mut DAT_1009fcb0) {
            FUN_10053780(param_1, &raw mut DAT_1009fcbc, param_3);
            return;
        }
        if streq(param_2, &raw mut DAT_1009fcd0) {
            let p = param_3.add(2);
            p.write_unaligned(p.read_unaligned() ^ 0x8000_0000u32 as i32);
            FUN_10053780(param_1, &raw mut DAT_1009fcdc, param_3);
            p.write_unaligned(p.read_unaligned() ^ 0x8000_0000u32 as i32);
            return;
        }
        if streq(param_2, &raw mut DAT_1009fcf4) {
            FUN_10053780(param_1, &raw mut DAT_1009fd08, param_3);
            FUN_10053780(param_1, &raw mut DAT_1009fd1c, param_3);
            FUN_10053780(param_1, &raw mut DAT_1009fd2c, param_3);
            return;
        }
        if streq(param_2, &raw mut DAT_1009fd3c) {
            FUN_10053780(param_1, &raw mut DAT_1009fd48, param_3);
            return;
        }
        if streq(param_2, &raw mut DAT_1009fd5c) {
            FUN_10053780(param_1, &raw mut DAT_1009fd68, param_3);
            return;
        }
        if streq(param_2, &raw mut DAT_1009fd7c) {
            FUN_10053780(param_1, &raw mut DAT_1009fd8c, param_3);
            return;
        }
        if streq(param_2, &raw mut DAT_1009fda0) {
            FUN_10053780(param_1, &raw mut DAT_1009fdac, param_3);
        }
    }
}
