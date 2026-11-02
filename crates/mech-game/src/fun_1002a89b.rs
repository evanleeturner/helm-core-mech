// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10083b74(param_1: i32, param_2: i32, param_3: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002a89b(param_1: i32, param_2: i16) {
    mech_trace::enter!("1002a89b", param_1, param_2);
    // SAFETY: param_1 is the address of the game's own object; fields are read
    // and written at the original's offsets, and the callee is the game's own.
    unsafe {
        mech_trace::wr!(
            "1002a89b",
            ((param_1.wrapping_add(0xdc)) as *mut i32),
            write_unaligned,
            param_2 as i32
        );
        mech_trace::cr!("1002a89b", "10083b74", FUN_10083b74(param_1, 1, 1));
        let a = mech_trace::rd!(
            "1002a89b",
            ((param_1.wrapping_add(0x1a2)) as *const i32),
            read_unaligned
        );
        let b = mech_trace::rd!(
            "1002a89b",
            ((param_1.wrapping_add(0xdc)) as *const i32),
            read_unaligned
        );
        if a == b {
            mech_trace::cr!("1002a89b", "10083b74", FUN_10083b74(param_1, 1, 1));
        }
        let v = mech_trace::rd!(
            "1002a89b",
            ((param_1.wrapping_add(0xdc)) as *const u32),
            read_unaligned
        );
        mech_trace::wr!(
            "1002a89b",
            ((param_1.wrapping_add(0x178)) as *mut i16),
            write_unaligned,
            v as i16
        );
    }
}
