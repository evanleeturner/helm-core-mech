// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_snake_case)]
unsafe extern "C" {
    fn qsort(
        base: *mut c_void,
        num: usize,
        size: usize,
        cmp: unsafe extern "C" fn(*const c_void, *const c_void) -> i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10017ff0(param_1: i32) {
    // SAFETY: param_1 is the caller's object address; fields are read and
    // written at the same offsets and in the same order as the original,
    // and qsort is the C runtime's own.
    unsafe {
        let field_d = ((param_1.wrapping_add(0xd)) as *const i32).read_unaligned();
        if param_1 != 0 && field_d == 1 {
            let base = ((param_1.wrapping_add(0x11)) as *const *mut c_void).read_unaligned();
            let num = ((param_1.wrapping_add(4)) as *const usize).read_unaligned();
            let cmp = ((param_1.wrapping_add(0x19))
                as *const unsafe extern "C" fn(*const c_void, *const c_void) -> i32)
                .read_unaligned();
            qsort(base, num, 4, cmp);
            ((param_1.wrapping_add(0xd)) as *mut i32).write_unaligned(0);
        }
    }
}
