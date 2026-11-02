// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003dd75(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f8af(param_1: i32) {
    mech_trace::enter!("1002f8af", param_1);
    // SAFETY: param_1 is the address of a game node; its fields are read at
    // the original's offsets and the callee is the game's own.
    unsafe {
        let v = mech_trace::rd!(
            "1002f8af",
            ((param_1 as usize).wrapping_add(0x6c) as *const i32),
            read_unaligned
        );
        if v != 0 {
            mech_trace::cr!("1002f8af", "1003dd75", FUN_1003dd75(v));
        }
        let mut local_8 = mech_trace::rd!(
            "1002f8af",
            ((param_1 as usize).wrapping_add(4) as *const i32),
            read_unaligned
        );
        while local_8 != 0 {
            FUN_1002f8af(local_8);
            local_8 = mech_trace::rd!(
                "1002f8af",
                ((local_8 as usize).wrapping_add(8) as *const i32),
                read_unaligned
            );
        }
    }
}
