// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100afbc0: *mut c_void;
    static mut DAT_100b9a18: *mut i32;
    static mut DAT_100b99f8: *mut i32;
    static mut DAT_100b03ec: i32;
    static mut DAT_100b9d4c: i32;
    static mut DAT_100adf6c: *mut c_void;
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100c9278: [i16; 256];
    static mut DAT_100b9d30: i32;
    static mut DAT_100c9250: [u8; 1];
    static mut DAT_100affd8: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetDC(hWnd: *mut c_void) -> *mut c_void;
    fn HeapAlloc(hHeap: *mut c_void, dwFlags: u32, dwBytes: u32) -> *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10082279(param_1: i32);
    fn FUN_100821fe(param_1: i32, param_2: i32);
    fn FUN_10045873(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_100455b7() -> i32;
    fn FUN_1004541e();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10045290(param_1: *mut i32, param_2: i32, param_3: i32) -> u32 {
    // SAFETY: globals and callees are the game's own; param_1 points at the
    // caller's 5-int structure, and the heap block is sized param_2*param_3.
    unsafe {
        if DAT_100afbc0.is_null() {
            if *DAT_100b9a18 != 2 {
                let f = core::mem::transmute::<u32, unsafe extern "C" fn()>(
                    *DAT_100b9a18.add(4) as u32
                );
                f();
                if DAT_100b03ec != DAT_100b9d4c {
                    FUN_10082279(0x100b03e8);
                }
                DAT_100b9a18 = DAT_100b99f8;
            }
            DAT_100afbc0 = GetDC(DAT_100adf6c);
            FUN_100821fe(param_2, param_3);
            let mut local_c: i32 = 0;
            while local_c < 0x100 {
                let p = (&raw mut DAT_100c9278).cast::<i16>().add(local_c as usize);
                p.write(local_c as i16);
                local_c = local_c.wrapping_add(1);
            }
            let iv = FUN_10045873(0, 0x100, 0x100b9a30);
            if iv == 0 {
                return 0xffff_ffff;
            }
            let size = (param_3 as u32).wrapping_mul(param_2 as u32);
            let pv = HeapAlloc(DAT_100adf74, 9, size);
            param_1.write_unaligned(pv as i32);
            if param_1.read_unaligned() == 0 {
                return 2;
            }
            DAT_100b9d30 = param_1.read_unaligned();
            param_1
                .add(1)
                .write_unaligned((param_2 as u32).wrapping_sub(1) as i32);
            param_1
                .add(2)
                .write_unaligned((param_3 as u32).wrapping_sub(1) as i32);
            param_1.add(4).write_unaligned(0);
            param_1
                .add(3)
                .write_unaligned((&raw mut DAT_100c9250) as i32);
            let mut p5 = param_1.read_unaligned() as u32 as *mut u8;
            let mut n = size >> 2;
            while n != 0 {
                p5.cast::<u32>().write_unaligned(0);
                p5 = p5.byte_add(4);
                n = n.wrapping_sub(1);
            }
            let mut n = size & 3;
            while n != 0 {
                p5.write(0);
                p5 = p5.byte_add(1);
                n = n.wrapping_sub(1);
            }
            if FUN_100455b7() == 0 {
                DAT_100affd8 = 1;
                0
            } else {
                FUN_1004541e();
                1
            }
        } else {
            0
        }
    }
}
