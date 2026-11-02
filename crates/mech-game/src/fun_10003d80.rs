// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10003d80(param_1: *mut i32) {
    // SAFETY: param_1 points at a valid int holding the address of two
    // writable 32-bit words, written where the original does.
    unsafe {
        let p = param_1.read_unaligned() as *mut u32;
        p.write_unaligned(1);
        let q = (param_1.read_unaligned() as usize).wrapping_add(4) as *mut u32;
        q.write_unaligned(0);
    }
}
