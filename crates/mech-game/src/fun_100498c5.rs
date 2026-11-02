// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100496bb(param_1: i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100498c5(param_1: i32) -> i32 {
    let mut local_c: i32 = 0;
    // SAFETY: the callee is the game's own; a non-null result points at
    // an int array whose second element is read.
    unsafe {
        let pi_var1 = FUN_100496bb(param_1);
        if !pi_var1.is_null() {
            local_c = pi_var1.add(1).read_unaligned();
        }
    }
    local_c
}
