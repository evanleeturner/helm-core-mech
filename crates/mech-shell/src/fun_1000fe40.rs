// SPDX-License-Identifier: MIT

#[repr(C)]
struct Locals {
    a: [i8; 32],
    b: [i8; 32],
    c: i32,
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000ce20() -> i32;
    fn FUN_1000d440() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000fdb0(param_1: *mut u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000fe40() {
    // SAFETY: the callees and globals are the game's own; the source strings
    // are NUL-terminated and copied unbounded into the locals, as the original does.
    unsafe {
        let src = FUN_1000ce20() as *const i8;
        if src.is_null() {
            return;
        }
        let mut l = Locals {
            a: [0; 32],
            b: [0; 32],
            c: 0,
        };
        let base = &raw mut l as *mut i8;

        let mut n: usize = 0;
        while *src.add(n) != 0 {
            n += 1;
        }
        n += 1;
        let mut i = 0;
        while i < n {
            base.add(i).write_volatile(*src.add(i));
            i += 1;
        }

        let src2 = DAT_10083508.wrapping_add(0x102) as *const i8;
        let mut n: usize = 0;
        while *src2.add(n) != 0 {
            n += 1;
        }
        n += 1;
        let dst2 = base.add(32);
        let mut i = 0;
        while i < n {
            dst2.add(i).write_volatile(*src2.add(i));
            i += 1;
        }

        let r = FUN_1000d440();
        let v = (DAT_10083508.wrapping_add(0x12a) as *const i32).read_unaligned();
        (&raw mut l.c).write_volatile(v.wrapping_add(r));
        FUN_1000fdb0(base as *mut u32);
    }
}
