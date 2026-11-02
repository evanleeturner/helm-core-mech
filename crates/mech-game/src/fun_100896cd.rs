// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_10089643(param_1: *mut u32, param_2: *mut i32);
    fn FUN_10089060(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100896cd(param_1: *mut u32, param_2: *mut i32) {
    mech_trace::enter!("100896cd", param_1, param_2);
    // SAFETY: param_1 and param_2 point at the caller's structures; the
    // callees are the game's own, and the locals outlive the calls.
    unsafe {
        let mut local_8: i32 =
            0u32.wrapping_sub(mech_trace::rd!("100896cd", param_1.add(9), read_unaligned)) as i32;
        let mut local_c: i32 =
            0u32.wrapping_sub(mech_trace::rd!("100896cd", param_1.add(10), read_unaligned)) as i32;
        let mut local_10: i32 = 0u32.wrapping_sub(mech_trace::rd!(
            "100896cd",
            param_1.add(0xb),
            read_unaligned
        )) as i32;
        FUN_10089643(param_1, param_2);
        mech_trace::wr!("100896cd", param_2.add(9), write_unaligned, 0);
        mech_trace::wr!("100896cd", param_2.add(10), write_unaligned, 0);
        mech_trace::wr!("100896cd", param_2.add(0xb), write_unaligned, 0);
        FUN_10089060(
            param_2,
            &raw mut local_8,
            &raw mut local_c,
            &raw mut local_10,
        );
        mech_trace::wr!("100896cd", param_2.add(9), write_unaligned, local_8);
        mech_trace::wr!("100896cd", param_2.add(10), write_unaligned, local_c);
        mech_trace::wr!("100896cd", param_2.add(0xb), write_unaligned, local_10);
    }
}
