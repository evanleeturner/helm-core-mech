// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c85e(param_1: i32, param_2: i32, param_3: u8) {
    // SAFETY: the parameters are addresses of the game's own structures;
    // reads and writes happen at the same offsets and in the same order as the original.
    unsafe {
        if ((param_1.wrapping_add(0x20)) as *const i32).read_unaligned() != 0 {
            let idx = ((param_2.wrapping_add(2)) as *const u16).read_unaligned() as u32;
            let off = ((param_2.wrapping_add(4)) as *const i32).read_unaligned();
            let addr = (idx as i32).wrapping_add(off).wrapping_add(param_2);
            (addr as *mut u8).write(param_3);
            let cur = ((param_2.wrapping_add(2)) as *const i16).read_unaligned() as i32;
            ((param_2.wrapping_add(2)) as *mut i16).write_unaligned(cur.wrapping_add(1) as i16);
        }
    }
}
