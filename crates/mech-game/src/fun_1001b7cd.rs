// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001b72b(param_1: *mut u16, param_2: *mut i32) -> u8;
    fn FUN_1001b550(param_1: i32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001b7cd(param_1: *mut u16, param_2: *mut i32) {
    // SAFETY: the callees are the game's own; param_1 points at the caller's
    // object, and the list nodes are walked and written where the original does.
    unsafe {
        FUN_1001b72b(param_1, param_2);
        let mut local_8: i32 = param_1.byte_add(0x1c).cast::<i32>().read_unaligned();
        while local_8 != 0 {
            FUN_1001b550(local_8, param_2);
            let v = param_1.byte_add(0x50).cast::<i32>().read_unaligned();
            (local_8 as u32).wrapping_add(0x10) as *mut i32;
            ((local_8 as u32).wrapping_add(0x10) as *mut i32).write_unaligned(v);
            local_8 = ((local_8 as u32).wrapping_add(0xc) as *const i32).read_unaligned();
        }
    }
}
