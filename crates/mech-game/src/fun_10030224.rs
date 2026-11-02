// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100301b0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10030224(param_1: *mut i32) {
    mech_trace::enter!("10030224", param_1);
    if !param_1.is_null() {
        // SAFETY: param_1 is a non-null node owned by the game; its child
        // pointers at indices 1 and 2 are read and recursed on as the
        // original does, then the node is passed to the game's own routine.
        unsafe {
            let c2 = mech_trace::rd!("10030224", param_1.add(2), read_unaligned);
            if c2 != 0 {
                FUN_10030224(c2 as *mut i32);
            }
            let c1 = mech_trace::rd!("10030224", param_1.add(1), read_unaligned);
            if c1 != 0 {
                FUN_10030224(c1 as *mut i32);
            }
            FUN_100301b0(param_1);
        }
    }
}
