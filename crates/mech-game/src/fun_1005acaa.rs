// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005ab17(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005acaa(
    param_1: *mut i32,
    mut param_2: i32,
    param_3: i32,
    param_4: i32,
    mut param_5: *const u8,
    param_6: i32,
) {
    loop {
        // SAFETY: the callee is the game's own; param_5 points at the caller's
        // NUL-terminated byte string, read where the original reads it.
        unsafe {
            let i_var1 = FUN_1005ab17(
                param_1,
                param_2,
                param_3,
                param_4,
                u32::from(*param_5) as i32,
                param_6,
            );
            param_2 = param_2.wrapping_add(i_var1);
            param_5 = param_5.add(1);
            if *param_5 == 0 {
                break;
            }
        }
    }
}
