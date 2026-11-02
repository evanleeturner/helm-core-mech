// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005d9bf(param_1: *mut i32, param_2: *mut u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005da75(param_1: *mut i32) {
    mech_trace::enter!("1005da75", param_1);
    // SAFETY: param_1 points at the game's list head; each node is a
    // record whose field at index 5 links to the next node.
    unsafe {
        let mut local_8 = mech_trace::rd!("1005da75", param_1, read_unaligned) as *mut u32;
        while !local_8.is_null() {
            mech_trace::cr!("1005da75", "1005d9bf", FUN_1005d9bf(param_1, local_8));
            local_8 = mech_trace::rd!("1005da75", local_8.add(5), read_unaligned) as *mut u32;
        }
    }
}
