// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10089643(param_1: *mut u32, param_2: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003ba44(param_1: i32, param_2: *mut u32) {
    mech_trace::enter!("1003ba44", param_1, param_2);
    // SAFETY: the callee is the game's own; param_1 is an address of a game
    // object and param_2 points at the caller's buffer, read and written
    // where the original does.
    unsafe {
        FUN_10089643(param_1.wrapping_add(0x54) as *mut u32, param_2);
        mech_trace::wr!(
            "1003ba44",
            param_2.add(9),
            write_unaligned,
            mech_trace::rd!(
                "1003ba44",
                (param_1.wrapping_add(0x78) as *const u32),
                read_unaligned
            )
        );
        mech_trace::wr!(
            "1003ba44",
            param_2.add(10),
            write_unaligned,
            mech_trace::rd!(
                "1003ba44",
                (param_1.wrapping_add(0x7c) as *const u32),
                read_unaligned
            )
        );
        mech_trace::wr!(
            "1003ba44",
            param_2.add(0xb),
            write_unaligned,
            mech_trace::rd!(
                "1003ba44",
                (param_1.wrapping_add(0x80) as *const u32),
                read_unaligned
            )
        );
    }
}
