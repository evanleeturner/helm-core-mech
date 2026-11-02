// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001c299(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c4b2(param_1: i32, param_2: i32) {
    // SAFETY: param_1 points at the game's structure and the list nodes it
    // links are the game's own; reads happen in the original's order.
    unsafe {
        let mut local_8: *const i32 =
            ((param_1.wrapping_add(0x1c)) as *const i32).read_unaligned() as *const i32;
        loop {
            if local_8.is_null() {
                return;
            }
            if local_8.read_unaligned() <= param_2 {
                break;
            }
            local_8 = local_8.byte_add(12).cast::<i32>().read_unaligned() as *const i32;
        }
        FUN_1001c299(param_1, local_8 as i32);
    }
}
