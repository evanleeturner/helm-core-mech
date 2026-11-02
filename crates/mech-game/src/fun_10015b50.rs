// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100660e1(param_1: i32, param_2: *mut i32, param_3: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10015b50(param_1: i32) {
    mech_trace::enter!("10015b50", param_1);
    // SAFETY: param_1 is the address of the caller's object; fields are
    // written at the original's offsets and the callee is the game's own.
    unsafe {
        mech_trace::wr!(
            "10015b50",
            ((param_1 as usize).wrapping_add(0x74) as *mut u32),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10015b50",
            ((param_1 as usize).wrapping_add(0x78) as *mut u32),
            write_unaligned,
            0x8000
        );
        mech_trace::wr!(
            "10015b50",
            ((param_1 as usize).wrapping_add(0x7c) as *mut u32),
            write_unaligned,
            0x8000
        );
        mech_trace::wr!(
            "10015b50",
            ((param_1 as usize).wrapping_add(0x80) as *mut u32),
            write_unaligned,
            0x2db7
        );
        let size = mech_trace::rd!(
            "10015b50",
            ((param_1 as usize).wrapping_add(0xa4) as *const i32),
            read_unaligned
        );
        FUN_100660e1(
            size,
            (param_1 as usize).wrapping_add(0x7c) as *mut i32,
            (param_1 as usize).wrapping_add(0x7c) as *mut i32,
        );
        let size = mech_trace::rd!(
            "10015b50",
            ((param_1 as usize).wrapping_add(0xa4) as *const i32),
            read_unaligned
        );
        FUN_100660e1(
            size,
            (param_1 as usize).wrapping_add(0x74) as *mut i32,
            (param_1 as usize).wrapping_add(0x74) as *mut i32,
        );
    }
}
