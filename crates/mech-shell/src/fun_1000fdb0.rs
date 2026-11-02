// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c4304: i32;
    static mut DAT_100c45b6: i32;
    static mut DAT_100c42c4: i32;
    static mut DAT_100c42c0: u32;
    static mut DAT_100c4308: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn LAB_1000fcb0(param_1: *const c_void, param_2: *const c_void) -> i32;
    fn FUN_1000fce0();
    fn _qsort(
        base: *mut c_void,
        num: u32,
        width: u32,
        compare: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> i32>,
    );
    fn FUN_1000fd70();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000fdb0(param_1: *mut u32) {
    // SAFETY: the globals and callees are the game's own memory and code;
    // param_1 points at a record of 0x45 bytes, read where the original does.
    unsafe {
        let mut i_var2: i32 = 500000000;
        let mut i_var4: i32 = -1;
        FUN_1000fce0();
        let mut i_var3: i32 = 0;
        let mut p1: *mut u8 = (&raw mut DAT_100c4304).cast::<u8>();
        let end = (&raw mut DAT_100c45b6) as usize;
        loop {
            p1.byte_add(4).write(0);
            let v = p1.cast::<i32>().read_unaligned();
            if v < i_var2 {
                i_var2 = v;
                i_var4 = i_var3;
            }
            p1 = p1.byte_add(0x45);
            i_var3 = i_var3.wrapping_add(1);
            if (p1 as usize) >= end {
                break;
            }
        }

        let limit = param_1.add(0x10).read_unaligned() as i32;
        if i_var2 < limit && -1 < i_var4 && i_var4 < 10 {
            let off = i_var4.wrapping_mul(0x45) as isize;
            let mut src = param_1;
            let mut dst = (&raw mut DAT_100c42c4)
                .cast::<u8>()
                .byte_offset(off)
                .cast::<u32>();
            let mut n: i32 = 0x11;
            while n != 0 {
                dst.write_unaligned(src.read_unaligned());
                src = src.add(1);
                dst = dst.add(1);
                n -= 1;
            }
            dst.cast::<u8>().write(src.cast::<u8>().read());
            (&raw mut DAT_100c4308)
                .cast::<u8>()
                .byte_offset(off)
                .write(1);
            _qsort(
                (&raw mut DAT_100c42c4).cast::<c_void>(),
                DAT_100c42c0,
                0x45,
                Some(LAB_1000fcb0),
            );
            FUN_1000fd70();
        }
    }
}
