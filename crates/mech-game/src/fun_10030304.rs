// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007c203(param_1: *mut i32, param_2: *mut u8, param_3: i32) -> u8;
    fn FUN_1007c066(param_1: *mut i32, param_2: *mut u8, param_3: i32) -> u8;
    fn FUN_10050ecc(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10030304(param_1: *mut i32, param_2: u32) {
    mech_trace::enter!("10030304", param_1, param_2);
    if !param_1.is_null() {
        // SAFETY: param_1 points at a live node; its fields are read where the
        // original reads them, and the callees are the game's own.
        unsafe {
            FUN_10030304(
                mech_trace::rd!("10030304", param_1.add(2), read_unaligned) as *mut i32,
                param_2,
            );
            let v = mech_trace::rd!("10030304", param_1.add(0x1b), read_unaligned);
            if v == 0
                || u32::from(mech_trace::rd!(
                    "10030304",
                    ((v as u32).wrapping_add(0x16) as *const u16),
                    read_unaligned
                )) != param_2
            {
                FUN_10030304(
                    mech_trace::rd!("10030304", param_1.add(1), read_unaligned) as *mut i32,
                    param_2,
                );
            } else {
                mech_trace::cr!(
                    "10030304",
                    "1007c203",
                    FUN_1007c203(
                        mech_trace::rd!("10030304", param_1.add(1), read_unaligned) as *mut i32,
                        FUN_10050ecc as usize as *mut u8,
                        param_2 as i32,
                    )
                );
                mech_trace::cr!(
                    "10030304",
                    "1007c066",
                    FUN_1007c066(param_1, FUN_10050ecc as usize as *mut u8, param_2 as i32)
                );
            }
        }
    }
}
