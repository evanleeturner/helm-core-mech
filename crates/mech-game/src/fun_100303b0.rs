// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100303b0(param_1: i32, param_2: *mut i32, param_3: *mut i32) {
    if param_1 != 0 {
        // SAFETY: param_1 is a node address and param_2/param_3 point at the
        // caller's list head and tail; accessed exactly as the original does.
        unsafe {
            if param_2.read_unaligned() == 0 && param_3.read_unaligned() == 0 {
                ((param_1 as usize + 0xc) as *mut i32).write_unaligned(0);
                ((param_1 as usize + 0x10) as *mut i32).write_unaligned(0);
                param_2.write_unaligned(param_1);
                param_3.write_unaligned(param_1);
            } else {
                ((param_1 as usize + 0xc) as *mut i32).write_unaligned(param_3.read_unaligned());
                ((param_1 as usize + 0x10) as *mut i32).write_unaligned(0);
                ((param_3.read_unaligned() as usize + 0x10) as *mut i32).write_unaligned(param_1);
                param_3.write_unaligned(param_1);
            }
        }
    }
}
