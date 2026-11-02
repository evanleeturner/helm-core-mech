// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002fa9c(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002fb55(param_1: *mut i32) {
    // SAFETY: param_1 points at the game's node structure; the child list is
    // walked through the offsets the original uses, and the callees are the
    // game's own.
    unsafe {
        if (param_1.add(0x1d).cast::<u8>().read() & 1) == 0 {
            if param_1.add(1).read_unaligned() != 0 {
                let mut local_8 = param_1.add(1).read_unaligned() as *mut i32;
                while !local_8.is_null() {
                    FUN_1002fb55(local_8);
                    local_8 = local_8.add(2).read_unaligned() as *mut i32;
                }
            }
        } else {
            FUN_1002fa9c(param_1);
        }
    }
}
