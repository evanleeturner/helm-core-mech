// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FUN_1006a109(param_1: i32, param_2: i32) -> i32 {
    // SAFETY: param_1 is an address of the game's own table; the entries
    // are read at the same offsets as the original.
    unsafe {
        let off = (param_1
            .wrapping_add(8)
            .wrapping_add(param_2.wrapping_mul(8)) as usize) as *const i32;
        let inner = off.read_unaligned();
        let addr = inner.wrapping_add(param_1).wrapping_add(4);
        (addr as usize as *const i32).read_unaligned()
    }
}
