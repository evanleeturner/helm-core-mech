// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000cec0(param_1: i32) -> u32 {
    let mut i_var2: i32 = 0;
    // SAFETY: param_1 points at the game's collection object; the callee and
    // the item pointers it returns are the game's own, read where the original does.
    unsafe {
        if (param_1.wrapping_add(4) as *const i32).read_unaligned() > 0 {
            loop {
                let i_var1 = FUN_10018060(param_1, i_var2);
                if (i_var1.wrapping_add(0x2064) as *const i8).read() == 1
                    && (i_var1.wrapping_add(0x20) as *const i32).read_unaligned() != -1
                {
                    return ((i_var1 >> 8) | 1) as u32;
                }
                i_var2 = i_var2.wrapping_add(1);
                let in_eax = (param_1.wrapping_add(4) as *const u32).read_unaligned();
                if !(i_var2 < in_eax as i32) {
                    break;
                }
            }
        }
    }
    0
}
