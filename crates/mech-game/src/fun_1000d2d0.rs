// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f6518: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005da75(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d2d0(param_1: i32) {
    // SAFETY: the global and the callee are the game's own; only the address
    // of the global is taken, offset in int-sized elements as in the C.
    unsafe {
        let p = (&raw mut DAT_101f6518).wrapping_offset(param_1.wrapping_mul(0x7c) as isize);
        FUN_1005da75(p);
    }
}
