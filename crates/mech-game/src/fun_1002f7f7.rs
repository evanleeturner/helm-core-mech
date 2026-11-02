// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003df3a(param_1: *mut u16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f7f7(param_1: i32) {
    mech_trace::enter!("1002f7f7", param_1);
    // SAFETY: param_1 is the address of a game node; its fields are read at
    // the original's offsets and the callees are the game's own.
    unsafe {
        let p = ((param_1 as usize).wrapping_add(0x6c)) as *const i32;
        let v = mech_trace::rd!("1002f7f7", p, read_unaligned);
        if v != 0 {
            mech_trace::cr!("1002f7f7", "1003df3a", FUN_1003df3a(v as *mut u16));
        }
        let mut local_8 = mech_trace::rd!(
            "1002f7f7",
            (((param_1 as usize).wrapping_add(4)) as *const i32),
            read_unaligned
        );
        while local_8 != 0 {
            FUN_1002f7f7(local_8);
            local_8 = mech_trace::rd!(
                "1002f7f7",
                (((local_8 as usize).wrapping_add(8)) as *const i32),
                read_unaligned
            );
        }
    }
}
