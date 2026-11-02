// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7590: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10089060(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d016(param_1: *mut i32) {
    // SAFETY: the global and the callee are the game's own; param_1 points at
    // the caller's three consecutive ints, only addressed here.
    unsafe {
        if DAT_100a7590 != -1 {
            FUN_10089060(
                (DAT_100a7590 as u32)
                    .wrapping_mul(0x7c)
                    .wrapping_add(0x101f5568) as *mut i32,
                param_1,
                param_1.wrapping_add(1),
                param_1.wrapping_add(2),
            );
        }
    }
}
