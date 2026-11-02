// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_1003d99c(param_1: i32, param_2: i32, param_3: i32) -> u8;
    fn FUN_1003d816(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003d8af(param_1: *mut u16) {
    mech_trace::enter!("1003d8af", param_1);
    let mut local_18: u32 = 0;
    let mut local_c: u32 = 0;
    let mut local_8: u32 = 0;
    // SAFETY: param_1 is the caller's object, read at the original's offsets;
    // the callees and the global are the game's own.
    unsafe {
        if !param_1.is_null()
            && mech_trace::gr!("1003d8af", "100aefc0", DAT_100aefc0) != 0
            && (mech_trace::rd!("1003d8af", param_1, read_unaligned) & 0x1000) == 0
        {
            let handle = mech_trace::rd!(
                "1003d8af",
                param_1.byte_add(0x18).cast::<i32>(),
                read_unaligned
            );
            mech_trace::cr!(
                "1003d8af",
                "1002f322",
                FUN_1002f322(
                    handle,
                    &raw mut local_c,
                    &raw mut local_18,
                    &raw mut local_8,
                )
            );
            let local_10 = (local_c as i32).wrapping_add(150000) / 25000;
            let local_14 = (local_8 as i32).wrapping_add(150000) / 25000;
            let a = mech_trace::rd!(
                "1003d8af",
                param_1.byte_add(0x44).cast::<i32>(),
                read_unaligned
            );
            let b = mech_trace::rd!(
                "1003d8af",
                param_1.byte_add(0x48).cast::<i32>(),
                read_unaligned
            );
            if a != local_10 || b != local_14 {
                if local_10 < 0 || local_10 > 0xb || local_14 < 0 || local_14 > 0xb {
                    let h = mech_trace::rd!(
                        "1003d8af",
                        param_1.byte_add(0x18).cast::<i32>(),
                        read_unaligned
                    );
                    mech_trace::cr!("1003d8af", "1003d816", FUN_1003d816(h));
                } else {
                    let h = mech_trace::rd!(
                        "1003d8af",
                        param_1.byte_add(0x18).cast::<i32>(),
                        read_unaligned
                    );
                    mech_trace::cr!("1003d8af", "1003d99c", FUN_1003d99c(h, local_10, local_14));
                }
            }
        }
    }
}
