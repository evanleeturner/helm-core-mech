// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100645c5(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10064617() {
    let mut local_8: u32 = 0;
    while local_8 < 0xfe {
        // SAFETY: the callee is the game's own function; it takes the index by value.
        unsafe {
            FUN_100645c5(local_8);
        }
        local_8 = local_8.wrapping_add(1);
    }
}
