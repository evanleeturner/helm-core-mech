// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000c220(param_1: i32, param_2: i32) -> *mut i32 {
    let mut i_var2: i32 = 0;
    // SAFETY: param_1 points at the game's collection; its count is read at
    // offset 4, and the callee returns item pointers that are read as i32.
    unsafe {
        if 0 < ((param_1 as usize + 4) as *const i32).read_unaligned() {
            loop {
                let pi_var1 = FUN_10018060(param_1, i_var2) as usize as *mut i32;
                if pi_var1.read_unaligned() == param_2 {
                    return pi_var1;
                }
                i_var2 = i_var2.wrapping_add(1);
                if i_var2 >= ((param_1 as usize + 4) as *const i32).read_unaligned() {
                    break;
                }
            }
        }
    }
    core::ptr::null_mut()
}
