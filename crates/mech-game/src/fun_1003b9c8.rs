// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003b59a(param_1: i32);
    fn FUN_1003b954(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003b9c8(param_1: *mut i32) {
    // SAFETY: param_1 points at the caller's int array; the callees are the
    // game's own functions, called in the original order.
    unsafe {
        param_1.add(0x13).write_unaligned(0);
        param_1.add(0x14).write_unaligned(0);
        FUN_1003b59a(param_1 as i32);
        FUN_1003b954(param_1);
    }
}
