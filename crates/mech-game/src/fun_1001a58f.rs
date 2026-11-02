// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10014480(param_1: i32, param_2: u32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001a58f(param_1: *mut i32) -> i32 {
    mech_trace::enter!("1001a58f", param_1);
    // SAFETY: param_1 points at the caller's three-int array; the callee is the game's own.
    unsafe {
        let a = mech_trace::rd!("1001a58f", param_1, read_unaligned);
        let b = mech_trace::rd!("1001a58f", param_1.add(1), read_unaligned);
        let c = mech_trace::rd!("1001a58f", param_1.add(2), read_unaligned);
        let mut local_8 = mech_trace::cr!("1001a58f", "10014480", FUN_10014480(a, b as u32, c));
        if local_8 < 1 {
            local_8 = local_8.wrapping_add(200);
        } else {
            local_8 = local_8.wrapping_add(500);
        }
        mech_trace::rt!("1001a58f", local_8)
    }
}
