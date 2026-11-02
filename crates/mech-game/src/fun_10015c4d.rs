// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100660e1(param_1: i32, param_2: *mut i32, param_3: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10015c4d(param_1: i32) {
    mech_trace::enter!("10015c4d", param_1);
    // SAFETY: param_1 is the address of the caller's object; fields are read
    // and written at the original's offsets, and the callee is the game's own.
    unsafe {
        let base = param_1 as usize as *mut u8;
        mech_trace::wr!(
            "10015c4d",
            base.byte_add(0x94).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10015c4d",
            base.byte_add(0x98).cast::<u32>(),
            write_unaligned,
            0x8000
        );
        mech_trace::wr!(
            "10015c4d",
            base.byte_add(0x9c).cast::<u32>(),
            write_unaligned,
            0x8000
        );
        mech_trace::wr!(
            "10015c4d",
            base.byte_add(0xa0).cast::<u32>(),
            write_unaligned,
            0x2db7
        );
        FUN_100660e1(
            mech_trace::rd!(
                "10015c4d",
                base.byte_add(0xa4).cast::<i32>(),
                read_unaligned
            ),
            base.byte_add(0x9c).cast::<i32>(),
            base.byte_add(0x9c).cast::<i32>(),
        );
        FUN_100660e1(
            mech_trace::rd!(
                "10015c4d",
                base.byte_add(0xa4).cast::<i32>(),
                read_unaligned
            ),
            base.byte_add(0x94).cast::<i32>(),
            base.byte_add(0x94).cast::<i32>(),
        );
    }
}
