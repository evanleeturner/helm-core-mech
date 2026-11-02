// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_100141c0(param_1: *mut u16, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f9c7(param_1: i32, param_2: i32) {
    mech_trace::enter!("1002f9c7", param_1, param_2);
    // SAFETY: param_1 is the address of a game node; its fields are read at
    // the original's offsets and the callees are the game's own.
    unsafe {
        let p = (param_1 as usize).wrapping_add(0x6c) as *const i32;
        let v = mech_trace::rd!("1002f9c7", p, read_unaligned);
        if v != 0 {
            FUN_100141c0(v as *mut u16, param_2);
        }
        let mut local_8 = mech_trace::rd!(
            "1002f9c7",
            ((param_1 as usize).wrapping_add(4) as *const i32),
            read_unaligned
        );
        while local_8 != 0 {
            FUN_1002f9c7(local_8, param_2);
            local_8 = mech_trace::rd!(
                "1002f9c7",
                ((local_8 as usize).wrapping_add(8) as *const i32),
                read_unaligned
            );
        }
    }
}
