// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003510c(param_1: i32, param_2: i32, param_3: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100351e7(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    mech_trace::enter!("100351e7", param_1, param_2, param_3, param_4);
    // SAFETY: param_4 points at the caller's structure, read at offsets 4 and 8;
    // the callee is the game's own.
    unsafe {
        let a = mech_trace::rd!(
            "100351e7",
            (param_4.wrapping_add(4) as *const i32),
            read_unaligned
        );
        let b = mech_trace::rd!(
            "100351e7",
            (param_4.wrapping_add(8) as *const i32),
            read_unaligned
        );
        mech_trace::cr!(
            "100351e7",
            "1003510c",
            FUN_1003510c(a.wrapping_add(param_1), b.wrapping_add(param_2), param_3)
        );
    }
}
