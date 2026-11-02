// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f6518: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005d9bf(param_1: *mut i32, param_2: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d29b(param_1: i32, param_2: *mut u32) {
    // SAFETY: the global and the callee are the game's own; the address is
    // computed with wrapping pointer arithmetic exactly as the original does.
    unsafe {
        let base = &raw mut DAT_101f6518;
        let p = base.wrapping_offset(param_1.wrapping_mul(0x7c) as isize);
        FUN_1005d9bf(p.cast::<i32>(), param_2);
    }
}
