// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000ad70(param_1: i32, param_2: i32) -> i32 {
    if param_1 == 0 {
        return 0;
    }
    let mut i_var2: i32 = 0;
    // SAFETY: param_1 points at the game's collection; the callee and the
    // item pointers it returns are the game's own memory.
    unsafe {
        if 0 < ((param_1 as usize + 4) as *const i32).read_unaligned() {
            loop {
                let i_var1 = FUN_10018060(param_1, i_var2) as i32;
                if ((i_var1 as usize + 0x80c) as *const i32).read_unaligned() == param_2 {
                    return i_var1;
                }
                i_var2 = i_var2.wrapping_add(1);
                if i_var2 >= ((param_1 as usize + 4) as *const i32).read_unaligned() {
                    break;
                }
            }
        }
    }
    0
}
