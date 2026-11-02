// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10003da2(param_1: *mut i32) {
    // SAFETY: param_1 points at a pointer-sized value that is itself the
    // address of two writable u32 slots, written in the original's order.
    unsafe {
        let p = param_1.read_unaligned() as u32;
        (p as *mut u32).write_unaligned(0);
        ((p.wrapping_add(4)) as *mut u32).write_unaligned(0);
    }
}
