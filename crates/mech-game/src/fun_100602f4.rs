// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10014db6(param_1: i32, param_2: i32, param_3: *mut i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100602f4(param_1: i32, param_2: *mut i32) -> u32 {
    // SAFETY: param_1 points at the game's object; its offsets are read where
    // the original reads them, and the callee is the game's own.
    unsafe {
        let i_var1 = (param_1.wrapping_add(0x20) as *const i32).read_unaligned();
        if i_var1 != 0 {
            let mut local_10: i32 = 0;
            while local_10 < (i_var1.wrapping_add(6) as *const i16).read_unaligned() as i32 {
                let base = (i_var1.wrapping_add(8) as *const i32).read_unaligned();
                let i_var2 = FUN_10014db6(
                    base.wrapping_add(i_var1)
                        .wrapping_add(local_10.wrapping_mul(0x24)),
                    i_var1.wrapping_add(0x18),
                    param_2,
                );
                if i_var2 != 0 {
                    return 1;
                }
                local_10 = local_10.wrapping_add(1);
            }
        }
        0
    }
}
