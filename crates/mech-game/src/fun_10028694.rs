// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10028bde(
        param_1: i32,
        param_2: u16,
        param_3: i16,
        param_4: *mut u16,
        param_5: *mut u16,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10028694(param_1: i32, param_2: u16) -> u16 {
    let mut param_2 = param_2;
    let mut local_c: [u16; 2] = [0; 2];
    let mut local_8: [u16; 2] = [0; 2];

    if ((param_2 as i16 as i32 as u32) & 0xf000) != 0 {
        // SAFETY: the callee is the game's own; the out pointers refer to live
        // local arrays, and param_1 points at the game's object, read at the
        // offsets the original reads.
        unsafe {
            FUN_10028bde(
                param_1,
                param_2,
                -3,
                local_c.as_mut_ptr(),
                local_8.as_mut_ptr(),
            );
            param_2 = local_8[0];
            if ((param_1 as u32).wrapping_add(0x176) as *const i16).read_unaligned() == 2
                && ((param_1 as u32).wrapping_add(0x1a8) as *const i32).read_unaligned() != 0
            {
                param_2 = local_c[0];
            }
        }
    }
    param_2
}
