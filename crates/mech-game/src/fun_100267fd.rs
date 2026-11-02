// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100267fd(param_1: *mut u32, param_2: *const u32) {
    let mut p1 = param_1;
    let mut p2 = param_2;
    let mut i_var1: i32 = 0xe;
    while i_var1 != 0 {
        // SAFETY: the caller supplies buffers of at least 14 dwords; each
        // dword is read from the source and written to the destination.
        unsafe {
            p1.write_unaligned(p2.read_unaligned());
            p2 = p2.add(1);
            p1 = p1.add(1);
        }
        i_var1 = i_var1.wrapping_sub(1);
    }
}
