// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c569(param_1: i32, param_2: u16) {
    // SAFETY: param_1 is the caller's object address; the pointer at offset
    // 0x20, when non-null, points at a record whose offset 0x14 is written.
    unsafe {
        let p = ((param_1 as usize).wrapping_add(0x20) as *const i32).read_unaligned();
        if p != 0 {
            ((p as usize).wrapping_add(0x14) as *mut u16).write_unaligned(param_2);
        }
    }
}
