// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f90b(param_1: i32, param_2: u32) {
    // SAFETY: param_1 and the linked nodes are the game's own structures,
    // read and written at the offsets the original uses.
    unsafe {
        let p = (param_1 as usize).wrapping_add(0x6c) as *const i32;
        let v = p.read_unaligned();
        if v != 0 {
            ((v as usize).wrapping_add(2) as *mut i16).write_unaligned(param_2 as i16);
        }
        let mut local_8 = ((param_1 as usize).wrapping_add(4) as *const i32).read_unaligned();
        while local_8 != 0 {
            FUN_1002f90b(local_8, param_2);
            local_8 = ((local_8 as usize).wrapping_add(8) as *const i32).read_unaligned();
        }
    }
}
