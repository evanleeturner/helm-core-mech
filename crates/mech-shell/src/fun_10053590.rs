// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009f37c: u32;
    static mut DAT_1009fc44: u8;
    static mut DAT_1009fc38: u8;
    static mut DAT_1009fc30: u8;
    static mut DAT_1009fc58: u8;
    static mut DAT_1009fc48: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100576a0(param_1: i32) -> i32;
    fn FID_conflict__fwprintf(file: *mut c_void, format: *const u8, ...) -> i32;
    fn FUN_10053470(param_1: *mut c_void, str1: i32);
    fn _strncmp(str1: *const u8, str2: *const u8, max_count: usize) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10053590(param_1: *mut c_void, str1: *mut u8, param_3: i32) {
    // SAFETY: callees and globals are the game's own; param_3 and the
    // structures it leads to are the caller's memory, read where the original does.
    unsafe {
        let i_var1 = FUN_100576a0(((param_3.wrapping_add(4)) as *const i32).read_unaligned());
        let i_var2 = ((param_3.wrapping_add(0x10)) as *const i32).read_unaligned();
        if -1 < i_var2 {
            DAT_1009f37c = DAT_1009f37c.wrapping_add(1);
            FID_conflict__fwprintf(param_1, &raw const DAT_1009fc30, str1);
            let tbl = ((i_var1.wrapping_add(0x70)) as *const i32).read_unaligned();
            let val = (tbl.wrapping_add(i_var2.wrapping_mul(4)) as *const u32).read_unaligned();
            FID_conflict__fwprintf(
                param_1,
                &raw const DAT_1009fc38,
                i_var1.wrapping_add(4),
                val,
            );
            FUN_10053470(param_1, param_3);
            FID_conflict__fwprintf(param_1, &raw const DAT_1009fc44);
            let r = _strncmp(str1, &raw const DAT_1009fc48, 0xd);
            if r == 0 {
                FUN_10053590(param_1, &raw mut DAT_1009fc58, param_3);
            }
        }
    }
}
