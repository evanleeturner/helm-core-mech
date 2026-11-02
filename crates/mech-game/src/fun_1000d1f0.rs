// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f6518: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005d8f5(param_1: *mut i32, param_2: *mut u8, param_3: u32, param_4: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d1f0(param_1: i32, param_2: *mut u8, param_3: u32, param_4: u32) {
    // SAFETY: the global and the callee are the game's own; the computed
    // address is only passed on, never dereferenced here.
    unsafe {
        let p = (&raw mut DAT_101f6518)
            .wrapping_offset(param_1.wrapping_mul(0x7c) as isize)
            .cast::<i32>();
        FUN_1005d8f5(p, param_2, param_3, param_4);
    }
}
