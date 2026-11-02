// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005fbab(
        param_1: *mut u16,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut u32,
        param_6: *mut u32,
        param_7: *mut i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005fb00(
    param_1: *mut u16,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    let mut local_10: u32 = 0;
    let mut local_c: i32 = 0;
    let mut local_8: u32 = 0;
    // SAFETY: the callee is the game's own; the out-pointers refer to live locals.
    unsafe {
        FUN_1005fbab(
            param_1,
            param_2,
            param_3,
            param_4,
            &raw mut local_8,
            &raw mut local_10,
            &raw mut local_c,
        );
    }
    local_8
}
