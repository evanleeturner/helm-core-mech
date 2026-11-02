// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10086973(param_1: *mut u32, param_2: *mut u32, param_3: u32) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008980c(param_1: *mut u32, param_2: *mut u32) {
    // SAFETY: the callee is the game's own; param_2 points at a caller-owned
    // buffer of at least 12 dwords, written at the same offsets as the original.
    unsafe {
        FUN_10086973(param_2, param_1, 0x30);
        param_2.add(0xb).write_unaligned(0);
        let v = param_2.add(0xb).read_unaligned();
        param_2.add(10).write_unaligned(v);
        let v = param_2.add(10).read_unaligned();
        param_2.add(9).write_unaligned(v);
    }
}
