// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100496bb(param_1: i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004890b(param_1: i32) {
    mech_trace::enter!("1004890b", param_1);
    // SAFETY: the callee is the game's own; a non-null result points at
    // an int array whose element 2 is written.
    unsafe {
        let pi_var1 = mech_trace::cr!("1004890b", "100496bb", FUN_100496bb(param_1));
        if !pi_var1.is_null() {
            mech_trace::wr!("1004890b", pi_var1.add(2), write_unaligned, 0);
        }
    }
}
