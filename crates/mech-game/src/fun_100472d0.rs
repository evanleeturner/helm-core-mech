// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100af0dc: u32;
    static mut DAT_100af148: *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007bbed(
        param_1: *mut i16,
        param_2: *mut i32,
        param_3: *mut i8,
        param_4: u32,
        param_5: *mut u32,
        param_6: *mut i32,
    ) -> *mut u16;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32);
}

unsafe extern "stdcall" {
    fn HeapFree(h: *mut c_void, flags: u32, mem: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100472d0(
    param_1: *mut i16,
    param_2: *mut u32,
    param_3: *mut u32,
    param_4: *mut u32,
    param_5: *mut u32,
    param_6: *mut u32,
    param_7: *mut u32,
    param_8: *mut u32,
) -> u32 {
    let mut local_8: u32 = 0;
    // SAFETY: the callees and globals are the game's own; the out pointers and
    // the returned buffer are valid for the reads and writes the original does.
    unsafe {
        let lp_mem = FUN_1007bbed(
            param_1,
            DAT_100af0dc as *mut i32,
            DAT_100af148,
            7,
            &raw mut local_8,
            core::ptr::null_mut(),
        );
        if lp_mem.is_null() {
            return 0;
        }
        param_2.write_unaligned(lp_mem.byte_add(0).cast::<u32>().read_unaligned());
        param_3.write_unaligned(lp_mem.byte_add(4).cast::<u32>().read_unaligned());
        param_4.write_unaligned(lp_mem.byte_add(8).cast::<u32>().read_unaligned());
        param_5.write_unaligned(lp_mem.byte_add(12).cast::<u32>().read_unaligned());
        param_6.write_unaligned(lp_mem.byte_add(16).cast::<u32>().read_unaligned());
        param_7.write_unaligned(lp_mem.byte_add(20).cast::<u32>().read_unaligned());
        param_8.write_unaligned(lp_mem.byte_add(24).cast::<u32>().read_unaligned());
        if param_1.read() == -1 {
            HeapFree(DAT_100adf74, 1, lp_mem.cast::<c_void>());
        } else {
            FUN_1008664f(param_1.read() as u32, DAT_100af0dc as *mut i32);
        }
        1
    }
}
