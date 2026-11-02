// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10089643(param_1: *mut u32, param_2: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003b9ff(param_1: i32, param_2: *mut u32) {
    // SAFETY: the callee is the game's own; param_1 is the address of the
    // caller's object and param_2 points at the caller's dwords, read and
    // written at the same offsets as the original.
    unsafe {
        FUN_10089643(param_2, param_1.wrapping_add(0x54) as *mut u32);
        ((param_1.wrapping_add(0x78)) as *mut u32).write_unaligned(param_2.add(9).read_unaligned());
        ((param_1.wrapping_add(0x7c)) as *mut u32)
            .write_unaligned(param_2.add(10).read_unaligned());
        ((param_1.wrapping_add(0x80)) as *mut u32)
            .write_unaligned(param_2.add(0xb).read_unaligned());
    }
}
