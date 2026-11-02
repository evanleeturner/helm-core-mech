// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10054fa4(param_1: *mut u32, param_2: i32, param_3: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10054f6b(param_1: *mut i16, param_2: *mut u32) {
    mech_trace::enter!("10054f6b", param_1, param_2);
    // SAFETY: param_1 points at the caller's short data; the callee is the
    // game's own and receives the same arguments as the original.
    unsafe {
        FUN_10054fa4(
            param_1.add(6).cast::<u32>(),
            mech_trace::rd!("10054f6b", param_1, read_unaligned) as i32,
            param_2,
        );
    }
}
