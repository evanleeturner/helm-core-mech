// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d37d(param_1: i32, param_2: unsafe extern "C" fn(i32)) {
    if param_1 != 0 {
        // SAFETY: param_1 is a pointer to the game's list head; each node's
        // next link lives at offset 8 and is read before the callback runs.
        unsafe {
            let mut local_8 = ((param_1 as usize + 8) as *const i32).read_unaligned();
            while local_8 != 0 {
                let i_var1 = ((local_8 as usize + 8) as *const i32).read_unaligned();
                param_2(local_8);
                local_8 = i_var1;
            }
        }
    }
}
