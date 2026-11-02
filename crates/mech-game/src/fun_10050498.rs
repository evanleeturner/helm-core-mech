// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0864: i32;
    static mut DAT_101155c0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100509fd(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10050498(param_1: i32, param_2: i32) {
    mech_trace::enter!("10050498", param_1, param_2);
    // SAFETY: the globals and callee are the game's own; the table at
    // DAT_101155c0 holds DAT_100b0864 entries of 0x48 bytes each.
    unsafe {
        let mut local_8: i32 = 0;
        while local_8 < mech_trace::gr!("10050498", "100b0864", DAT_100b0864) {
            let base = (&raw mut DAT_101155c0).cast::<u8>();
            let v = mech_trace::rd!(
                "10050498",
                base.byte_add(local_8.wrapping_mul(0x48) as usize)
                    .cast::<i32>(),
                read_unaligned
            );
            if v == param_1 {
                mech_trace::cr!("10050498", "100509fd", FUN_100509fd(local_8, param_2));
            }
            local_8 = local_8.wrapping_add(1);
        }
    }
}
