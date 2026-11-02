// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003ddbc(param_1: *mut u16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f853(param_1: i32) {
    // SAFETY: param_1 is a node address in the game's own memory; fields are
    // read at the offsets the original uses, and the callee is the game's.
    unsafe {
        let p = ((param_1 as u32).wrapping_add(0x6c) as *const u32).read_unaligned();
        if p != 0 {
            FUN_1003ddbc(p as *mut u16);
        }
        let mut local_8 = ((param_1 as u32).wrapping_add(4) as *const i32).read_unaligned();
        while local_8 != 0 {
            FUN_1002f853(local_8);
            local_8 = ((local_8 as u32).wrapping_add(8) as *const i32).read_unaligned();
        }
    }
}
