// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10030224(param_1: *mut i32);
    fn FUN_100301b0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10030282(param_1: *mut i32, param_2: u32) {
    if !param_1.is_null() {
        // SAFETY: param_1 points at the game's node structure; its child
        // links and field at index 0x1b are read as the original does, and
        // the callees are the game's own.
        unsafe {
            FUN_10030282(param_1.add(2).read_unaligned() as *mut i32, param_2);
            let p = param_1.add(0x1b).read_unaligned();
            if p == 0
                || (((p as u32).wrapping_add(0x16) as *const u16).read_unaligned() as u32)
                    != param_2
            {
                FUN_10030282(param_1.add(1).read_unaligned() as *mut i32, param_2);
            } else {
                FUN_10030224(param_1.add(1).read_unaligned() as *mut i32);
                FUN_100301b0(param_1);
            }
        }
    }
}
