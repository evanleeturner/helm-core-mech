// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036384(param_1: i32, param_2: i32) {
    if param_1 != 0 && param_2 != 0 {
        // SAFETY: both params are non-null addresses of the caller's structures;
        // the dword at param_1 + 8 is read and written to param_2 + 0xc.
        unsafe {
            let v = ((param_1 as u32).wrapping_add(8) as *const u32).read_unaligned();
            ((param_2 as u32).wrapping_add(0xc) as *mut u32).write_unaligned(v);
        }
    }
}
