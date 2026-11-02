// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098f78: *mut u32;
    static mut DAT_10098f80: u32;
    static mut DAT_100ba428: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063f40(param_1: *mut i32) -> *mut i32;
    fn FUN_10063ea0(param_1: *mut i32) -> u8;
    fn FUN_1006e790(param_1: *mut c_void) -> u8;
    fn FUN_10045b60(param_1: i32, param_2: i32) -> *mut u32;
    fn FUN_10063d30(param_1: *mut i32) -> u8;
    fn FUN_10063de0(param_1: *mut i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10044940() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10046720(param_1: i32, param_2: *mut i8) {
    // SAFETY: globals and callees are the game's own; param_2 is a
    // NUL-terminated string copied into the game's buffer as the original does.
    unsafe {
        let mut i_var6: i32 = -1;
        if !param_2.is_null() {
            let mut p = param_2;
            let mut u_var4: u32 = 0xffff_ffff;
            loop {
                let mut p7 = p;
                if u_var4 == 0 {
                    p = p7;
                    break;
                }
                u_var4 = u_var4.wrapping_sub(1);
                p7 = p.byte_add(1);
                let c = *p;
                p = p7;
                if c == 0 {
                    break;
                }
            }
            u_var4 = !u_var4;
            let mut src = p.byte_sub(u_var4 as usize) as *const u8;
            let mut dst = &raw mut DAT_100ba428;
            let mut n = u_var4 >> 2;
            while n != 0 {
                (dst as *mut u32).write_unaligned((src as *const u32).read_unaligned());
                src = src.byte_add(4);
                dst = dst.byte_add(4);
                n = n.wrapping_sub(1);
            }
            let mut m = u_var4 & 3;
            while m != 0 {
                *dst = *src;
                src = src.byte_add(1);
                dst = dst.byte_add(1);
                m = m.wrapping_sub(1);
            }
        }
        let mut pi3 = FUN_10063f40(DAT_10098f78 as *mut i32);
        if !pi3.is_null() && 0 < pi3.add(10).read_unaligned() {
            i_var6 = pi3.add(9).read_unaligned();
        }
        FUN_10063ea0(DAT_10098f78 as *mut i32);
        if !DAT_10098f78.is_null() {
            FUN_1006e790(DAT_10098f78 as *mut c_void);
        }
        DAT_10098f78 = FUN_10045b60(param_1, 0);
        DAT_10098f80 = param_1 as u32;
        FUN_10063d30(DAT_10098f78 as *mut i32);
        if -1 < i_var6 && DAT_10098f78.read_unaligned() != 0xffff_ffff {
            pi3 = DAT_10098f78 as *mut i32;
            while pi3.add(9).read_unaligned() != i_var6 {
                let pi1 = pi3.add(0xe);
                pi3 = pi3.add(0xe);
                if pi1.read_unaligned() == -1 {
                    FUN_10044940();
                    return;
                }
            }
            pi3.add(0xb).write_unaligned(2);
            pi3.add(0x19).write_unaligned(2);
            pi3.add(0x27).write_unaligned(2);
            FUN_10063de0(DAT_10098f78 as *mut i32);
        }
        FUN_10044940();
    }
}
