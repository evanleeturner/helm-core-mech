// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002fc9d(param_1: i32) -> i32;
    fn FUN_10089060(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut i32,
    ) -> u8;
    fn FUN_10024699(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10027e20(param_1: i32) {
    let mut local_20: i32 = 0;
    let mut local_14: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_1c: i32 = -1000;
    let mut local_8: i32 = -0x514;
    // SAFETY: param_1 points at the game's object; the callees are the game's
    // own and receive pointers to live locals.
    unsafe {
        let local_c =
            FUN_1002fc9d(((param_1.wrapping_add(0x40)) as usize as *const i32).read_unaligned())
                as *mut i32;
        FUN_10089060(
            local_c,
            &raw mut local_10,
            &raw mut local_18,
            &raw mut local_1c,
        );
        FUN_10089060(
            local_c,
            &raw mut local_14,
            &raw mut local_20,
            &raw mut local_8,
        );
        FUN_10024699(
            local_10,
            local_18,
            local_1c,
            local_14.wrapping_sub(local_10),
            local_20.wrapping_sub(local_18),
            local_8.wrapping_sub(local_1c),
        );
    }
}
